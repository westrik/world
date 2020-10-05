output "app_target_group_arn" {
  value = aws_lb_target_group.app_blue.arn
}

output "app_target_group_name" {
  value = aws_lb_target_group.app_blue.name
}

output "app_lb_listener_arn" {
  value = aws_lb_listener.app.arn
}
