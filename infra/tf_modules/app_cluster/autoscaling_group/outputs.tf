output "app_autoscaling_group_id" {
  value = aws_autoscaling_group.app_cluster.id
}

output "app_scaling_sns_arn" {
  value = aws_sns_topic.app_scaling.arn
}
