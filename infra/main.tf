// Tested with Terraform v0.12.20 (as of 2020-04-07)

terraform {
  backend "s3" {
    bucket         = "westrikworld-tfstate"
    key            = "global/s3/terraform.tfstate"
    region         = "us-east-1"
    dynamodb_table = "westrikworld-tfstate-lock"
    encrypt        = true
  }
}

provider "aws" {
  region = var.aws_region
}

// TODO: module.admin for user management (e.g. for KMS)
data "aws_iam_user" "admin_user" {
  user_name = "mwestrik-mbp"
}

module "core_infra" {
  source = "./modules/core_infra"

  aws_region = var.aws_region
  aws_az1    = var.aws_az1
  aws_az2    = var.aws_az2

  project_name = var.project_name
  project_slug = var.project_slug
  deploy_name  = var.deploy_name
}

module "database" {
  source = "./modules/database"

  project_name                                 = var.project_name
  project_slug                                 = var.project_slug
  deploy_name                                  = var.deploy_name
  app_subnet_ids                               = module.core_infra.app_subnet_ids
  app_security_group_ids                       = module.core_infra.app_security_group_ids
  app_vpc_id                                   = module.core_infra.app_vpc_id
  admin_user_arn                               = data.aws_iam_user.admin_user.arn
  lambda_iam_role_arn__create_db_with_iam_role = module.core_infra.lambda_iam_role_arn__create_db_with_iam_role
  lambda_deploy_bucket                         = module.core_infra.lambda_deploy_bucket
}

module "content_buckets" {
  source = "./modules/content_buckets"

  aws_region = var.aws_region

  project_slug         = var.project_slug
  deploy_name          = var.deploy_name
  app_host_iam_role_id = module.app_instances.app_host_iam_role_id
}

module "deploy_pipeline" {
  source = "./modules/deploy_pipeline"

  aws_region = var.aws_region

  project_slug                 = var.project_slug
  deploy_name                  = var.deploy_name
  deploy_bucket                = module.core_infra.app_deploy_bucket
  deploy_bucket_arn            = module.core_infra.app_deploy_bucket_arn
  deploy_cloudfront_bucket     = module.core_infra.app_deploy_cloudfront_bucket
  deploy_cloudfront_bucket_arn = module.core_infra.app_deploy_cloudfront_bucket_arn

  app_lb_listener_arn        = module.app_load_balancer.app_lb_listener_arn
  app_autoscaling_group_ids  = module.app_instances.app_autoscaling_group_ids
  app_blue_target_group_name = module.app_load_balancer.app_blue_target_group_name
}

module "app_cloudfront" {
  source = "./modules/app_cloudfront"

  aws_region = var.aws_region

  project_name                         = var.project_name
  deploy_name                          = var.deploy_name
  root_domain_name                     = var.root_domain_name
  deploy_cloudfront_bucket_domain_name = module.core_infra.app_deploy_cloudfront_bucket_domain_name
}

module "app_load_balancer" {
  source = "./modules/app_load_balancer"

  aws_region = var.aws_region
  aws_az1    = var.aws_az1
  aws_az2    = var.aws_az2

  project_name     = var.project_name
  project_slug     = var.project_slug
  deploy_name      = var.deploy_name
  root_domain_name = var.root_domain_name
  api_domain_name  = var.api_domain_name
  admin_email      = var.admin_email

  app_vpc_id             = module.core_infra.app_vpc_id
  app_security_group_ids = module.core_infra.app_security_group_ids
  app_subnet_ids         = module.core_infra.app_subnet_ids

  lambda_deploy_bucket                   = module.core_infra.lambda_deploy_bucket
  lambda_iam_role_arn__renew_certificate = module.core_infra.lambda_iam_role_arn__renew_certificate
}

module "app_instances" {
  source = "./modules/app_instances"

  aws_region = var.aws_region
  aws_az1    = var.aws_az1
  aws_az2    = var.aws_az2

  project_name = var.project_name
  deploy_name  = var.deploy_name

  app_security_group_ids    = module.core_infra.app_security_group_ids
  app_subnet_ids            = module.core_infra.app_subnet_ids
  num_app_instances         = var.num_app_instances
  app_blue_target_group_arn = module.app_load_balancer.app_blue_target_group_arn
}


module "secrets" {
  source = "./modules/secrets"

  project_name          = var.project_name
  sendgrid_api_key      = var.sendgrid_api_key
  outbound_email_sender = var.outbound_email_sender
  root_domain_name      = var.root_domain_name
}
