variable "project_name" {
  default = "westrikworld"
}

variable "project_slug" {
  default = "westrikworld"
}

variable "deploy_name" {
  default = "production"
}

variable "aws_region" {
  default = "us-east-1"
}

variable "aws_az1" {
  default = "us-east-1a"
}

variable "aws_az2" {
  default = "us-east-1b"
}

variable "api_domain_name" {
  default = "api.westrikworld.com"
}

variable "root_domain_name" {
  default = "westrikworld.com"
}

variable "admin_email" {
  type    = string
  default = "m+wwadmin@ttwestrik.com"
}

variable "provisioning_lambda" {
  type    = bool
  default = false
}

variable "num_app_instances" {
  type    = number
  default = 1
}
