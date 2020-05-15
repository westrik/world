# defaults in this file are duplicated from ../../variables.tf
# TODO: refactor this to avoid duplication

variable "project_name" {
  default = "westrikworld"
}

variable "project_slug" {
  default = "westrikworld"
}

variable "deploy_name" {
  default = "production"
}

variable "aws_region" {
  default = "us-east-1"
}

variable "aws_az1" {
  default = "us-east-1a"
}

variable "aws_az2" {
  default = "us-east-1b"
}