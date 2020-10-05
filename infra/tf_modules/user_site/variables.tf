variable "aws_region" {
  type = string
}

variable "project_name" {
  type = string
}

variable "project_slug" {
  type = string
}

variable "deploy_name" {
  type = string
}

variable "root_domain_name" {
  type = string
}

variable "alias_domain_names" {
  type = list(string)
}
