// - Provision an EC2 instance running our custom AMI

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

resource "aws_launch_template" "app_blue" {
  name_prefix            = "${var.project_name}-app-${var.deploy_name}-blue-"
  image_id               = data.aws_ami.app.id
  instance_type          = "t3a.micro"
  vpc_security_group_ids = var.app_security_group_ids

  # TODO: make sure EBS volume is encrypted

  iam_instance_profile {
    name = aws_iam_instance_profile.app_host.name
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_autoscaling_group" "app_blue" {
  name                = "${aws_launch_template.app_blue.name}-asg"
  desired_capacity    = var.num_app_instances
  max_size            = var.num_app_instances + 1
  min_size            = var.num_app_instances
  availability_zones  = [var.aws_az1, var.aws_az2]
  vpc_zone_identifier = var.app_subnet_ids
  target_group_arns   = [var.app_blue_target_group_arn]

  launch_template {
    id      = aws_launch_template.app_blue.id
    version = "$Latest"
  }

  lifecycle {
    create_before_destroy = true
  }

  tags = [
    {
      key                 = "Name"
      value               = "app"
      propagate_at_launch = true
    },
    {
      key                 = "Environment"
      value               = var.deploy_name
      propagate_at_launch = true
    },
    {
      key                 = "Project"
      value               = var.project_name
      propagate_at_launch = true
    }
  ]
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
