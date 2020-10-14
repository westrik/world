variable "project_name" {
  type = string
}

variable "app_host_iam_role_id" {
  type = string
}

# output from module.core_infra.s3_buckets
variable "lambda_deploy_bucket" {
  type = string
}
