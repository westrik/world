// Configure S3 bucket for user-uploaded content (and system-generated derivative files)

provider "aws" {
  region = var.aws_region
}

resource "random_string" "content_bucket_hash" {
  length  = 6
  special = false
  upper   = false
}

resource "aws_s3_bucket" "user_uploads" {
  bucket = "${var.project_slug}-${var.deploy_name}-user-uploads-${random_string.content_bucket_hash.result}"
  acl    = "private"

  versioning {
    enabled = false
  }

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["PUT"]
    allowed_origins = ["https://westrik.world"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}

resource "aws_iam_role_policy" "app_host_allow_content_upload" {
  name = "${var.project_slug}_app_host-user-content-upload"
  role = var.app_host_iam_role_id

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:PutObject*",
        "s3:GetObject*"
      ],
      "Effect": "Allow",
      "Resource": ["${aws_s3_bucket.user_uploads.arn}/*"]
    }
  ]
}
EOF
}

resource "aws_secretsmanager_secret" "user_content_bucket_name" {
  name                    = "${var.project_slug}_content_bucket_name"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_url" {
  secret_id     = aws_secretsmanager_secret.user_content_bucket_name.id
  secret_string = aws_s3_bucket.user_uploads.bucket
}




resource "aws_cloudfront_origin_access_identity" "user_uploads" {
  comment = "Access user-uploaded content via CloudFront"
}

resource "aws_cloudfront_distribution" "user_uploads" {
  enabled = false
  default_cache_behavior {
    allowed_methods        = []
    cached_methods         = []
    target_origin_id       = ""
    viewer_protocol_policy = ""
    forwarded_values {
      query_string = false
      cookies {
        forward = ""
      }
    }
  }
  restrictions {
    geo_restriction {
      restriction_type = ""
    }
  }
  viewer_certificate {}

  origin {
    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.user_uploads.cloudfront_access_identity_path
    }
    domain_name = ""
    origin_id   = ""
  }
}

data "aws_iam_policy_document" "access_user_uploads_via_cloudfront_oai" {
  statement {
    actions   = ["s3:GetObject"]
    resources = ["${aws_s3_bucket.user_uploads.arn}/*"]

    principals {
      type        = "AWS"
      identifiers = [aws_cloudfront_origin_access_identity.user_uploads.iam_arn]
    }
  }
}

resource "aws_s3_bucket_policy" "access_user_uploads_via_cloudfront_oai" {
  bucket = aws_s3_bucket.user_uploads.id
  policy = data.aws_iam_policy_document.access_user_uploads_via_cloudfront_oai.json
}


//resource "aws_cloudfront_distribution" "app" {
//  origin {
//    domain_name = var.deploy_cloudfront_bucket_domain_name
//    origin_id   = "public"
//    origin_path = "/public"
//
//    //    s3_origin_config {
//    //      origin_access_identity = "origin-access-identity/cloudfront/ABCDEFG1234567" // TODO: replace
//    //    }
//  }
//
//  enabled         = true
//  is_ipv6_enabled = true
//  //  comment             = ""
//  default_root_object = "index.html"
//
//  //  logging_config {
//  //    include_cookies = false
//  //    bucket          = "mylogs.s3.amazonaws.com"
//  //    prefix          = "myprefix"
//  //  }
//
//  aliases = [var.root_domain_name]
//
//  default_cache_behavior {
//    allowed_methods  = ["GET", "HEAD", "OPTIONS"]
//    cached_methods   = ["GET", "HEAD"]
//    target_origin_id = "public"
//
//    forwarded_values {
//      query_string = false
//
//      cookies {
//        forward = "none"
//      }
//    }
//
//    compress               = true
//    viewer_protocol_policy = "redirect-to-https"
//    min_ttl                = 0
//    default_ttl            = 3600
//    max_ttl                = 86400
//  }
//
//  custom_error_response {
//    error_code         = 404
//    response_page_path = "/index.html"
//    response_code      = 200
//  }
//
//  price_class = "PriceClass_100"
//
//  restrictions {
//    geo_restriction {
//      restriction_type = "whitelist"
//      locations        = ["US", "CA"]
//    }
//  }
//
//  tags = {
//    Environment = var.deploy_name
//    Project     = var.project_name
//  }
//
//  viewer_certificate {
//    acm_certificate_arn      = aws_acm_certificate.cloudfront.arn
//    ssl_support_method       = "sni-only"
//    minimum_protocol_version = "TLSv1.2_2019"
//  }
//}
//
//resource "aws_acm_certificate" "cloudfront" {
//  domain_name       = var.root_domain_name
//  validation_method = "DNS"
//
//  tags = {
//    Environment = var.deploy_name
//    Project     = var.project_name
//  }
//
//  lifecycle {
//    create_before_destroy = true
//  }
//}
//
//resource "aws_route53_record" "cloudfront_acm" {
//  zone_id = data.aws_route53_zone.app.id
//  name    = aws_acm_certificate.cloudfront.domain_validation_options[0].resource_record_name
//  type    = aws_acm_certificate.cloudfront.domain_validation_options[0].resource_record_type
//  records = [
//    aws_acm_certificate.cloudfront.domain_validation_options[0].resource_record_value]
//  ttl = 60
//}
//
//resource "aws_acm_certificate_validation" "cloudfront" {
//  certificate_arn         = aws_acm_certificate.cloudfront.arn
//  validation_record_fqdns = [aws_route53_record.cloudfront_acm.fqdn]
//}
//
//data "aws_route53_zone" "app" {
//  name = "${var.root_domain_name}."
//}
//
//resource "aws_route53_record" "app" {
//  zone_id = data.aws_route53_zone.app.id
//  name    = var.root_domain_name
//  type    = "A"
//
//  alias {
//    name                   = aws_cloudfront_distribution.app.domain_name
//    zone_id                = aws_cloudfront_distribution.app.hosted_zone_id
//    evaluate_target_health = false
//  }
//}


/* CONTENT BUCKET FOR DEV & TESTING ONLY */
// TODO: move to separate AWS account

resource "aws_s3_bucket" "dev_content_upload" {
  bucket = "${var.project_slug}-dev-user-uploads-${random_string.content_bucket_hash.result}"
  acl    = "private"

  versioning {
    enabled = false
  }

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["PUT"]
    allowed_origins = ["https://local.westrik.world"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}
