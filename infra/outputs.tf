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

output "deploy_upload_bucket" {
  value = module.core_infra.app_deploy_bucket
}
