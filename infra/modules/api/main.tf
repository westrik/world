// - Set up a VPC, security group, and subnet for our application servers to live in
// - Provision an EC2 instance running our custom AMI
// - Set up an NLB to point to the application server subnet
// - Request a TLS certificate from ACM for the LB
// - Point Route 53 DNS at the LB

provider "aws" {
  region = var.aws_region
}

resource "aws_vpc" "app" {
  cidr_block = "10.0.0.0/16"
  // TODO: enable if/when NLBs support IPv6
  //  assign_generated_ipv6_cidr_block = true

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
  map_public_ip_on_launch = true

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

resource "aws_instance" "app" {
  # TODO: [harden] change default login and SSH config for AMI (no password)
  # TODO?: configure with a stored keypair to allow login via bastion

  count = var.num_app_instances

  instance_type          = "t3a.micro"
  ami                    = data.aws_ami.app.id
  vpc_security_group_ids = [aws_security_group.app_inbound.id, aws_security_group.app_outbound.id, aws_security_group.app_outbound_s3.id]
  subnet_id              = aws_subnet.app_az1.id
  iam_instance_profile   = aws_iam_instance_profile.app_host.name

  # TODO: encrypt EBS with KMS key? or figure out how to avoid saving things to disk

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

resource "aws_lb_target_group" "app_insecure" {
  name     = "app-lb-insecure-target-group"
  port     = 80
  protocol = "TCP"
  vpc_id   = aws_vpc.app.id
}
resource "aws_lb_target_group_attachment" "app_insecure" {
  count            = var.num_app_instances
  target_id        = aws_instance.app[count.index].id
  target_group_arn = aws_lb_target_group.app_insecure.arn
  port             = 80
}
resource "aws_lb_listener" "app_insecure" {
  load_balancer_arn = aws_lb.app.arn
  port              = 80
  protocol          = "TCP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.app_insecure.arn
  }
}


resource "aws_lb_target_group" "app" {
  name     = "app-lb-target-group"
  port     = 443
  protocol = "TCP_UDP"
  vpc_id   = aws_vpc.app.id
}
resource "aws_lb_target_group_attachment" "app" {
  count            = var.num_app_instances
  target_id        = aws_instance.app[count.index].id
  target_group_arn = aws_lb_target_group.app.arn
  port             = 443
}
resource "aws_lb_listener" "app" {
  load_balancer_arn = aws_lb.app.arn
  port              = 443
  protocol          = "TCP_UDP"

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

resource "aws_route53_record" "app_caa" {
  zone_id = data.aws_route53_zone.app.id
  name    = var.api_domain_name
  type    = "CAA"
  records = ["0 issue \"amazon.com\""]
  ttl     = 60
}



resource "aws_secretsmanager_secret" "westrikworld_api_cert" {
  name                    = "westrikworld_api_cert"
  recovery_window_in_days = 0
}

/** Lambda for renewing certificate with Let's Encrypt */
resource "aws_lambda_function" "renew_certificate" {
  function_name = "renew_certificate"
  role          = aws_iam_role.lambda_renew_certificate.arn
  // TODO: zip needs to be built on ami-0080e4c5bc078760e
  // see https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html
  filename = "./lambda/renew_certificate.zip"
  handler  = "renew_certificate.lambda_handler"
  runtime  = "python3.7"

  vpc_config {
    security_group_ids = [aws_security_group.app_inbound.id, aws_security_group.app_outbound.id, aws_security_group.app_outbound_s3.id]
    subnet_ids         = [aws_subnet.app_az1.id, aws_subnet.app_az2.id]

  }
}

data "aws_iam_policy_document" "lambda_renew_certificate" {
  statement {
    sid = "1"

    actions = [
      "sts:AssumeRole",
    ]

    principals {
      identifiers = ["lambda.amazonaws.com"]
      type        = "Service"
    }
  }
}

resource "aws_iam_role" "lambda_renew_certificate" {
  name               = "lambda_renew_certificate"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.lambda_renew_certificate.json
}

resource "aws_iam_role_policy_attachment" "role_attach_lambda_vpc" {
  role       = aws_iam_role.lambda_renew_certificate.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}

resource "aws_iam_role_policy_attachment" "role_attach_lambda_secrets_manager" {
  role       = aws_iam_role.lambda_renew_certificate.name
  policy_arn = "arn:aws:iam::aws:policy/SecretsManagerReadWrite"
}

//data "aws_lambda_invocation" "renew_certificate" {
//  function_name = aws_lambda_function.renew_certificate.function_name
//  depends_on    = [aws_lambda_function.renew_certificate]
//
//  input = <<JSON
//{
//  "domains": ["${var.api_domain_name}"],
//  "email": "${var.admin_email}",
//  "secret_id": "${aws_secretsmanager_secret.westrikworld_api_cert.name}"
//}
//JSON
//}
//
