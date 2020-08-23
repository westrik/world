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

/* -----------------------------------------------------------------------------
 changes to defaults above this line must be synced with core_infra/variables.tf
 TODO: refactor this to avoid duplication
 */

variable "api_domain_name" {
  default = "api.westrik.world"
}

variable "root_domain_name" {
  default = "westrik.world"
}

variable "admin_email" {
  default = "m+wwadmin@ttwestrik.com"
}

variable "outbound_email_sender" {
  default = "admin@westrik.world"
}

variable "num_app_instances" {
  type    = number
  default = 2
}

// SECRETS
variable "sendgrid_api_key" {
  type = string
}

variable "cloudfront_keypair_id" {
  type = string
}

variable "cloudfront_private_key" {
  type = string
}
