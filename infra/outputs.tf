output "lambda_result_create_db_user_with_iam_role" {
  description = "Lambda result: create IAM DB user"
  value       = module.database.lambda_result_create_db_user_with_iam_role
}

output "deploy_upload_secret" {
  value = module.deploy_pipeline.deploy_upload_secret
}

output "deploy_upload_access_key" {
  value = module.deploy_pipeline.deploy_upload_access_key
}

output "app_deploy_bucket" {
  value = module.core_infra.app_deploy_bucket
}

output "lambda_deploy_bucket" {
  value = module.core_infra.lambda_deploy_bucket
}

output "lambda_iam_role_arn__create_db_user_with_iam_role" {
  value = module.core_infra.lambda_iam_role_arn__create_db_user_with_iam_role
}

output "lambda_iam_role_arn__renew_certificate" {
  value = module.core_infra.lambda_iam_role_arn__renew_certificate
}

output "app_security_group_ids" {
  value = module.core_infra.app_security_group_ids
}

output "app_subnet_ids" {
  value = module.core_infra.app_subnet_ids
}
