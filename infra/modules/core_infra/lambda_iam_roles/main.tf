provider "aws" {
  region = var.aws_region
}

/**
----------------------------------------------------
IAM role for Lambda that renews the TLS certificate
----------------------------------------------------
*/

resource "aws_iam_role" "lambda_renew_certificate" {
  name               = "lambda_renew_certificate"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.lambda_renew_certificate.json
}

data "aws_iam_policy_document" "lambda_renew_certificate" {
  statement {
    sid = "1"

    actions = [
      "sts:AssumeRole",
    ]

    principals {
      identifiers = ["lambda.amazonaws.com"]
      type        = "Service"
    }
  }
}


resource "aws_iam_role_policy_attachment" "lambda_renew_certificate_role_attach_lambda_vpc" {
  role       = aws_iam_role.lambda_renew_certificate.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}

resource "aws_iam_role_policy_attachment" "lambda_renew_certificate_role_attach_secrets_manager" {
  role       = aws_iam_role.lambda_renew_certificate.name
  policy_arn = "arn:aws:iam::aws:policy/SecretsManagerReadWrite"
}


/**
----------------------------------------------------
IAM role for Lambda that creates RDS DB user for IAM
----------------------------------------------------
*/

resource "aws_iam_role" "lambda_create_db_user_with_iam_role" {
  name               = "lambda_create_db_user_with_iam_role"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.lambda_create_db_user_with_iam_role.json
}

data "aws_iam_policy_document" "lambda_create_db_user_with_iam_role" {
  statement {
    sid = "1"

    actions = [
      "sts:AssumeRole",
    ]

    principals {
      identifiers = ["lambda.amazonaws.com"]
      type        = "Service"
    }
  }
}

resource "aws_iam_role_policy_attachment" "lambda_create_db_user_with_iam_role_role_attach_lambda_vpc" {
  role       = aws_iam_role.lambda_create_db_user_with_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}

resource "aws_iam_role_policy_attachment" "lambda_create_db_user_with_iam_role_role_attach_lambda_rds" {
  role       = aws_iam_role.lambda_create_db_user_with_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonRDSFullAccess"
}
