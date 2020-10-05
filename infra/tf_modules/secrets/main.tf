resource "aws_secretsmanager_secret" "root_domain_name" {
  name                    = "${var.project_name}_root_domain_name"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "root_domain_name" {
  secret_id     = aws_secretsmanager_secret.root_domain_name.id
  secret_string = var.root_domain_name
}

resource "aws_secretsmanager_secret" "outbound_email_sender" {
  name                    = "${var.project_name}_outbound_email_sender"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "outbound_email_sender" {
  secret_id     = aws_secretsmanager_secret.outbound_email_sender.id
  secret_string = var.outbound_email_sender
}

resource "aws_secretsmanager_secret" "sendgrid_api_key" {
  name                    = "${var.project_name}_sendgrid_api_key"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "sendgrid_api_key" {
  secret_id     = aws_secretsmanager_secret.sendgrid_api_key.id
  secret_string = var.sendgrid_api_key
}

resource "aws_secretsmanager_secret" "cloudfront_keypair_id" {
  name                    = "${var.project_name}_cloudfront_keypair_id"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "cloudfront_keypair_id" {
  secret_id     = aws_secretsmanager_secret.cloudfront_keypair_id.id
  secret_string = var.cloudfront_keypair_id
}

resource "aws_secretsmanager_secret" "cloudfront_private_key" {
  name                    = "${var.project_name}_cloudfront_private_key"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "cloudfront_private_key" {
  secret_id     = aws_secretsmanager_secret.cloudfront_private_key.id
  secret_string = var.cloudfront_private_key
}
