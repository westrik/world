output "app_subnets" {
  value = [aws_subnet.app_az1.id, aws_subnet.app_az2.id]
}

output "app_security_groups" {
  value = [aws_security_group.app.id]
}

output "app_vpc" {
  value = aws_vpc.app.id
}
