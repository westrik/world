// Tested with Terraform v0.12.18 (as of 2020-01-05)

provider "aws" {
  region = var.aws_region
}

module "build_resources" {
  source = "./modules/build_resources"

  aws_az     = var.aws_az
  aws_region = var.aws_region
}

module "api" {
  source = "./modules/api"

  aws_az           = var.aws_az
  aws_region       = var.aws_region
  api_domain_name  = var.api_domain_name
  root_domain_name = var.root_domain_name
}

module "database" {
  source     = "./modules/database"
  app_subnet = module.api.app_subnet
}
