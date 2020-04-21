output "app_deploy_bucket" {
  value = aws_s3_bucket.app_deploy.bucket
}

output "app_deploy_bucket_arn" {
  value = aws_s3_bucket.app_deploy.arn
}

output "app_deploy_cloudfront_bucket" {
  value = aws_s3_bucket.app_deploy_cloudfront.bucket
}

output "app_deploy_cloudfront_bucket_arn" {
  value = aws_s3_bucket.app_deploy_cloudfront.arn
}

output "app_deploy_cloudfront_bucket_domain_name" {
  value = aws_s3_bucket.app_deploy_cloudfront.bucket_domain_name
}

