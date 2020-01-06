// Tested with Terraform v0.12.18 (as of 2020-01-05)

/*
TODO:
  - [ ] split current tf code up into something more manageable
  - [ ] create S3 deploy zip bucket
  - [ ] upload a release zip to S3 manually
  - [ ] set up CodeDeploy app that pulls from S3
  - [ ] set up CodeDeploy agent correctly on instances (Q: how to authenticate?)
  - [ ] set up logging to S3 for NLB and applications
  - [ ] set up RDS
  - [ ] securely provide RDS creds to EC2 instances
  - [ ] handle migrations at app start-up with [`run_pending_migrations`](https://docs.rs/diesel_migrations/1.4.0/diesel_migrations/fn.run_pending_migrations.html)
  - [ ] set up S3 bucket for statically hosted files
  - [ ] set up CloudFront to point to S3
  - [ ] point [westrikworld.com](https://westrikworld.com) at CloudFront
  - [ ] verify that app works at [westrikworld.com](https://westrikworld.com)
  - [ ] serve static error when no backends are responding
  - [ ] double-check healthchecks are working (spin up 3 instances, kill 2 - should have no 500s)
  - [ ] set up CodePipeline to pull from S3 and trigger CodeDeploy automatically

later:
- [ ] handle IPv6
- [ ] provision ACM private cert to use with NLB
*/

provider "aws" {
  region = var.aws_region
}

/*
---------------------------------------------------------------------
Set up VPC, security group, subnet, EC2 and RDS instances, and an NLB
---------------------------------------------------------------------
*/

resource "aws_vpc" "ww_prod_app" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name        = "ww_prod_app_vpc"
    Environment = "production"
  }
}

# security group for our app instances
resource "aws_security_group" "ww_prod_app" {
  name        = "ww_prod_app_sg"
  description = "Primary westrikworld production VPC"
  vpc_id      = aws_vpc.ww_prod_app.id

  # HTTP access from the VPC
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  # HTTPS access from the VPC
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  # SSH access from the VPC
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  # Outbound HTTPS access (for CodeDeploy)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] # TODO: specify CIDR for CodeDeploy?
  }
}

resource "aws_internet_gateway" "ww_prod_app" {
  vpc_id = aws_vpc.ww_prod_app.id
}

resource "aws_route" "ww_prod_app_internet_access" {
  route_table_id         = aws_vpc.ww_prod_app.main_route_table_id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.ww_prod_app.id
}

resource "aws_subnet" "ww_prod_app" {
  availability_zone = var.aws_az
  vpc_id            = aws_vpc.ww_prod_app.id
  cidr_block        = "10.0.1.0/24"

  tags = {
    "Name" = "ww_prod_app"
  }
}

data "aws_ami" "westrikworld" {
  most_recent = true
  owners      = ["self"]
  count       = var.no_ami ? 0 : 1

  filter {
    name   = "name"
    values = ["westrikworld *"]
  }

  filter {
    name   = "tag:Environment"
    values = ["production"]
  }

  filter {
    name   = "tag:OS_Version"
    values = ["Debian 10"]
  }
}

resource "aws_instance" "ww_prod_app" {
  # TODO: change default login and SSH config for AMI (no password)
  # TODO: configure with a stored keypair to allow login via bastion

  instance_type          = "t3a.micro"
  ami                    = data.aws_ami.westrikworld[0].id
  vpc_security_group_ids = [aws_security_group.ww_prod_app.id]
  subnet_id              = aws_subnet.ww_prod_app.id
  count                  = (var.no_ami ? 0 : 1)

  tags = {
    Name        = "ww_prod_app_instance"
    Environment = "production"
  }
}

data "aws_route53_zone" "ww_prod_app" {
  name = "${var.root_domain_name}."
}

module "acm" {
  source  = "terraform-aws-modules/acm/aws"
  version = "~> 2.0"

  domain_name = var.api_domain_name
  zone_id     = data.aws_route53_zone.ww_prod_app.id
}

resource "aws_lb" "ww_prod_app" {
  name               = "ww-prod-app-nlb"
  load_balancer_type = "network"
  subnets            = [aws_subnet.ww_prod_app.id]

  //  TODO: set up access log bucket
  //    access_logs = {
  //      bucket = module.log_bucket.this_s3_bucket_id
  //    }

  tags = {
    Environment = "production"
  }
}

resource "aws_lb_target_group" "ww_prod_app" {
  name     = "ww-prod-app-lb-target-group"
  port     = 80
  protocol = "TCP"
  vpc_id   = aws_vpc.ww_prod_app.id

  # work-around for https://github.com/terraform-providers/terraform-provider-aws/issues/9093
  stickiness {
    enabled = false
    type    = "lb_cookie"
  }
}

resource "aws_lb_listener" "ww_prod_app_https" {
  load_balancer_arn = aws_lb.ww_prod_app.arn
  port              = "443"
  protocol          = "TLS"
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  certificate_arn   = module.acm.this_acm_certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.ww_prod_app.arn
  }
}

resource "aws_lb_listener" "ww_prod_app_http" {
  load_balancer_arn = aws_lb.ww_prod_app.arn
  port              = "80"
  protocol          = "TCP"

  // TODO: send 301 on http from nginx
  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.ww_prod_app.arn
  }
}

resource "aws_route53_record" "ww_prod_app" {
  zone_id = data.aws_route53_zone.ww_prod_app.id
  name    = var.api_domain_name
  type    = "A"

  alias {
    name                   = aws_lb.ww_prod_app.dns_name
    zone_id                = aws_lb.ww_prod_app.zone_id
    evaluate_target_health = false
  }
}

/*
---------------------------------------------------------------------
 Configure a VPC, security group, and subnet to build AMIs with
---------------------------------------------------------------------
*/

resource "aws_vpc" "packer_build" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name = "packer_build"
  }
}

resource "aws_security_group" "packer_build" {
  name        = "packer_build_sg"
  description = "security group for building AMIs with packer"
  vpc_id      = aws_vpc.packer_build.id

  # SSH access from anywhere
  # TODO: lock down when using CodeBuild for AMI builds
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound HTTPS access (for installing packages)
  # TODO?: specify CIDR block for internal package manager(s)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound DNS (TCP)
  # TODO?: specify CIDR block for internal DNS server
  egress {
    from_port   = 53
    to_port     = 53
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound DNS (UDP)
  # TODO?: specify CIDR block for internal DNS server
  egress {
    from_port   = 53
    to_port     = 53
    protocol    = "udp"
    cidr_blocks = ["0.0.0.0/0"] # TODO: specify IP for internal DNS
  }
}

resource "aws_internet_gateway" "packer_build" {
  vpc_id = aws_vpc.packer_build.id
}

resource "aws_route" "packer_build_internet_access" {
  route_table_id         = aws_vpc.packer_build.main_route_table_id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.packer_build.id
}

resource "aws_subnet" "packer_build" {
  availability_zone       = var.aws_az
  vpc_id                  = aws_vpc.packer_build.id
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true

  tags = {
    "Name"         = "packer_build"
    "Network Type" = "Public"
  }
}
