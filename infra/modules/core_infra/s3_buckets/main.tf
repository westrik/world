provider "aws" {
  region = var.aws_region
}

resource "random_string" "deploy_bucket_hash" {
  length  = 6
  special = false
  upper   = false
}
// TODO: add ACL and lifecycle rule
resource "aws_s3_bucket" "app_deploy" {
  bucket = "${var.project_slug}-deploy-${random_string.deploy_bucket_hash.result}"
  acl    = "private"

  versioning {
    enabled = true
  }
}

resource "aws_s3_bucket" "app_deploy_cloudfront" {
  bucket = "${var.project_slug}-public-${random_string.deploy_bucket_hash.result}"
  acl    = "public-read"

  versioning {
    enabled = true
  }

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET"]
    allowed_origins = ["https://local.westrik.world", "https://staging.westrik.world"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}

resource "aws_s3_bucket" "lambda_deploy" {
  bucket = "${var.project_slug}-lambda-deploy-${random_string.deploy_bucket_hash.result}"
  acl    = "private"

  versioning {
    enabled = true
  }
}
