output "app_deploy_hosts" {
  value = aws_instance.app.*.id
}

