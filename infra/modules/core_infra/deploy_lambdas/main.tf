provider "aws" {
  region = var.aws_region
}

resource "aws_security_group" "lambda_provision" {
  name        = "lambda_provision_sg"
  description = "Security group for provisioning Lambda images"
  vpc_id      = var.vpc_id

  # SSH access
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound HTTPS access
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound DNS access
  egress {
    from_port   = 53
    to_port     = 53
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "random_string" "lambda_deploy_bucket_hash" {
  length  = 6
  special = false
  upper   = false
}
resource "aws_s3_bucket" "lambda_deploy" {
  bucket = "lambda-deploy-${random_string.lambda_deploy_bucket_hash.result}"
  acl    = "private"

  versioning {
    enabled = true
  }
}

resource "aws_iam_access_key" "lambda_deploy_upload" {
  user = aws_iam_user.lambda_deploy_upload.name
}

resource "aws_iam_user" "lambda_deploy_upload" {
  name = "lambda_deploy_upload"
}

resource "aws_iam_user_policy" "lambda_deploy_upload" {
  name = "lambda_deploy_upload"
  user = aws_iam_user.lambda_deploy_upload.name

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:PutObject*"
      ],
      "Effect": "Allow",
      "Resource": ["${aws_s3_bucket.lambda_deploy.arn}/*"]
    }
  ]
}
EOF
}

data "aws_ami" "lambda_provision" {
  most_recent = true
  owners      = ["137112412989"]

  filter {
    name   = "image-id"
    values = ["ami-0080e4c5bc078760e"]
  }
}

resource "aws_instance" "app" {
  count = var.provisioning ? 1 : 0

  instance_type          = "t3a.micro"
  ami                    = data.aws_ami.lambda_provision.id
  vpc_security_group_ids = [aws_security_group.lambda_provision.id]
  subnet_id              = var.app_subnet_ids[0]

  // TODO: SSH keypair
  // TODO: run lambda provisioner (scp lambda folders, run pip install, zip, curl to S3)

  tags = {
    Name        = "app"
    Environment = "deploy"
  }
}
