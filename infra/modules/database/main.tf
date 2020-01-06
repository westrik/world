// Provision an RDS database

/*
TODO:
  - create IAM policy mapping database user to IAM role
  - attach IAM role to EC2 instance(s)

LATER:
  - add custom security group to RDS instance

Resources:
- https://www.terraform.io/docs/providers/postgresql/r/postgresql_role.html
- https://aws.amazon.com/premiumsupport/knowledge-center/users-connect-rds-iam/
- https://github.com/assembl/assembl/blob/531123115bb12a2dbb090b9910d375a67905d775/assembl/scripts/lambda_create_db_aws_user.py
*/

resource "random_password" "password" {
  length = 16
  special = true
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

// TODO: create IAM role granting access to RDS (AmazonRDSFullAccess for now)
// TODO: configure security group & subnets for lambdas

// TODO: set up lambda handler
//resource "aws_lambda_function" "ww_prod_app_lambda_rotate_db_password" {
//  function_name = "rotate_db_password"
//  handler = ""
//  role = ""
//  runtime = "python3.8"
//  vpc_config {
//    security_group_ids = []
//    subnet_ids = []
//  }
//}
//

// TODO: use terraform to invoke the DB rotate lambda (providing the random password as env var)
// see https://www.terraform.io/docs/providers/aws/d/lambda_invocation.html

/*
TODO: set up lambda handler
  - set up user on RDS PG that authenticates with IAM token (probably with a Lambda?)
  ```
  CREATE USER {dbusername};
  GRANT rds_iam to {dbusername};
  ```
  - tf lambda docs: https://www.terraform.io/docs/providers/aws/r/lambda_function.html
  - tf iam docs: https://www.terraform.io/docs/providers/aws/r/iam_role_policy_attachment.html
*/
//resource "aws_lambda_function" "ww_prod_app_lambda_create_db_user_with_iam_role" {
//  function_name = "create_db_user_with_iam_role"
//  handler = ""
//  role = ""
//  runtime = "python3.8"
//  vpc_config {
//    security_group_ids = []
//    subnet_ids = []
//  }
//}
//
//resource "aws_secretsmanager_secret" "ww_prod_app_db_password" {
//  name             = "ww_prod_app_db_password"
//  description      = "Password for root RDS account"
//
//  rotation_rules {
//    automatically_after_days = 7
//  }
//  rotation_lambda_arn = aws_lambda_function.ww_prod_app_lambda_rotate_db_password.arn
//}
//

//data "aws_iam_policy_document" "example" {
//  statement {
//    sid = "1"
//
//    actions = [
//      "s3:ListAllMyBuckets",
//      "s3:GetBucketLocation",
//    ]
//
//    resources = [
//      "arn:aws:s3:::*",
//    ]
//  }
//
//  statement {
//    actions = [
//      "s3:ListBucket",
//    ]
//
//    resources = [
//      "arn:aws:s3:::${var.s3_bucket_name}",
//    ]
//
//    condition {
//      test     = "StringLike"
//      variable = "s3:prefix"
//
//      values = [
//        "",
//        "home/",
//        "home/&{aws:username}/",
//      ]
//    }
//  }
//
//  statement {
//    actions = [
//      "s3:*",
//    ]
//
//    resources = [
//      "arn:aws:s3:::${var.s3_bucket_name}/home/&{aws:username}",
//      "arn:aws:s3:::${var.s3_bucket_name}/home/&{aws:username}/*",
//    ]
//  }
//}
//
//resource "aws_iam_policy" "example" {
//  name   = "example_policy"
//  path   = "/"
//  policy = "${data.aws_iam_policy_document.example.json}"
//}
