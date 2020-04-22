variable "project_name" {
  type    = string
  default = "westrikworld"
}

variable "aws_region" {
  type = string
}

variable "deploy_bucket" {
  type = string
}

variable "deploy_bucket_arn" {
  type = string
}

variable "deploy_cloudfront_bucket" {
  type = string
}

variable "deploy_cloudfront_bucket_arn" {
  type = string
}
