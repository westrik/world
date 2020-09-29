output "app_vpc_id" {
  value = module.network.app_vpc_id
}

output "app_subnet_ids" {
  value = module.network.app_subnet_ids
}

output "app_security_group_ids" {
  value = module.network.app_security_group_ids
}

output "app_deploy_bucket" {
  value = module.s3_buckets.app_deploy_bucket
}

output "app_deploy_bucket_arn" {
  value = module.s3_buckets.app_deploy_bucket_arn
}

output "consul_security_group_ids" {
  value = module.network.consul_security_group_ids
}

output "lambda_deploy_bucket" {
  value = module.s3_buckets.lambda_deploy_bucket
}

output "app_deploy_cloudfront_bucket" {
  value = module.s3_buckets.app_deploy_cloudfront_bucket
}

output "app_deploy_cloudfront_bucket_arn" {
  value = module.s3_buckets.app_deploy_cloudfront_bucket_arn
}

output "app_deploy_cloudfront_bucket_domain_name" {
  value = module.s3_buckets.app_deploy_cloudfront_bucket_domain_name
}

output "lambda_iam_role_arn__create_db_with_iam_role" {
  value = module.lambda_iam_roles.lambda_iam_role_arn__create_db_with_iam_role
}

output "lambda_iam_role_arn__renew_certificate" {
  value = module.lambda_iam_roles.lambda_iam_role_arn__renew_certificate
}
