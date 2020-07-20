// Configure S3 bucket for custom site

provider "aws" {
  region = var.aws_region
}

//resource "aws_s3_bucket" "user_site" {
//  bucket = "${var.project_slug}-user-site-${random_string.deploy_bucket_hash.result}"
//  acl    = "public-read"
//
//  versioning {
//    enabled = true
//  }
//}

resource "aws_iam_access_key" "content_upload" {
  user = aws_iam_user.content_upload.name
}

resource "aws_iam_user" "content_upload" {
  name = "content_upload"
}

resource "aws_iam_user_policy" "content_upload" {
  name = "content_upload"
  user = aws_iam_user.content_upload.name

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:PutObject*"
      ],
      "Effect": "Allow",
      "Resource": ["${TODO_BUCKET_ARN}/*"]
    }
  ]
}
EOF
}

// TODO: cloudfront distribution for user site
