variable "project_name" {
  type    = string
  default = "westrikworld"
}

variable "aws_region" {
  type = string
}

variable "app_deploy_hosts" {
  type = list(string)
}

variable "root_domain_name" {
  type = string
}
