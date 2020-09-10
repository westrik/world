variable "project_name" {
  type = string
}

variable "project_slug" {
  type = string
}

variable "deploy_name" {
  type = string
}

variable "aws_region" {
  type = string
}

variable "aws_az1" {
  type = string
}

variable "aws_az2" {
  type = string
}

variable "api_domain_name" {
  type = string
}

variable "root_domain_name" {
  type = string
}

variable "admin_email" {
  type = string
}

variable "app_vpc_id" {
  type = string
}

variable "app_security_group_ids" {
  type = list(string)
}

variable "app_subnet_ids" {
  type = list(string)
}

# output from module.core_infra.lambda_iam_roles
variable "lambda_iam_role_arn__renew_certificate" {
  type = string
}

# output from module.core_infra.s3_buckets
variable "lambda_deploy_bucket" {
  type = string
}
