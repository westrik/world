// - Set up a VPC, security group, and subnet for our application servers to live in
// - Provision an EC2 instance running our custom AMI
// - Set up an NLB to point to the application server subnet
// - Request a TLS certificate from ACM for the LB
// - Point Route 53 DNS at the LB

/*
TODO(later):
  - [ ] handle IPv6
  - [ ] provision ACM private cert to use with NLB
*/

provider "aws" {
  region = var.aws_region
}

resource "aws_vpc" "app" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name        = "app_vpc"
    Environment = "production"
  }
}

resource "aws_vpc_endpoint" "s3" {
  vpc_id          = aws_vpc.app.id
  service_name    = "com.amazonaws.${var.aws_region}.s3"
  route_table_ids = [aws_vpc.app.main_route_table_id]

  tags = {
    Environment = "production"
  }
}

resource "aws_security_group" "app_inbound" {
  name        = "app_in_sg"
  description = "Primary ${var.project_name} production SG (inbound)"
  vpc_id      = aws_vpc.app.id

  # SSH access from me
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    //    cidr_blocks = ["10.0.0.0/16"]
  }

  # Inbound HTTP via NLB
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Inbound HTTPS via NLB
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

}
resource "aws_security_group" "app_outbound_s3" {
  name        = "app_out_s3_sg"
  description = "Primary ${var.project_name} production SG (outbound for s3)"
  vpc_id      = aws_vpc.app.id

  # Outbound HTTPS access to S3 (via VPC endpoint)
  egress {
    from_port = 443
    to_port   = 443
    protocol  = "tcp"
    prefix_list_ids = [
    aws_vpc_endpoint.s3.prefix_list_id]
  }

  # Outbound DNS access TODO: VPC endpoint?
  egress {
    from_port = 53
    to_port   = 53
    protocol  = "tcp"
    cidr_blocks = [
    "0.0.0.0/0"]
    # TODO: specify CIDR for DNS
  }
}


data "aws_ip_ranges" "amazon_services" {
  regions  = [var.aws_region]
  services = ["amazon"]
}

resource "aws_security_group" "app_outbound" {
  name        = "app_out_sg"
  description = "Primary ${var.project_name} production SG (outbound)"
  vpc_id      = aws_vpc.app.id

  # Outbound HTTPS to AWS (CodeDeploy)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] #data.aws_ip_ranges.amazon_services.cidr_blocks
  }

  # RDS
  egress {
    from_port = 5432
    to_port   = 5432
    protocol  = "tcp"
    cidr_blocks = [
    "10.0.0.0/16"]
  }
}

resource "aws_internet_gateway" "app" {
  vpc_id = aws_vpc.app.id
}

resource "aws_route" "app_internet_access" {
  route_table_id         = aws_vpc.app.main_route_table_id
  gateway_id             = aws_internet_gateway.app.id
  destination_cidr_block = "0.0.0.0/0"
}

resource "aws_subnet" "app_az1" {
  availability_zone       = var.aws_az1
  vpc_id                  = aws_vpc.app.id
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true #TODO: remove

  tags = {
    Name        = "app"
    Environment = "production"
  }
}

resource "aws_subnet" "app_az2" {
  availability_zone = var.aws_az2
  vpc_id            = aws_vpc.app.id
  cidr_block        = "10.0.2.0/24"

  tags = {
    Name        = "app"
    Environment = "production"
  }
}

