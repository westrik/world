variable "project_slug" {
  type = string
}

variable "deploy_name" {
  type = string
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

variable "app_lb_listener_arn" {
  type = string
}

variable "app_target_group_name" {
  type = string
}

variable "app_blue_autoscaling_group_id" {
  type = string
}

variable "app_green_autoscaling_group_id" {
  type = string
}
