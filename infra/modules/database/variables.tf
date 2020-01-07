variable "db_username" {
  type    = string
  default = "westrikworld_admin"
}

# output from module.api
variable "app_subnets" {
  type = list(string)
}

# output from module.api
variable "app_security_groups" {
  type = list(string)
}
