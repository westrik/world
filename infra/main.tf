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

module "build_resources" {
  source = "./modules/build_resources"

  aws_az     = var.aws_az1
  aws_region = var.aws_region
}

module "deploy_buckets" {
  source = "./modules/deploy_buckets"

  aws_region = var.aws_region
}

//module "deploy_lambdas" {
//  source = "./modules/deploy_lambdas"
//
//  aws_region = var.aws_region
//  app_subnet_ids = module.network.app_subnet_ids
//  vpc_id = module.network.app_vpc_id
//}

module "network" {
  source = "./modules/network"

  aws_region   = var.aws_region
  aws_az1      = var.aws_az1
  aws_az2      = var.aws_az2
  project_name = var.project_name
}

module "api" {
  source = "./modules/api"

  aws_az1          = var.aws_az1
  aws_az2          = var.aws_az2
  aws_region       = var.aws_region
  api_domain_name  = var.api_domain_name
  root_domain_name = var.root_domain_name
  admin_email      = var.admin_email
  project_name     = var.project_name

  instance_security_group_ids = module.network.instance_security_group_ids
  app_subnet_ids              = module.network.app_subnet_ids
  vpc_id                      = module.network.app_vpc_id
}

module "database" {
  source              = "./modules/database"
  app_subnet_ids      = module.network.app_subnet_ids
  app_security_groups = module.network.app_security_groups
  app_vpc_id          = module.network.app_vpc_id
}

module "deploy" {
  source           = "./modules/deploy"
  app_deploy_hosts = module.api.app_deploy_hosts
  aws_region       = var.aws_region
  root_domain_name = var.root_domain_name
}
