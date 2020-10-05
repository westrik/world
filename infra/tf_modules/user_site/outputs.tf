output "site_bucket" {
  value = aws_s3_bucket.user_test_cloudfront.bucket
}

output "upload_secret" {
  value = aws_iam_access_key.user_upload.secret
}

output "upload_access_key" {
  value = aws_iam_access_key.user_upload.id
}
