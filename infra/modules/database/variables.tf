variable "db_username" {
  type    = string
  default = "westrikworld_user"
}

variable "db_password" {
  type    = string
  default = "6p%wNab23u@2@w_:!IUg" # TODO: don't put this here
}

# output from module.api
variable "app_subnet" {
  type = string
}
