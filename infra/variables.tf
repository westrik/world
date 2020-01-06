variable "aws_region" {
  default = "us-east-1"
}

variable "aws_az" {
  default = "us-east-1a"
}

variable "api_domain_name" {
  default = "api.westrikworld.com"
}

variable "frontend_domain_name" {
  default = "westrikworld.com"
}

# Set to "true" to provision everything except instances that rely on AMIs that haven't been built yet
variable "no_ami" {
  type    = bool
  default = false
}
