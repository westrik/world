output "app_subnet_ids" {
  value = [aws_subnet.app_az1.id, aws_subnet.app_az2.id]
}

output "app_security_groups" {
  value = [aws_security_group.app_inbound.id, aws_security_group.app_outbound.id, aws_security_group.app_outbound_s3.id]
}

output "app_vpc_id" {
  value = aws_vpc.app.id
}

output "instance_security_group_ids" {
  value = [
    aws_security_group.app_inbound.id,
    aws_security_group.app_outbound.id,
  aws_security_group.app_outbound_s3.id]
}
