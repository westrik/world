/*
--------------------------------------------------
Security groups for app instances
--------------------------------------------------
*/

resource "aws_security_group" "app_inbound" {
  name        = "app_in_sg"
  description = "[${var.project_slug}-${var.deploy_name}] inbound app instance security group"
  vpc_id      = var.vpc_id

  # Inbound SSH (TODO: disable)
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Inbound HTTP via NLB
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  # Inbound HTTPS via NLB
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

}
resource "aws_security_group" "app_outbound_s3" {
  name        = "app_out_s3_sg"
  description = "[${var.project_slug}-${var.deploy_name}] app instance security group for traffic outbound to S3"
  vpc_id      = var.vpc_id

  # Outbound HTTPS access to S3 (via VPC endpoint)
  egress {
    from_port       = 443
    to_port         = 443
    protocol        = "tcp"
    prefix_list_ids = var.prefix_list_ids
  }

  # Outbound DNS access TODO: VPC endpoint?
  egress {
    from_port   = 53
    to_port     = 53
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
}

resource "aws_security_group" "app_outbound" {
  name        = "app_out_sg"
  description = "[${var.project_slug}-${var.deploy_name}] app instance security group for outbound traffic"
  vpc_id      = var.vpc_id

  # Outbound HTTPS to AWS (CodeDeploy)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] #data.aws_ip_ranges.amazon_services.cidr_blocks
  }

  # RDS
  egress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
}


locals {
  consul_ports_tcp_only = [
    //    8500, # HTTP
    8501, # HTTPS
    8300, # Server RPC
    8502, # gRPC
  ]
  consul_ports_tcp_and_udp = [
    8600, # DNS
    8301, # Serf LAN
    8302, # Serf WAN
  ]
  consul_cidr_blocks = ["10.0.0.0/16"]
}

resource "aws_security_group" "consul" {
  name        = "app_consul_sg"
  description = "[${var.project_slug}-${var.deploy_name}] consul security group"
  vpc_id      = var.vpc_id
}

resource "aws_security_group_rule" "allow_consul_tcp_ingress" {
  for_each = {
    for port in concat(local.consul_ports_tcp_only, local.consul_ports_tcp_and_udp) : port => {}
  }
  from_port         = each.key
  to_port           = each.key
  protocol          = "tcp"
  type              = "ingress"
  security_group_id = aws_security_group.consul.id
  cidr_blocks       = local.consul_cidr_blocks
}

resource "aws_security_group_rule" "allow_consul_tcp_egress" {
  for_each = {
    for port in concat(local.consul_ports_tcp_only, local.consul_ports_tcp_and_udp) : port => {}
  }
  from_port         = each.key
  to_port           = each.key
  protocol          = "tcp"
  type              = "egress"
  security_group_id = aws_security_group.consul.id
  cidr_blocks       = local.consul_cidr_blocks
}

resource "aws_security_group_rule" "allow_consul_udp_ingress" {
  for_each = {
    for port in local.consul_ports_tcp_and_udp : port => {}
  }
  from_port         = each.key
  to_port           = each.key
  protocol          = "udp"
  type              = "ingress"
  security_group_id = aws_security_group.consul.id
  cidr_blocks       = local.consul_cidr_blocks
}

resource "aws_security_group_rule" "allow_consul_udp_egress" {
  for_each = {
    for port in local.consul_ports_tcp_and_udp : port => {}
  }
  from_port         = each.key
  to_port           = each.key
  protocol          = "udp"
  type              = "egress"
  security_group_id = aws_security_group.consul.id
  cidr_blocks       = local.consul_cidr_blocks
}

resource "aws_security_group_rule" "allow_consul_tcp_ingress_http" {
  from_port         = 8500
  to_port           = 8500
  protocol          = "tcp"
  type              = "ingress"
  security_group_id = aws_security_group.consul.id
  cidr_blocks       = ["0.0.0.0/0"]
}

resource "aws_security_group_rule" "allow_consul_tcp_egress_http" {
  from_port         = 8500
  to_port           = 8500
  protocol          = "tcp"
  type              = "egress"
  security_group_id = aws_security_group.consul.id
  cidr_blocks       = ["0.0.0.0/0"]
}