data "aws_ami" "app" {
  most_recent = true
  owners      = ["self"]

  filter {
    name   = "name"
    values = ["${var.project_name} *"]
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

#TODO: remove
resource "tls_private_key" "westrikworld_staging_key" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

#TODO: remove
resource "aws_key_pair" "generated_key" {
  key_name   = "westrikworld-staging"
  public_key = tls_private_key.westrikworld_staging_key.public_key_openssh
}



resource "aws_instance" "app" {
  # TODO: [harden] change default login and SSH config for AMI (no password)
  # TODO?: configure with a stored keypair to allow login via bastion

  instance_type          = "t3a.micro"
  ami                    = data.aws_ami.app.id
  vpc_security_group_ids = [aws_security_group.app_inbound.id, aws_security_group.app_outbound.id, aws_security_group.app_outbound_s3.id]
  subnet_id              = aws_subnet.app_az1.id
  iam_instance_profile   = aws_iam_instance_profile.app_host.name
  #TODO: remove
  key_name = aws_key_pair.generated_key.key_name

  #TODO: remove
  connection {
    # The default username for our AMI
    user        = "admin"
    host        = self.public_ip
    private_key = tls_private_key.westrikworld_staging_key.private_key_pem
    # The connection will use the local SSH agent for authentication.
  }

  tags = {
    Name        = "app"
    Environment = "production"
  }
}

// Grant EC2 access to RDS
resource "aws_iam_instance_profile" "app_host" {
  name = "app_host"
  role = aws_iam_role.app_host.name
}
resource "aws_iam_role" "app_host" {
  name               = "ec2_app_host"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.app_rds.json
}
resource "aws_iam_role_policy_attachment" "app_rds" {
  role       = aws_iam_role.app_host.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonRDSFullAccess"
}
resource "aws_iam_role_policy_attachment" "app_code_deploy" {
  role       = aws_iam_role.app_host.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforAWSCodeDeploy"
}
resource "aws_iam_role_policy_attachment" "app_secrets" {
  role       = aws_iam_role.app_host.name
  policy_arn = "arn:aws:iam::aws:policy/SecretsManagerReadWrite"
}
data "aws_iam_policy_document" "app_rds" {
  statement {
    sid = "1"

    actions = [
      "sts:AssumeRole",
    ]

    principals {
      identifiers = ["ec2.amazonaws.com"]
      type        = "Service"
    }
  }
}

data "aws_route53_zone" "app" {
  name = "${var.root_domain_name}."
}

module "acm" {
  source  = "terraform-aws-modules/acm/aws"
  version = "~> 2.0"

  domain_name = var.api_domain_name
  zone_id     = data.aws_route53_zone.app.id
}

resource "aws_lb" "app" {
  name               = "app-nlb"
  load_balancer_type = "network"
  subnets            = [aws_subnet.app_az1.id]

  //  TODO: set up access log bucket
  //    access_logs = {
  //      bucket = module.log_bucket.this_s3_bucket_id
  //    }

  tags = {
    Environment = "production"
  }
}

resource "aws_lb_target_group" "app" {
  name     = "app-lb-target-group"
  port     = 80
  protocol = "TCP"
  vpc_id   = aws_vpc.app.id

  # work-around for https://github.com/terraform-providers/terraform-provider-aws/issues/9093
  stickiness {
    enabled = false
    type    = "lb_cookie"
  }
}

resource "aws_lb_target_group_attachment" "app" {
  target_group_arn = aws_lb_target_group.app.arn
  target_id        = aws_instance.app.id
  port             = 80
}

resource "aws_lb_listener" "app_https" {
  load_balancer_arn = aws_lb.app.arn
  port              = "443"
  protocol          = "TLS"
  ssl_policy        = "ELBSecurityPolicy-TLS-1-2-2017-01"
  certificate_arn   = module.acm.this_acm_certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.app.arn
  }
}

resource "aws_lb_listener" "app_http" {
  load_balancer_arn = aws_lb.app.arn
  port              = "80"
  protocol          = "TCP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.app.arn
  }
}

resource "aws_route53_record" "app" {
  zone_id = data.aws_route53_zone.app.id
  name    = var.api_domain_name
  type    = "A"

  alias {
    name                   = aws_lb.app.dns_name
    zone_id                = aws_lb.app.zone_id
    evaluate_target_health = false // TODO: enable?
  }
}
