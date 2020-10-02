// Configure CodeDeploy and CodePipeline

provider "aws" {
  region = var.aws_region
}

/*
--------------------------------
CodeDeploy
--------------------------------
*/

resource "aws_codedeploy_app" "app" {
  name             = "${var.project_slug}_app"
  compute_platform = "Server"
}

resource "aws_codedeploy_deployment_group" "app_blue" {
  app_name               = aws_codedeploy_app.app.name
  deployment_group_name  = "${var.project_slug}_app_${var.deploy_name}-blue"
  service_role_arn       = aws_iam_role.codedeploy.arn
  deployment_config_name = "CodeDeployDefault.OneAtATime"
  autoscaling_groups = [
    var.app_blue_autoscaling_group_id,
  ]

  deployment_style {
    deployment_option = "WITH_TRAFFIC_CONTROL"
    deployment_type   = "IN_PLACE"
  }

  auto_rollback_configuration {
    enabled = true
    events = [
      "DEPLOYMENT_FAILURE",
    ]
  }

  load_balancer_info {
    target_group_info {
      name = var.app_target_group_name
    }
  }

  trigger_configuration {
    trigger_name       = "${var.project_slug}_app_${var.deploy_name}-trigger"
    trigger_events     = ["DeploymentFailure", "InstanceStart"]
    trigger_target_arn = var.codedeploy_app_scaling_sns_arn
  }

  ec2_tag_set {
    ec2_tag_filter {
      key   = "Environment"
      type  = "KEY_AND_VALUE"
      value = var.deploy_name
    }
  }
}

resource "aws_iam_access_key" "deploy_upload" {
  user = aws_iam_user.deploy_upload.name
}

resource "aws_iam_user" "deploy_upload" {
  name = "deploy_upload"
}

resource "aws_iam_user_policy" "deploy_upload" {
  name = "deploy_upload"
  user = aws_iam_user.deploy_upload.name

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:PutObject*"
      ],
      "Effect": "Allow",
      "Resource": ["${var.deploy_bucket_arn}/*"]
    }
  ]
}
EOF
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

resource "aws_iam_role" "codepipeline" {
  name = "codepipeline"

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

resource "aws_iam_role_policy" "codepipeline" {
  name = "codepipeline_policy"
  role = aws_iam_role.codepipeline.id

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
        "s3:PutObject*"
      ],
      "Resource": [
        "${var.deploy_bucket_arn}",
        "${var.deploy_bucket_arn}/*",
        "${var.deploy_cloudfront_bucket_arn}/*"
      ]
    },
    {
      "Effect": "Allow",
      "Action": [
        "codedeploy:*"
      ],
      "Resource": "*"
    }
  ]
}
EOF
}

resource "aws_codepipeline" "app" {
  name     = "${var.project_slug}_${var.deploy_name}"
  role_arn = aws_iam_role.codepipeline.arn

  // TODO: use CloudWatch S3 events for change detection (requires a CloudTrail and a CloudWatch Events rule)
  // https://docs.aws.amazon.com/codepipeline/latest/userguide/trigger-S3-migration-cwe.html
  // https://www.terraform.io/docs/providers/aws/r/cloudtrail.html
  // https://www.terraform.io/docs/providers/aws/r/cloudwatch_event_rule.html

  artifact_store {
    location = var.deploy_bucket
    type     = "S3"
  }

  stage {
    name = "Source_S3"

    action {
      run_order        = 1
      name             = "Source_S3"
      category         = "Source"
      owner            = "AWS"
      provider         = "S3"
      version          = "1"
      output_artifacts = ["westrikworld_app"]

      configuration = {
        S3Bucket             = var.deploy_bucket
        S3ObjectKey          = "westrikworld_app.zip"
        PollForSourceChanges = true # TODO: disable when setting up CloudWatch event triggers
        // TODO: add KMSEncryptionKeyARN
      }
    }
  }

  // TODO: change Source to pull from GitHub & add Build stage that triggers CodeBuild
  // TODO: add a Test stage

  stage {
    name = "Deploy_EC2"

    action {
      run_order       = 2
      name            = "Deploy_EC2"
      category        = "Deploy"
      owner           = "AWS"
      provider        = "CodeDeploy"
      version         = "1"
      input_artifacts = ["westrikworld_app"]

      configuration = {
        ApplicationName     = aws_codedeploy_app.app.name
        DeploymentGroupName = aws_codedeploy_deployment_group.app_blue.deployment_group_name
      }
    }
  }

  stage {
    name = "Deploy_CloudFront"

    action {
      run_order       = 3
      name            = "Deploy_CloudFront"
      category        = "Deploy"
      owner           = "AWS"
      provider        = "S3"
      version         = "1"
      input_artifacts = ["westrikworld_app"]

      configuration = {
        BucketName = var.deploy_cloudfront_bucket
        Extract    = true
        CannedACL  = "public-read"
      }
    }
  }
}
