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

// TODO: allow EC2 instances to access S3 bucket

// TODO: cloudfront distribution for user content

// TODO: cloudfront + S3 cookie generation policies?
