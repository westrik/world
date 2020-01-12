output "deploy_upload_secret" {
  value = aws_iam_access_key.deploy_upload.secret
}

output "deploy_upload_access_key" {
  value = aws_iam_access_key.deploy_upload.id
}

output "deploy_upload_bucket" {
  value = aws_s3_bucket.app_deploy.bucket
}
