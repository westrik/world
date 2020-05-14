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

module "network" {
  source       = "./network"
  aws_region   = var.aws_region
  aws_az1      = var.aws_az1
  aws_az2      = var.aws_az2
  project_name = var.project_name
  deploy_name  = ""
  project_slug = ""
}

module "lambda_iam_roles" {
  source       = "./lambda_iam_roles"
  aws_region   = var.aws_region
}
