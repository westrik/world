output "app_vpc_id" {
  value = aws_vpc.app.id
}

output "app_subnet_ids" {
  value = [aws_subnet.app_az1.id, aws_subnet.app_az2.id]
}

output "app_security_group_ids" {
  value = module.security_groups.app_security_group_ids
}

output "consul_security_group_ids" {
  value = module.security_groups.consul_security_group_ids
}

output "nomad_security_group_ids" {
  value = module.security_groups.nomad_security_group_ids
}
