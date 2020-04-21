provider "aws" {
  region = var.aws_region
}

resource "aws_cloudfront_distribution" "app" {
  origin {
    domain_name = var.deploy_cloudfront_bucket_domain_name
    origin_id   = "public"
    origin_path = "/public"

    //    s3_origin_config {
    //      origin_access_identity = "origin-access-identity/cloudfront/ABCDEFG1234567" // TODO: replace
    //    }
  }

  enabled         = true
  is_ipv6_enabled = true
  //  comment             = ""
  default_root_object = "index.html"

  //  logging_config {
  //    include_cookies = false
  //    bucket          = "mylogs.s3.amazonaws.com"
  //    prefix          = "myprefix"
  //  }

  aliases = [var.root_domain_name]

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD", "OPTIONS"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "public"

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }
    }

    compress               = true
    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  custom_error_response {
    error_code         = 404
    response_page_path = "/index.html"
    response_code      = 200
  }

  price_class = "PriceClass_100"

  restrictions {
    geo_restriction {
      restriction_type = "whitelist"
      locations        = ["US", "CA"]
    }
  }

  tags = {
    Environment = "production"
  }

  viewer_certificate {
    acm_certificate_arn      = aws_acm_certificate.cloudfront.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2018"
  }
}

resource "aws_acm_certificate" "cloudfront" {
  domain_name       = var.root_domain_name
  validation_method = "DNS"

  tags = {
    Environment = "production"
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_route53_record" "cloudfront_acm" {
  zone_id = data.aws_route53_zone.app.id
  name    = aws_acm_certificate.cloudfront.domain_validation_options[0].resource_record_name
  type    = aws_acm_certificate.cloudfront.domain_validation_options[0].resource_record_type
  records = [
  aws_acm_certificate.cloudfront.domain_validation_options[0].resource_record_value]
  ttl = 60
}

resource "aws_acm_certificate_validation" "cloudfront" {
  certificate_arn         = aws_acm_certificate.cloudfront.arn
  validation_record_fqdns = [aws_route53_record.cloudfront_acm.fqdn]
}

data "aws_route53_zone" "app" {
  name = "${var.root_domain_name}."
}

resource "aws_route53_record" "app" {
  zone_id = data.aws_route53_zone.app.id
  name    = var.root_domain_name
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.app.domain_name
    zone_id                = aws_cloudfront_distribution.app.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "app_caa" {
  zone_id = data.aws_route53_zone.app.id
  name    = var.root_domain_name
  type    = "CAA"
  records = ["0 issue \"amazon.com\""]
  ttl     = 60
}
