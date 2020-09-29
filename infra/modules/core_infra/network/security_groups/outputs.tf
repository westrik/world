output "app_security_group_ids" {
  value = [
    aws_security_group.app_inbound.id,
    aws_security_group.app_outbound.id,
    aws_security_group.app_outbound_s3.id
  ]
}
