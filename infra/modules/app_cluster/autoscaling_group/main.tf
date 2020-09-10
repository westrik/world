
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

resource "aws_key_pair" "test_key" {
  public_key = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC8u441SFCy5higGr/0mWsSfGsiJyzpouDvcVW6WO8tNqC24DCnVF8LOfGZvFH2bWNCrFFeMwj3PCd3B6CeLuGP3iLE5WLZqutb6+ca8/hrYlwSF1hzt451k5/4tXL5O1rRkmVbosmjjuJzm/vib9nDHeF8ebXabSBjvE+V8nhj26UpOoheSYTc3XDzkbDJuOj1wSSrirfMsZVVse9GgSzOMdZVVjrheZAUPxMFKbEZEL0ZIkr4DIDld78UyU7ZPsLJoZjRK+MzEFwjyZ/TNjIsvn6rgaCM+MFFeHXc2z1yG60Tv8trtPLu7KHpTcSrVVo2DUEUlbR32uQ86MvFCS4B4OfWW+cDTbYBw+5wjUkhwg6AvmvcU7Ix4N4vosSq+ny/Sj/LbxmmE4QL1r8ZUUQ+3AqtA2O0MCuzdQtt1pQDCur9v+PD5lF411KT4BsG/me+GW4xiAbJSXpzhfTgu/gsjzbIbet8onzC7+naofgRdbB0kLJEco3/2hIgHLXdVCM="
}

resource "aws_launch_template" "app_bluegreen" {
  name_prefix            = "${var.project_name}-app-${var.deploy_name}-${var.color}-"
  image_id               = data.aws_ami.app.id
  instance_type          = "t3a.micro"
  vpc_security_group_ids = var.app_security_group_ids

  key_name = aws_key_pair.test_key.key_name

  # TODO: make sure EBS volume is encrypted

  iam_instance_profile {
    name = var.iam_instance_profile_name
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_autoscaling_group" "app_bluegreen" {
  name                = "${aws_launch_template.app_bluegreen.name}-asg"
  desired_capacity    = var.num_app_instances
  max_size            = var.num_app_instances + 1
  min_size            = var.num_app_instances
  vpc_zone_identifier = var.app_subnet_ids
  target_group_arns   = [var.target_group_arn]

  launch_template {
    id      = aws_launch_template.app_bluegreen.id
    version = "$Latest"
  }

  lifecycle {
    create_before_destroy = true
  }

  tags = [
    {
      key                 = "Name"
      value               = "app-${var.color}"
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
