variable "project_name" {
  type = string
}

variable "deploy_name" {
  type = string
}

variable "ami_id" {
  type = string
}

variable "app_security_group_ids" {
  type = list(string)
}

variable "consul_security_group_ids" {
  type = list(string)
}

variable "nomad_security_group_ids" {
  type = list(string)
}

variable "app_subnet_ids" {
  type = list(string)
}

variable "num_app_instances" {
  type = number
}

variable "target_group_arn" {
  type = string
}

variable "iam_instance_profile_name" {
  type = string
}

variable "color" {
  type = string
}
