output "app_autoscaling_group_ids" {
  //  value = [aws_autoscaling_group.app_blue.id, aws_autoscaling_group.app_green.id]
  value = [aws_autoscaling_group.app_blue.id]
}
