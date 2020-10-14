resource "aws_key_pair" "test_key" {
  public_key = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC8u441SFCy5higGr/0mWsSfGsiJyzpouDvcVW6WO8tNqC24DCnVF8LOfGZvFH2bWNCrFFeMwj3PCd3B6CeLuGP3iLE5WLZqutb6+ca8/hrYlwSF1hzt451k5/4tXL5O1rRkmVbosmjjuJzm/vib9nDHeF8ebXabSBjvE+V8nhj26UpOoheSYTc3XDzkbDJuOj1wSSrirfMsZVVse9GgSzOMdZVVjrheZAUPxMFKbEZEL0ZIkr4DIDld78UyU7ZPsLJoZjRK+MzEFwjyZ/TNjIsvn6rgaCM+MFFeHXc2z1yG60Tv8trtPLu7KHpTcSrVVo2DUEUlbR32uQ86MvFCS4B4OfWW+cDTbYBw+5wjUkhwg6AvmvcU7Ix4N4vosSq+ny/Sj/LbxmmE4QL1r8ZUUQ+3AqtA2O0MCuzdQtt1pQDCur9v+PD5lF411KT4BsG/me+GW4xiAbJSXpzhfTgu/gsjzbIbet8onzC7+naofgRdbB0kLJEco3/2hIgHLXdVCM="
}

resource "aws_launch_template" "app_cluster_instance" {
  name_prefix            = "${var.project_name}-app-${var.deploy_name}-${var.cluster_name}-"
  image_id               = var.ami_id
  instance_type          = "t3a.micro"
  vpc_security_group_ids = var.app_security_group_ids

  key_name = aws_key_pair.test_key.key_name

  # TODO: make sure EBS volume is encrypted

  iam_instance_profile {
    name = var.iam_instance_profile_name
  }

  lifecycle {
    create_before_destroy = false
  }
}

locals {
  app_instance_lifetime_seconds = 60 * 60 * 24 * 7 # one week in seconds
}

resource "aws_autoscaling_group" "app_cluster" {
  name             = "${aws_launch_template.app_cluster_instance.name}-asg"
  desired_capacity = var.num_app_instances
  max_size         = var.num_app_instances * 2
  min_size         = var.num_app_instances

  vpc_zone_identifier = var.app_subnet_ids
  target_group_arns   = [var.target_group_arn]

  termination_policies  = ["OldestLaunchTemplate", "OldestInstance"]
  max_instance_lifetime = local.app_instance_lifetime_seconds

  health_check_grace_period = 120
  // health_check_type = "ELB"
  // default_cooldown = 10

  // metrics_granularity = "1Minute"
  // enabled_metrics = []

  launch_template {
    id      = aws_launch_template.app_cluster_instance.id
    version = "$Latest"
  }

  lifecycle {
    create_before_destroy = false
  }

  //  initial_lifecycle_hook {
  //    name                 = "${var.project_name}-app-${var.deploy_name}-starting"
  //    default_result       = "CONTINUE"
  //    heartbeat_timeout    = 2000
  //    lifecycle_transition = "autoscaling:EC2_INSTANCE_LAUNCHING"
  //
  //    //    notification_metadata = <<EOF
  //    //{
  //    //  "foo": "bar"
  //    //}
  //    //EOF
  //
  //    notification_target_arn = aws_sns_topic.app_scaling.arn
  //    // role_arn                = "arn:aws:iam::123456789012:role/S3Access"
  //  }

  tags = [
    {
      key                 = "Name"
      value               = "app-${var.cluster_name}"
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

resource "aws_autoscaling_notification" "app_scaling" {
  group_names = [
    aws_autoscaling_group.app_cluster.name,
  ]

  notifications = [
    "autoscaling:EC2_INSTANCE_LAUNCH",
    "autoscaling:EC2_INSTANCE_TERMINATE",
    "autoscaling:EC2_INSTANCE_LAUNCH_ERROR",
    "autoscaling:EC2_INSTANCE_TERMINATE_ERROR",
  ]

  topic_arn = aws_sns_topic.app_scaling.arn
}

//resource "aws_autoscaling_lifecycle_hook" "app_scale_up" {
//  name                   = "${var.project_name}-app-${var.deploy_name}-scale_up"
//  autoscaling_group_name = aws_autoscaling_group.app_bluegreen.name
//  default_result         = "CONTINUE"
//  heartbeat_timeout      = 2000
//  lifecycle_transition   = "autoscaling:EC2_INSTANCE_LAUNCHING"
//
////  notification_metadata = <<EOF
////{
////  "foo": "bar"
////}
////EOF
//
//  notification_target_arn = aws_sns_topic.app_scaling.arn
//  role_arn                = "arn:aws:iam::123456789012:role/S3Access"
//}

resource "aws_sns_topic" "app_scaling" {
  name = "${var.project_name}-app-${var.deploy_name}-scaling"
  //  kms_master_key_id = "alias/aws/sns"
  delivery_policy = <<EOF
{
  "http": {
    "defaultHealthyRetryPolicy": {
      "minDelayTarget": 20,
      "maxDelayTarget": 20,
      "numRetries": 3,
      "numMaxDelayRetries": 0,
      "numNoDelayRetries": 0,
      "numMinDelayRetries": 0,
      "backoffFunction": "linear"
    },
    "disableSubscriptionOverrides": false
  }
}
EOF
}
