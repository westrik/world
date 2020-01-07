output "app_subnets" {
  value = [aws_subnet.ww_prod_app_az1.id, aws_subnet.ww_prod_app_az2.id]
}

output "app_security_groups" {
  value = [aws_security_group.ww_prod_app.id]
}
