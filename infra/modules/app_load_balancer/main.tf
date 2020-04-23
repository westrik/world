// - Set up an NLB to point to the application server subnet
// - Request a TLS certificate from ACM for the LB
// - Point Route 53 DNS at the LB

provider "aws" {
  region = var.aws_region
}

data "aws_route53_zone" "app" {
  name = "${var.root_domain_name}."
}

/*
TODO(later):
  - [ ] handle IPv6
  - [ ] provision ACM private cert to use with NLB
*/

resource "aws_lb" "app" {
  name               = "app-nlb"
  load_balancer_type = "network"
  subnets            = var.app_subnet_ids

  //  TODO: set up access log bucket
  //    access_logs = {
  //      bucket = module.log_bucket.this_s3_bucket_id
  //    }

  tags = {
    Environment = var.deploy_name
    Project     = var.project_name
  }
}

resource "aws_lb_target_group" "app_insecure" {
  name     = "app-lb-insecure-target-group"
  port     = 80
  protocol = "TCP"
  vpc_id   = var.app_vpc_id
}
resource "aws_lb_target_group_attachment" "app_insecure" {
  count            = length(var.app_instance_ids)
  target_id        = var.app_instance_ids[count.index]
  target_group_arn = aws_lb_target_group.app_insecure.arn
  port             = 80
}
resource "aws_lb_listener" "app_insecure" {
  load_balancer_arn = aws_lb.app.arn
  port              = 80
  protocol          = "TCP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.app_insecure.arn
  }
}


resource "aws_lb_target_group" "app" {
  name     = "app-lb-target-group"
  port     = 443
  protocol = "TCP_UDP"
  vpc_id   = var.app_vpc_id
}
resource "aws_lb_target_group_attachment" "app" {
  count            = length(var.app_instance_ids)
  target_id        = var.app_instance_ids[count.index]
  target_group_arn = aws_lb_target_group.app.arn
  port             = 443
}
resource "aws_lb_listener" "app" {
  load_balancer_arn = aws_lb.app.arn
  port              = 443
  protocol          = "TCP_UDP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.app.arn
  }
}

resource "aws_route53_record" "app" {
  zone_id = data.aws_route53_zone.app.id
  name    = var.api_domain_name
  type    = "A"

  alias {
    name                   = aws_lb.app.dns_name
    zone_id                = aws_lb.app.zone_id
    evaluate_target_health = false // TODO: enable?
  }
}

resource "aws_route53_record" "app_caa" {
  zone_id = data.aws_route53_zone.app.id
  name    = var.api_domain_name
  type    = "CAA"
  records = ["0 issue \"amazon.com\""]
  ttl     = 60
}



resource "aws_secretsmanager_secret" "api_cert" {
  name                    = "${var.project_slug}_api_cert"
  recovery_window_in_days = 0
}

/** Lambda for renewing certificate with Let's Encrypt */
resource "aws_lambda_function" "renew_certificate" {
  function_name = "renew_certificate"
  role          = aws_iam_role.lambda_renew_certificate.arn
  // TODO: zip needs to be built on ami-0080e4c5bc078760e
  // see https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html
  filename = "./lambda/renew_certificate.zip"
  // TODO: pull from S3
  //  s3_bucket = ""
  //  s3_key = ""
  handler = "renew_certificate.lambda_handler"
  runtime = "python3.7"

  vpc_config {
    security_group_ids = var.app_security_group_ids
    subnet_ids         = var.app_subnet_ids
  }
}

data "aws_iam_policy_document" "lambda_renew_certificate" {
  statement {
    sid = "1"

    actions = [
      "sts:AssumeRole",
    ]

    principals {
      identifiers = ["lambda.amazonaws.com"]
      type        = "Service"
    }
  }
}

resource "aws_iam_role" "lambda_renew_certificate" {
  name               = "lambda_renew_certificate"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.lambda_renew_certificate.json
}

resource "aws_iam_role_policy_attachment" "role_attach_lambda_vpc" {
  role       = aws_iam_role.lambda_renew_certificate.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}

resource "aws_iam_role_policy_attachment" "role_attach_lambda_secrets_manager" {
  role       = aws_iam_role.lambda_renew_certificate.name
  policy_arn = "arn:aws:iam::aws:policy/SecretsManagerReadWrite"
}

//data "aws_lambda_invocation" "renew_certificate" {
//  function_name = aws_lambda_function.renew_certificate.function_name
//  depends_on    = [aws_lambda_function.renew_certificate]
//
//  input = <<JSON
//{
//  "domains": ["${var.api_domain_name}"],
//  "email": "${var.admin_email}",
//  "secret_id": "${aws_secretsmanager_secret.api_cert.name}"
//}
//JSON
//}
//
