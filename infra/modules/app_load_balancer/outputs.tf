output "app_blue_target_group_arn" {
  value = aws_lb_target_group.app_blue.arn
}

output "app_blue_target_group_name" {
  value = aws_lb_target_group.app_blue.name
}

output "app_green_target_group_arn" {
  value = aws_lb_target_group.app_green.arn
}

output "app_green_target_group_name" {
  value = aws_lb_target_group.app_green.name
}

output "app_lb_listener_arn" {
  value = aws_lb_listener.app.arn
}
