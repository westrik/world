// Configure CodeDeploy and CodePipeline
/*
TODO:
- upload a release zip to S3 manually
- set up CodePipeline to pull from S3 and trigger CodeDeploy automatically
*/

provider "aws" {
  region = var.aws_region
}

resource "aws_codedeploy_app" "app" {
  name = "${var.project_name}_app"
}

resource "aws_codedeploy_deployment_group" "app" {
  app_name              = aws_codedeploy_app.app.name
  deployment_group_name = "${var.project_name}_app"
  service_role_arn      = aws_iam_role.codedeploy.arn

  deployment_config_name = "CodeDeployDefault.OneAtATime"

  ec2_tag_filter {
    key   = "Environment"
    type  = "KEY_AND_VALUE"
    value = "production"
  }

  auto_rollback_configuration {
    enabled = true
    events = [
      "DEPLOYMENT_FAILURE",
    ]
  }
}

resource "random_string" "deploy_bucket_hash" {
  length  = 6
  special = false
  upper = false
}
resource "aws_s3_bucket" "app_deploy" {
  bucket = "${var.project_name}-deploy-${random_string.deploy_bucket_hash.result}"
  acl    = "private"
}

resource "aws_iam_role" "codedeploy" {
  name               = "codedeploy"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.codedeploy.json
}

resource "aws_iam_role_policy_attachment" "codedeploy" {
  role       = aws_iam_role.codedeploy.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSCodeDeployRole"
}

data "aws_iam_policy_document" "codedeploy" {
  statement {
    sid = "1"

    actions = [
      "sts:AssumeRole",
    ]

    principals {
      identifiers = ["codedeploy.amazonaws.com"]
      type        = "Service"
    }
  }
}
