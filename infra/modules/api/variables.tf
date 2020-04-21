variable "project_name" {
  type = string
}

variable "admin_email" {
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

variable "vpc_id" {
  type = string
}

variable "app_security_group_ids" {
  type = list(string)
}

variable "app_subnet_ids" {
  type = list(string)
}

variable "num_app_instances" {
  type    = number
  default = 1
}

