data "aws_lambda_invocation" "renew_certificate" {
  function_name = "renew_certificate"
  depends_on    = [aws_secretsmanager_secret.consul_cert]

  input = <<JSON
{
  "domains": [${join(", ", formatlist("\"%s\"", var.consul_domain_names))}],
  "email": "${var.admin_email}",
  "secret_id": "${aws_secretsmanager_secret.consul_cert.name}"
}
JSON
}

resource "aws_secretsmanager_secret" "consul_cert" {
  name                    = "${var.project_slug}_consul_cert"
  recovery_window_in_days = 0
}
