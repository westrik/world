// Provision an RDS database

resource "aws_db_subnet_group" "default" {
  name       = "ww_prod_app_db_subnet_group"
  subnet_ids = var.app_subnets

  tags = {
    Name        = "ww_prod_app_db_subnet_group"
    Environment = "production"
  }
}

resource "aws_db_instance" "ww_prod_app" {
  allocated_storage    = 20
  storage_type         = "gp2"
  engine               = "postgres"
  engine_version       = "11.5"
  instance_class       = "db.t2.micro"
  name                 = "westrikworld_prod_app"
  username             = var.db_username
  password             = var.db_password
  parameter_group_name = "default.postgres11"

  db_subnet_group_name = aws_db_subnet_group.default.name
  iam_database_authentication_enabled = true

  //  storage_encrypted = true
  //  kms_key_id = "KMS_ENCRYPTION_KEY_ARN"


  //  monitoring_role_arn = "IAM_ROLE_ARN"

  //  deletion_protection = true

  tags = {
    Name        = "ww_prod_app_db_instance"
    Environment = "production"
  }
}

// PROBLEM:
// - i've configured RDS instance to not allow public connections, which makes sense
// - however, then I can't connect to configure the IAM DB role...
// SOLUTION A: find a way to specify automatic IAM role setup for RDS
// SOLUTION B: have to run terraform on an instance in the VPC?

provider "postgresql" {
  host            = aws_db_instance.ww_prod_app.address
  port            = 5432
  database        = "westrikworld_prod_app"
  username        = var.db_username
  password        = var.db_password
  superuser       = false
  sslmode         = "verify-full"
  connect_timeout = 15
}

resource "postgresql_database" "ww_prod_app" {
  name              = "westrikworld_prod_app"
  owner             = var.db_username
  allow_connections = true
}

resource "postgresql_role" "ww_prod_app_db_iam_user" {
  name     = "iam_user"
}

resource "postgresql_grant" "w" {
  database    = postgresql_database.ww_prod_app.name
  role        = postgresql_role.ww_prod_app_db_iam_user.name
  schema      = "public"
  object_type = "table"
  privileges  = ["rds_iam"]
}

/*
TODO:
  - set up user on RDS PG that authenticates with IAM token
      ```
      CREATE USER {dbusername};
      GRANT rds_iam to {dbusername};
      ```
  - create IAM role with AmazonRDSFullAccess (or create new IAM policy with fewer privileges)
  - create IAM policy mapping database user to IAM role
  - attach IAM role to EC2 instance(s)

  - add custom security group to RDS instance

Resources:
- https://www.terraform.io/docs/providers/postgresql/r/postgresql_role.html
- https://aws.amazon.com/premiumsupport/knowledge-center/users-connect-rds-iam/
*/



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
