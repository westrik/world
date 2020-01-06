variable "db_username" {
  type    = string
  default = "westrikworld_user"
}

variable "db_default_password" {
  type    = string
  default = "6pwNab23u2wUg" # TODO: load at runtime
}

# output from module.api
variable "app_subnets" {
  type = list(string)
}
