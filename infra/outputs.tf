output "lambda_result_create_db_user_with_iam_role" {
  description = "Lambda result: create IAM DB user"
  value       = module.database.lambda_result_create_db_user_with_iam_role
}

output "deploy_upload_secret" {
  value = module.deploy.deploy_upload_secret
}

output "deploy_upload_access_key" {
  value = module.deploy.deploy_upload_access_key
}

output "deploy_upload_bucket" {
  value = module.deploy.deploy_upload_bucket
}

output "instance_ip" {
  value = module.api.instance_ip
}

output "instance_private_key_pem" {
  value = module.api.instance_private_key_pem
}
