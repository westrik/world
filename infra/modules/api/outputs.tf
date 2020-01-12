output "app_subnets" {
  value = [aws_subnet.app_az1.id, aws_subnet.app_az2.id]
}

output "app_security_groups" {
  value = [aws_security_group.app_inbound.id, aws_security_group.app_outbound.id, aws_security_group.app_outbound_s3.id]
}

output "app_vpc" {
  value = aws_vpc.app.id
}

output "app_deploy_hosts" {
  value = [aws_instance.app.id]
}

#TODO: remove
output "instance_ip" {
  value = aws_instance.app.public_ip
}
output "instance_private_key_pem" {
  value = tls_private_key.westrikworld_staging_key.private_key_pem
}
