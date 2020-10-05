resource "aws_lambda_function" "service_proxy" {
  function_name = "service_proxy"
  role          = aws_iam_role.lambda_service_proxy.arn
  s3_bucket     = var.lambda_deploy_bucket
  s3_key        = "service_proxy.zip"
  handler       = "app.lambda_handler"
  runtime       = "python3.8"
  timeout       = 60
}

resource "aws_secretsmanager_secret" "service_proxy_lambda_arn" {
  name                    = "${var.project_name}_service_proxy_lambda_arn"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "service_proxy_lambda_arn" {
  secret_id     = aws_secretsmanager_secret.service_proxy_lambda_arn.id
  secret_string = aws_lambda_function.service_proxy.arn
}

resource "aws_iam_role" "lambda_service_proxy" {
  name               = "lambda_service_proxy"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.lambda_service_proxy.json
}

data "aws_iam_policy_document" "lambda_service_proxy" {
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

resource "aws_iam_role_policy_attachment" "lambda_service_proxy_role_attach_exec_role" {
  role       = aws_iam_role.lambda_service_proxy.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}


resource "aws_iam_role_policy" "app_host_allow_invoke_lambda" {
  name = "app_host_allow_invoke_lambda"
  role = var.app_host_iam_role_id

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "lambda:InvokeFunction"
      ],
      "Effect": "Allow",
      "Resource": ["${aws_lambda_function.service_proxy.arn}"]
    }
  ]
}
EOF
}

