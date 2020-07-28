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
}

resource "aws_iam_access_key" "user_content_upload" {
  user = aws_iam_user.user_content_upload.name
}

resource "aws_iam_user" "user_content_upload" {
  name = "${var.project_slug}-${var.deploy_name}-user-content-upload"
}

resource "aws_iam_user_policy" "user_content_upload" {
  name = "${var.project_slug}-${var.deploy_name}-user-content-upload"
  user = aws_iam_user.user_content_upload.name

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:PutObject*"
      ],
      "Effect": "Allow",
      "Resource": ["${aws_s3_bucket.user_uploads.arn}/*"]
    }
  ]
}
EOF
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

// TODO: cloudfront distribution for user content

// TODO: cloudfront + S3 cookie generation policies?
