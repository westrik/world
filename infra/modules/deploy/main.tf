// Configure CodeDeploy and CodePipeline

provider "aws" {
  region = var.aws_region
}

/*
--------------------------------
CodeDeploy
--------------------------------
*/

resource "aws_codedeploy_app" "app" {
  name = "${var.project_name}_app"
}

resource "aws_codedeploy_deployment_group" "app" {
  app_name              = aws_codedeploy_app.app.name
  deployment_group_name = "${var.project_name}_app"
  service_role_arn      = aws_iam_role.codedeploy.arn

//  deployment_config_name = "CodeDeployDefault.OneAtATime"
  deployment_config_name = "CodeDeployDefault.AllAtOnce"

  ec2_tag_filter {
    key   = "Environment"
    type  = "KEY_AND_VALUE"
    value = "production"
  }

  auto_rollback_configuration {
    enabled = true
    events = [
      "DEPLOYMENT_FAILURE",
    ]
  }
}

resource "random_string" "deploy_bucket_hash" {
  length  = 6
  special = false
  upper = false
}
// TODO: add ACL and lifecycle rule
resource "aws_s3_bucket" "app_deploy" {
  bucket = "${var.project_name}-deploy-${random_string.deploy_bucket_hash.result}"
  acl    = "private"

  versioning {
    enabled = true
  }
}

resource "aws_iam_access_key" "deploy_upload" {
  user    = aws_iam_user.deploy_upload.name
}

resource "aws_iam_user" "deploy_upload" {
  name = "deploy_upload"
}

// TODO: refactor
resource "aws_iam_user_policy" "deploy_upload" {
  name = "deploy_upload"
  user = aws_iam_user.deploy_upload.name

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:PutObject*"
      ],
      "Effect": "Allow",
      "Resource": ["${aws_s3_bucket.app_deploy.arn}/*"]
    }
  ]
}
EOF
}

resource "aws_s3_bucket" "app_deploy_cloudfront" {
  bucket = "${var.project_name}-public-${random_string.deploy_bucket_hash.result}"
  acl    = "public-read"

  versioning {
    enabled = true
  }

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET"]
    allowed_origins = ["http://westrik.world:1234", "https://staging.westrikworld.com"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}

resource "aws_iam_role" "codedeploy" {
  name               = "codedeploy"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.codedeploy.json
}

resource "aws_iam_role_policy_attachment" "codedeploy" {
  role       = aws_iam_role.codedeploy.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSCodeDeployRole"
}

data "aws_iam_policy_document" "codedeploy" {
  statement {
    sid = "1"

    actions = [
      "sts:AssumeRole",
    ]

    principals {
      identifiers = ["codedeploy.amazonaws.com"]
      type        = "Service"
    }
  }
}


/*
--------------------------------
CodePipeline
--------------------------------
*/

// TODO: refactor
resource "aws_iam_role" "codepipeline" {
  name = "codepipeline"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "codepipeline.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

// TODO: refactor
resource "aws_iam_role_policy" "codepipeline" {
  name = "codepipeline_policy"
  role = aws_iam_role.codepipeline.id

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect":"Allow",
      "Action": [
        "s3:GetObject",
        "s3:GetObjectVersion",
        "s3:GetBucketVersioning",
        "s3:PutObject*"
      ],
      "Resource": [
        "${aws_s3_bucket.app_deploy.arn}",
        "${aws_s3_bucket.app_deploy.arn}/*",
        "${aws_s3_bucket.app_deploy_cloudfront.arn}/*"
      ]
    },
    {
      "Effect": "Allow",
      "Action": [
        "codedeploy:*"
      ],
      "Resource": "*"
    }
  ]
}
EOF
}

// docs: https://www.terraform.io/docs/providers/aws/r/codepipeline.html
resource "aws_codepipeline" "app" {
  name     = "${var.project_name}_prod"
  role_arn = aws_iam_role.codepipeline.arn

  // TODO: use CloudWatch S3 events for change detection (requires a CloudTrail and a CloudWatch Events rule)
  // https://docs.aws.amazon.com/codepipeline/latest/userguide/trigger-S3-migration-cwe.html
  // https://www.terraform.io/docs/providers/aws/r/cloudtrail.html
  // https://www.terraform.io/docs/providers/aws/r/cloudwatch_event_rule.html

  artifact_store {
    location = aws_s3_bucket.app_deploy.bucket
    type     = "S3"
  }

  stage {
    name = "Source_S3"

    action {
      run_order        = 1
      name             = "Source_S3"
      category         = "Source"
      owner            = "AWS"
      provider         = "S3"
      version          = "1"
      output_artifacts = ["westrikworld_app"]

      configuration = {
        S3Bucket = aws_s3_bucket.app_deploy.bucket
        S3ObjectKey = "westrikworld_app.zip"
        PollForSourceChanges = true # TODO: disable when setting up CloudWatch event triggers
        // TODO: add KMSEncryptionKeyARN
      }
    }
  }

  // TODO: change Source to pull from GitHub & add Build stage that triggers CodeBuild
  // TODO: add a Test stage

  stage {
    name = "Deploy_EC2"

    action {
      run_order       = 2
      name            = "Deploy_EC2"
      category        = "Deploy"
      owner           = "AWS"
      provider        = "CodeDeploy"
      version         = "1"
      input_artifacts = ["westrikworld_app"]

      configuration = {
        ApplicationName = aws_codedeploy_app.app.name
        DeploymentGroupName = aws_codedeploy_deployment_group.app.deployment_group_name
      }
    }
  }

  stage {
    name = "Deploy_CloudFront"

    action {
      run_order       = 3
      name            = "Deploy_CloudFront"
      category        = "Deploy"
      owner           = "AWS"
      provider        = "S3"
      version         = "1"
      input_artifacts = ["westrikworld_app"]

      configuration = {
        BucketName = aws_s3_bucket.app_deploy_cloudfront.bucket
        Extract = true
        CannedACL = "public-read"
      }
    }
  }
}

resource "aws_cloudfront_distribution" "app" {
  origin {
    domain_name = aws_s3_bucket.app_deploy_cloudfront.bucket_domain_name
    origin_id   = "public"
    origin_path = "/public"

//    s3_origin_config {
//      origin_access_identity = "origin-access-identity/cloudfront/ABCDEFG1234567" // TODO: replace
//    }
  }

  enabled             = true
  is_ipv6_enabled     = true
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
    error_code = 404
    response_page_path = "/index.html"
    response_code = 200
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
    acm_certificate_arn = aws_acm_certificate.cloudfront.arn
    ssl_support_method = "sni-only"
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
  ttl     = 60
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
