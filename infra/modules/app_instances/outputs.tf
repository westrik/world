output "app_autoscaling_group_ids" {
  value = [aws_autoscaling_group.app_blue.id]
}

output "app_host_iam_role_id" {
  value = aws_iam_role.app_host.id
}
