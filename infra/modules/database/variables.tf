variable "project_name" {
  type = string
}

variable "project_slug" {
  type = string
}

variable "deploy_name" {
  type = string
}

variable "db_username" {
  type    = string
  default = "westrikworld_admin"
}

variable "admin_user_arn" {
  type = string
}

# output from module.network
variable "app_subnet_ids" {
  type = list(string)
}

# output from module.network
variable "app_security_group_ids" {
  type = list(string)
}

# output from module.network
variable "app_vpc_id" {
  type = string
}

# output from module.core_infra.lambda_iam_roles
variable "lambda_iam_role_arn__create_db_with_iam_role" {
  type = string
}

# output from module.core_infra.s3_buckets
variable "lambda_deploy_bucket" {
  type = string
}
