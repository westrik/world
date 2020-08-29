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
    allowed_origins = ["https://${var.root_domain_name}"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}

// TODO: split IAM role for worker and app hosts (worker needs Get+Put, app only needs Put)
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

data "aws_iam_policy_document" "access_user_uploads_via_cloudfront_oai" {
  statement {
    actions = ["s3:GetObject"]
    // TODO: restrict access via signed cookie policy...
    // see https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-setting-signed-cookie-custom-policy.html
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

locals {
  user_uploads_origin_id   = "user_uploads"
  user_uploads_domain_name = "uploads.${var.root_domain_name}"
}

resource "aws_cloudfront_distribution" "user_uploads" {
  comment = "Grant access to user-uploaded content"

  aliases = [local.user_uploads_domain_name]

  enabled         = true
  is_ipv6_enabled = true
  price_class     = "PriceClass_100"

  origin {
    domain_name = aws_s3_bucket.user_uploads.bucket_regional_domain_name
    origin_id   = local.user_uploads_origin_id

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.user_uploads.cloudfront_access_identity_path
    }
  }

  viewer_certificate {
    acm_certificate_arn      = aws_acm_certificate.user_uploads_cloudfront.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2019"
  }

  // TODO: set up loggging to S3
  //  logging_config {
  //    include_cookies = false
  //    bucket          = "mylogs.s3.amazonaws.com"
  //    prefix          = "myprefix"
  //  }

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD", "OPTIONS"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = local.user_uploads_origin_id
    trusted_signers  = ["self"]

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
}

resource "aws_acm_certificate" "user_uploads_cloudfront" {
  domain_name       = local.user_uploads_domain_name
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }

  tags = {
    Environment = var.deploy_name
    Project     = var.project_name
  }
}

resource "aws_route53_record" "user_uploads_cloudfront_acm" {
  for_each = {
    for dvo in aws_acm_certificate.user_uploads_cloudfront.domain_validation_options : dvo.domain_name => {
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

resource "aws_acm_certificate_validation" "user_uploads_cloudfront" {
  for_each = {
    for dvo in aws_acm_certificate.user_uploads_cloudfront.domain_validation_options : dvo.domain_name => {}
  }
  certificate_arn         = aws_acm_certificate.user_uploads_cloudfront.arn
  validation_record_fqdns = [aws_route53_record.user_uploads_cloudfront_acm[each.key].fqdn]
}

data "aws_route53_zone" "app" {
  name = "${var.root_domain_name}."
}

resource "aws_route53_record" "user_uploads_cloudfront" {
  zone_id = data.aws_route53_zone.app.id
  name    = local.user_uploads_domain_name
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.user_uploads.domain_name
    zone_id                = aws_cloudfront_distribution.user_uploads.hosted_zone_id
    evaluate_target_health = false
  }
}


/* CONTENT BUCKET FOR DEV & TESTING ONLY */
// TODO: refactor to re-use the `content_buckets` module for prod + test

resource "aws_s3_bucket" "dev_content_upload" {
  bucket = "${var.project_slug}-dev-user-uploads-${random_string.content_bucket_hash.result}"
  acl    = "private"

  versioning {
    enabled = false
  }

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["PUT"]
    allowed_origins = ["https://local.${var.root_domain_name}"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}
