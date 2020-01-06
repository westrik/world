variable "db_username" {
  type    = string
  default = "westrikworld_user"
}

# output from module.api
variable "app_subnets" {
  type = list(string)
}
