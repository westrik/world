// Provision an RDS database

/*
TODO:
  - set up user on RDS PG that authenticates with IAM token (probably with a Lambda?)
      ```
      CREATE USER {dbusername};
      GRANT rds_iam to {dbusername};
      ```
  - create IAM role with AmazonRDSFullAccess (or create new IAM policy with fewer privileges)
  - create IAM policy mapping database user to IAM role
  - attach IAM role to EC2 instance(s)

LATER:
  - add custom security group to RDS instance

Resources:
- https://www.terraform.io/docs/providers/postgresql/r/postgresql_role.html
- https://aws.amazon.com/premiumsupport/knowledge-center/users-connect-rds-iam/
- https://github.com/assembl/assembl/blob/531123115bb12a2dbb090b9910d375a67905d775/assembl/scripts/lambda_create_db_aws_user.py
*/

resource "aws_db_instance" "ww_prod_app" {
  allocated_storage    = 5
  storage_type         = "gp2"
  engine               = "postgres"
  engine_version       = "11.5"
  instance_class       = "db.t2.micro"
  identifier           = "ww_prod_app_db"
  name                 = "westrikworld_prod_app"
  username             = var.db_username
  password             = var.db_password
  parameter_group_name = "default.postgres11"

  db_subnet_group_name = aws_db_subnet_group.default.name
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
