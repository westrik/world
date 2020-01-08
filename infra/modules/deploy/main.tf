// Configure CodeDeploy and CodePipeline
/*
TODO:
- upload a release zip to S3 manually
- set up CodePipeline to pull from S3 and trigger CodeDeploy automatically
*/

provider "aws" {
  region = var.aws_region
}

/*
--------------------------------
CodeDeploy
--------------------------------
*/

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
// TODO: add ACL and lifecycle rule
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


/*
--------------------------------
CodePipeline
--------------------------------
*/

resource "aws_s3_bucket" "codepipeline_bucket" {
  bucket = "test-bucket"
  acl    = "private"
}

resource "aws_iam_role" "codepipeline_role" {
  name = "test-role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "codepipeline.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

resource "aws_iam_role_policy" "codepipeline_policy" {
  name = "codepipeline_policy"
  role = "${aws_iam_role.codepipeline_role.id}"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect":"Allow",
      "Action": [
        "s3:GetObject",
        "s3:GetObjectVersion",
        "s3:GetBucketVersioning",
        "s3:PutObject"
      ],
      "Resource": [
        "${aws_s3_bucket.codepipeline_bucket.arn}",
        "${aws_s3_bucket.codepipeline_bucket.arn}/*"
      ]
    },
    {
      "Effect": "Allow",
      "Action": [
        "codebuild:BatchGetBuilds",
        "codebuild:StartBuild"
      ],
      "Resource": "*"
    }
  ]
}
EOF
}

// TODO: do we need KMS?
data "aws_kms_alias" "s3kmskey" {
  name = "alias/myKmsKey"
}

// docs: https://www.terraform.io/docs/providers/aws/r/codepipeline.html
resource "aws_codepipeline" "codepipeline" {
  name     = "tf-test-pipeline"
  role_arn = "${aws_iam_role.codepipeline_role.arn}"

  // TODO: use CloudWatch S3 events for change detection (requires a CloudTrail and a CloudWatch Events rule)
  // https://docs.aws.amazon.com/codepipeline/latest/userguide/trigger-S3-migration-cwe.html
  // https://www.terraform.io/docs/providers/aws/r/cloudtrail.html
  // https://www.terraform.io/docs/providers/aws/r/cloudwatch_event_rule.html

  // TODO: why do we need an artifact store?
  artifact_store {
    location = "${aws_s3_bucket.codepipeline_bucket.bucket}"
    type     = "S3"

    encryption_key {
      id   = "${data.aws_kms_alias.s3kmskey.arn}"
      type = "KMS"
    }
  }

  stage {
    name = "Source"

    action {
      name             = "Source"
      category         = "Source"
      owner            = "ThirdParty"
      provider         = "GitHub" // TODO: change to S3
      version          = "1"
      output_artifacts = ["source_output"]

      configuration = {
        Owner  = "my-organization"
        Repo   = "test"
        Branch = "master"
      }
    }
  }

  // TODO: add Build stage that triggers CodeBuild

  stage {
    name = "Deploy"

    action {
      name            = "Deploy"
      category        = "Deploy"
      owner           = "AWS"
      provider        = "CloudFormation" // TODO: change to CodeDeploy
      input_artifacts = ["build_output"]
      version         = "1"

      configuration = {
        ActionMode     = "REPLACE_ON_FAILURE"
        Capabilities   = "CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM"
        OutputFileName = "CreateStackOutput.json"
        StackName      = "MyStack"
        TemplatePath   = "build_output::sam-templated.yaml"
      }
    }
  }
}
