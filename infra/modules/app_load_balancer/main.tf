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

# /**
# Invoke Lambda to renew Let's Encrypt certificate
# Expects the lambda to have been deployed with the SAM CLI (see infra/lambda/README.md)
# */
# data "aws_lambda_invocation" "renew_certificate" {
#   function_name = "renew-certificate"
#   depends_on    = [aws_secretsmanager_secret.api_cert]
#
#   input = <<JSON
# {
#   "domains": ["${var.api_domain_name}"],
#   "email": "${var.admin_email}",
#   "secret_id": "${aws_secretsmanager_secret.api_cert.name}"
# }
# JSON
# }
