// Provision an RDS database

/*
TODO:
  - configure security group & subnets for lambdas
  - create IAM role granting access to RDS (AmazonRDSFullAccess for now)
  - create IAM policy mapping database user to IAM role

LATER:
  - add custom security group to RDS instance

Resources:
- tf iam docs: https://www.terraform.io/docs/providers/aws/r/iam_role_policy_attachment.html
- https://www.terraform.io/docs/providers/postgresql/r/postgresql_role.html
- https://aws.amazon.com/premiumsupport/knowledge-center/users-connect-rds-iam/
*/

// TODO: replace this with a Lambda to rotate password from Secrets Manager
resource "random_password" "password" {
  length           = 16
  special          = true
  override_special = "_%@"
}

resource "aws_db_instance" "ww_prod_app" {
  allocated_storage    = 5
  storage_type         = "gp2"
  engine               = "postgres"
  engine_version       = "11.5"
  instance_class       = "db.t2.micro"
  identifier           = "ww-prod-app-db"
  name                 = "westrikworld_prod_app"
  username             = var.db_username
  password             = random_password.password.result
  parameter_group_name = "default.postgres11"

  skip_final_snapshot = true # TODO: remove and set final_snapshot_identifier

  db_subnet_group_name                = aws_db_subnet_group.default.name
  iam_database_authentication_enabled = true

  //  storage_encrypted = true
  //  kms_key_id = "KMS_ENCRYPTION_KEY_ARN"

  //  monitoring_role_arn = "IAM_ROLE_ARN"
  //  enabled_cloudwatch_logs_exports = ["postgresql", "upgrade"]

  //  deletion_protection = true

  //  maintenance_window = "Mon:00:00-Mon:03:00"
  //  backup_window      = "03:00-06:00"

  backup_retention_period = 0 # TODO: disable after testing

  tags = {
    Name        = "ww_prod_app_db_instance"
    Environment = "production"
  }
}

resource "aws_db_subnet_group" "default" {
  name       = "ww_prod_app_db_subnet_group"
  subnet_ids = var.app_subnets

  tags = {
    Name        = "ww_prod_app_db_subnet_group"
    Environment = "production"
  }
}

/*
To allow our app EC2 instances to communicate with RDS, we need to create a DB user with the `rds_iam` role.
Once we have the appropriate IAM policy in place to allow RDS<>EC2 communication, Postgres will use an IAM token
to auto-authenticate connections. This token will be rotated every 15 minutes, so there is no need to revoke it.
Using this approach lets us avoid having to store and rotate DB passwords on app hosts.

Since RDS is closed to outside connections, the easiest way to create the DB user is with a Lambda running in our VPC.
*/
resource "aws_lambda_function" "create_db_user_with_iam_role" {
  function_name = "create_db_user_with_iam_role"
  role          = aws_iam_role.lambda_create_db_user_with_iam_role.arn
  filename      = "./lambda/create_db_user_with_iam_role.zip"
  handler       = "create_db_user_with_iam_role.lambda_handler"
  runtime       = "python3.7"

  vpc_config {
    security_group_ids = var.app_security_groups
    subnet_ids         = var.app_subnets
  }
}

//data "aws_lambda_invocation" "create_db_user_with_iam_role" {
//  function_name = aws_lambda_function.create_db_user_with_iam_role.function_name
//  depends_on = aws_lambda_function.create_db_user_with_iam_role
//
//  // TODO: change host to the real one?
//  input = <<JSON
//{
//  "host": "${aws_db_instance.ww_prod_app.name}",
//  "port": "5432",
//  "database": "${aws_db_instance.ww_prod_app.name}",
//  "username": "${var.db_username}",
//  "password": "${random_password.password.result}"
//}
//JSON
//}

//output "lambda_result_create_db_user_with_iam_role" {
//  description = "Lambda result: create IAM DB user"
//  value       = data.aws_lambda_invocation.create_db_user_with_iam_role.result
//}

data "aws_iam_policy_document" "lambda_assume_roles" {
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

resource "aws_iam_role" "lambda_create_db_user_with_iam_role" {
  name               = "lambda_create_db_user_with_iam_role"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_roles.json
}

data "aws_iam_policy_document" "grant_rds_role" {
  statement {
    sid = "1"

    actions = [
      "rds:*",
    ]

    resources = [aws_db_instance.ww_prod_app.arn]
  }
}

resource "aws_iam_policy" "grant_rds_role" {
  policy = data.aws_iam_policy_document.grant_rds_role.json
}

resource "aws_iam_role_policy_attachment" "lambda_create_db_user_with_iam_role_rds" {
  role       = aws_iam_role.lambda_create_db_user_with_iam_role.name
  policy_arn = aws_iam_policy.grant_rds_role.arn
}

resource "aws_iam_role_policy_attachment" "role_attach_lambdavpc" {
  role       = aws_iam_role.lambda_create_db_user_with_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}

