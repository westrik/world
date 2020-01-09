output "deploy_upload_secret" {
  value = "${aws_iam_access_key.deploy_upload.secret}"
}

output "deploy_upload_user" {
  value = "${aws_iam_access_key.deploy_upload.user}"
}

output "deploy_upload_bucket" {
  value = "${aws_s3_bucket.app_deploy.bucket_domain_name}"
}
