module "ami_sandbox" {
  source = "./ami_sandbox"

  aws_az     = var.aws_az1
  aws_region = var.aws_region

  deploy_name = var.deploy_name
}

module "s3_buckets" {
  source = "./s3_buckets"

  aws_region   = var.aws_region
  project_slug = var.project_slug
  deploy_name  = var.deploy_name
}

module "deploy_lambdas" {
  source = "./deploy_lambdas"

  // OQ: should this use the same VPC/subnet as app instances?
  aws_region     = var.aws_region
  app_subnet_ids = module.network.app_subnet_ids
  vpc_id         = module.network.app_vpc_id
  provisioning   = var.provisioning_lambda
  deploy_name    = ""
}

module "network" {
  source       = "./network"
  aws_region   = var.aws_region
  aws_az1      = var.aws_az1
  aws_az2      = var.aws_az2
  project_name = var.project_name
  deploy_name  = ""
  project_slug = ""
}
