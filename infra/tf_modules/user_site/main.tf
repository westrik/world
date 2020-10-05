provider "aws" {
  region = var.aws_region
}

resource "random_string" "user_test_hash" {
  length  = 6
  special = false
  upper   = false
}

locals {
  cloudfront_domain_name_suffix = "sites.${var.root_domain_name}"
  test_cloudfront_domain_name   = "${random_string.user_test_hash.result}.${local.cloudfront_domain_name_suffix}"
}

// TODO: IAM user for uploads, output info

resource "aws_s3_bucket" "user_test_cloudfront" {
  bucket = "${var.project_slug}-user-public-${random_string.user_test_hash.result}"
  acl    = "public-read"

  versioning {
    enabled = true
  }
}

resource "aws_cloudfront_distribution" "user_test" {
  origin {
    domain_name = aws_s3_bucket.user_test_cloudfront.bucket_domain_name
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

  // TODO: Need to include alias_domain_names in ACM cert somehow
  //  aliases = concat([local.test_cloudfront_domain_name], var.alias_domain_names)
  aliases = [local.test_cloudfront_domain_name]

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
    Environment = var.deploy_name
    Project     = var.project_name
  }

  viewer_certificate {
    acm_certificate_arn      = aws_acm_certificate.user_test_cloudfront.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2019"
  }
}

resource "aws_acm_certificate" "user_test_cloudfront" {
  domain_name       = local.test_cloudfront_domain_name
  validation_method = "DNS"

  tags = {
    Environment = var.deploy_name
    Project     = var.project_name
  }

  lifecycle {
    create_before_destroy = true
  }
}

data "aws_route53_zone" "app" {
  name = "${var.root_domain_name}."
}

resource "aws_route53_record" "user_test_cloudfront_acm" {
  for_each = {
    for dvo in aws_acm_certificate.user_test_cloudfront.domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }
  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = data.aws_route53_zone.app.id
}

resource "aws_acm_certificate_validation" "cloudfront" {
  for_each = {
    for dvo in aws_acm_certificate.user_test_cloudfront.domain_validation_options : dvo.domain_name => {}
  }
  certificate_arn         = aws_acm_certificate.user_test_cloudfront.arn
  validation_record_fqdns = [aws_route53_record.user_test_cloudfront_acm[each.key].fqdn]
}

resource "aws_route53_record" "user_test" {
  zone_id = data.aws_route53_zone.app.id
  name    = local.test_cloudfront_domain_name
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.user_test.domain_name
    zone_id                = aws_cloudfront_distribution.user_test.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "user_test_caa" {
  zone_id = data.aws_route53_zone.app.id
  name    = "*.${local.cloudfront_domain_name_suffix}"
  type    = "CAA"
  records = ["0 issue \"amazon.com\""]
  ttl     = 60
}


/* User access key: */

resource "aws_iam_access_key" "user_upload" {
  user = aws_iam_user.user_upload.name
}

resource "aws_iam_user" "user_upload" {
  name = "test_user_upload"
}

resource "aws_iam_user_policy" "user_upload" {
  name = "test_user_upload"
  user = aws_iam_user.user_upload.name

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:PutObject*"
      ],
      "Effect": "Allow",
      "Resource": ["${aws_s3_bucket.user_test_cloudfront.arn}/*"]
    }
  ]
}
EOF
}
