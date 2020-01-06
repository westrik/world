variable "db_username" {
  type    = string
  default = "westrikworld_user"
}

variable "db_password" {
  type    = string
  default = "6pwNab23u2wUg" # TODO: don't put this here
}

# output from module.api
variable "app_subnets" {
  type = list(string)
}
