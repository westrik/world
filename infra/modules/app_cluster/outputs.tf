output "app_blue_autoscaling_group_id" {
  value = module.autoscaling_group_blue.app_autoscaling_group_id
}

output "app_host_iam_role_id" {
  value = aws_iam_role.app_host.id
}

output "app_lb_listener_arn" {
  value = module.app_load_balancer.app_lb_listener_arn
}

output "app_target_group_arn" {
  value = module.app_load_balancer.app_target_group_arn
}

output "app_target_group_name" {
  value = module.app_load_balancer.app_target_group_name
}

output "app_scaling_sns_arn" {
  value = module.autoscaling_group_blue.app_scaling_sns_arn
}
