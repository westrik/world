variable "project_name" {
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

variable "app_security_group_ids" {
  type = list(string)
}

variable "app_target_group_arns" {
  type = list(string)
}

variable "app_subnet_ids" {
  type = list(string)
}

variable "num_app_instances" {
  type = number
}
