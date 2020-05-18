output "app_target_group_arns" {
  value = [aws_lb_target_group.app_insecure.arn, aws_lb_target_group.app_secure.arn]
}

