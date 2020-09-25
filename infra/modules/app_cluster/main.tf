// Provision a cluster of EC2 instances running our custom AMI

provider "aws" {
  region = var.aws_region
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
    values = [var.deploy_name]
  }

  filter {
    name   = "tag:OS_Version"
    values = ["Debian 10"]
  }
}

module "autoscaling_group_blue" {
  source = "./autoscaling_group"

  color        = "blue"
  deploy_name  = var.deploy_name
  project_name = var.project_name

  ami_id                    = "ami-06654a2796c88fe40"
  app_security_group_ids    = var.app_security_group_ids
  app_subnet_ids            = var.app_subnet_ids
  iam_instance_profile_name = aws_iam_instance_profile.app_host.name
  num_app_instances         = var.num_app_instances
  target_group_arn          = module.app_load_balancer.app_target_group_arn
}

module "app_load_balancer" {
  source = "./load_balancer"

  aws_region = var.aws_region
  aws_az1    = var.aws_az1
  aws_az2    = var.aws_az2

  project_name     = var.project_name
  project_slug     = var.project_slug
  deploy_name      = var.deploy_name
  root_domain_name = var.root_domain_name
  api_domain_names = var.api_domain_names
  admin_email      = var.admin_email

  app_vpc_id             = var.app_vpc_id
  app_security_group_ids = var.app_security_group_ids
  app_subnet_ids         = var.app_subnet_ids

  lambda_deploy_bucket                   = var.lambda_deploy_bucket
  lambda_iam_role_arn__renew_certificate = var.lambda_iam_role_arn__renew_certificate
}

// Grant EC2 access to RDS
resource "aws_iam_instance_profile" "app_host" {
  name = "app_host"
  role = aws_iam_role.app_host.name
}

resource "aws_iam_role" "app_host" {
  name               = "ec2_app_host"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.ec2_assume_role.json
}
data "aws_iam_policy_document" "ec2_assume_role" {
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

// TODO: don't allow RDSFullAccess to non-admin instances
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
resource "aws_iam_role_policy" "app_host_assume_role" {
  name = "app_host_assume_role"
  role = aws_iam_role.app_host.id

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "sts:AssumeRole"
      ],
      "Effect": "Allow",
      "Resource": ["${aws_iam_role.app_host.arn}"]
    }
  ]
}
EOF
}

resource "aws_secretsmanager_secret" "app_host_role_arn" {
  name                    = "${var.project_name}_ec2_app_host_role_arn"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "app_host_role_arn" {
  secret_id     = aws_secretsmanager_secret.app_host_role_arn.id
  secret_string = aws_iam_role.app_host.arn
}
