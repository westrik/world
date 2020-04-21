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

module "ami_sandbox" {
  source = "./modules/ami_sandbox"

  aws_az     = var.aws_az1
  aws_region = var.aws_region
}

module "deploy_buckets" {
  source = "./modules/deploy_buckets"

  aws_region   = var.aws_region
  project_name = var.project_name
}

module "deploy_lambdas" {
  source = "./modules/deploy_lambdas"

  aws_region     = var.aws_region
  app_subnet_ids = module.network.app_subnet_ids
  vpc_id         = module.network.app_vpc_id
  provisioning   = var.provisioning_lambda
}

module "network" {
  source = "./modules/network"

  aws_region   = var.aws_region
  aws_az1      = var.aws_az1
  aws_az2      = var.aws_az2
  project_name = var.project_name
}

module "database" {
  source              = "./modules/database"
  app_subnet_ids      = module.network.app_subnet_ids
  app_security_groups = module.network.app_security_group_ids
  app_vpc_id          = module.network.app_vpc_id
  admin_user_arn      = data.aws_iam_user.admin_user.arn
}

module "app_load_balancer" {
  source = "./modules/app_load_balancer"

  aws_region       = var.aws_region
  aws_az1          = var.aws_az1
  aws_az2          = var.aws_az2
  project_name     = var.project_name
  root_domain_name = var.root_domain_name
  api_domain_name  = var.api_domain_name

  app_vpc_id             = module.network.app_vpc_id
  app_instance_ids       = module.app_instances.instance_ids
  app_security_group_ids = module.network.app_security_group_ids
  app_subnet_ids         = module.network.app_subnet_ids
}

module "app_instances" {
  source = "./modules/app_instances"

  aws_region   = var.aws_region
  project_name = var.project_name

  app_security_group_ids = module.network.app_security_group_ids
  app_subnet_ids         = module.network.app_subnet_ids
  num_app_instances      = var.num_app_instances
}

module "app_cloudfront" {
  source                               = "./modules/app_cloudfront"
  aws_region                           = var.aws_region
  root_domain_name                     = var.root_domain_name
  deploy_cloudfront_bucket_domain_name = module.deploy_buckets.app_deploy_cloudfront_bucket_domain_name
}

module "deploy_pipeline" {
  source           = "./modules/deploy_pipeline"
  aws_region       = var.aws_region
  root_domain_name = var.root_domain_name

  deploy_bucket                = module.deploy_buckets.app_deploy_bucket
  deploy_bucket_arn            = module.deploy_buckets.app_deploy_bucket_arn
  deploy_cloudfront_bucket     = module.deploy_buckets.app_deploy_cloudfront_bucket
  deploy_cloudfront_bucket_arn = module.deploy_buckets.app_deploy_cloudfront_bucket_arn
}

