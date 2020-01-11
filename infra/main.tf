// Tested with Terraform v0.12.18 (as of 2020-01-05)

provider "aws" {
  region = var.aws_region
}

module "build_resources" {
  source = "./modules/build_resources"

  aws_az     = var.aws_az1
  aws_region = var.aws_region
}

// TODO: create module.network and move VPC, SG, etc. config from module.api

module "api" {
  source = "./modules/api"

  aws_az1          = var.aws_az1
  aws_az2          = var.aws_az2
  aws_region       = var.aws_region
  api_domain_name  = var.api_domain_name
  root_domain_name = var.root_domain_name
}

module "database" {
  source              = "./modules/database"
  app_subnets         = module.api.app_subnets
  app_security_groups = module.api.app_security_groups
  app_vpc             = module.api.app_vpc
}

module "deploy" {
  source           = "./modules/deploy"
  app_deploy_hosts = module.api.app_deploy_hosts
  aws_region       = var.aws_region
  root_domain_name = var.root_domain_name
}
