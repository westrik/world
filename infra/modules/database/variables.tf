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
variable "app_security_groups" {
  type = list(string)
}

# output from module.network
variable "app_vpc_id" {
  type = string
}
