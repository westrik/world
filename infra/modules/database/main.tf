// Provision an RDS database

// TODO: replace this with a Lambda to rotate password from Secrets Manager
resource "random_password" "password" {
  length           = 16
  special          = true
  override_special = "_%"
}

resource "aws_kms_key" "app_db" {
  description             = "app database encryption key"
  deletion_window_in_days = 7
  enable_key_rotation     = false # TODO: enable
  policy                  = data.aws_iam_policy_document.rds_access_to_kms.json
}

data "aws_iam_policy_document" "rds_access_to_kms" {
  statement {
    sid = "1"

    effect = "Allow"

    principals {
      type        = "AWS"
      identifiers = [var.admin_user_arn]
    }

    resources = ["*"]

    actions = [
      "kms:Create*",
      "kms:Describe*",
      "kms:Enable*",
      "kms:List*",
      "kms:Put*",
      "kms:Update*",
      "kms:Revoke*",
      "kms:Disable*",
      "kms:Get*",
      "kms:Delete*",
      "kms:ScheduleKeyDeletion",
      "kms:CancelKeyDeletion"
    ]
  }
  statement {
    sid = "2"

    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["rds.amazonaws.com"]
    }

    actions = [
      "kms:Encrypt",
      "kms:Decrypt",
      "kms:ReEncrypt*",
      "kms:GenerateDataKey*",
      "kms:CreateGrant",
      "kms:ListGrants",
      "kms:DescribeKey"
    ]
  }
}

locals {
  db_instance_class       = "db.t3.micro"
  db_engine               = "postgres"
  db_engine_version       = "12.3"
  db_option_group_name    = "default:postgres-11"
  db_parameter_group_name = "default.postgres11"


  db_allocated_storage  = 5
  db_storage_type       = "gp2"
  db_ca_cert_identifier = "rds-ca-2019"
}


resource "aws_db_instance" "app" {
  instance_class = local.db_instance_class
  engine         = local.db_engine
  engine_version = local.db_engine_version
  //  option_group_name    = local.db_option_group_name
  //  parameter_group_name = local.db_parameter_group_name
  //  allow_major_version_upgrade = false
  apply_immediately = false

  identifier = "${var.project_slug}-app"
  name       = "${var.project_slug}_app"

  allocated_storage = local.db_allocated_storage
  kms_key_id        = aws_kms_key.app_db.arn
  storage_type      = local.db_storage_type
  storage_encrypted = true

  username = var.db_username
  password = random_password.password.result

  ca_cert_identifier                  = local.db_ca_cert_identifier
  db_subnet_group_name                = aws_db_subnet_group.app.name
  iam_database_authentication_enabled = true
  vpc_security_group_ids              = [aws_security_group.app_db.id]

  //  replicate_source_db = aws_db_instance.app.identifier

  enabled_cloudwatch_logs_exports = ["postgresql", "upgrade"]
  //  monitoring_role_arn = "IAM_ROLE_ARN"

  backup_window           = "03:00-06:00"
  backup_retention_period = 0 # TODO: disable after testing
  //  deletion_protection = true
  maintenance_window  = "Mon:00:00-Mon:03:00"
  skip_final_snapshot = true # TODO: replace w/ final_snapshot_identifier

  tags = {
    Name        = "${var.project_slug}_app_db-primary"
    Environment = var.deploy_name
    Project     = var.project_name
  }
}

resource "aws_db_subnet_group" "app" {
  name       = "app_db_subnet_group"
  subnet_ids = var.app_subnet_ids

  tags = {
    Name        = "app_db_subnet_group"
    Environment = var.deploy_name
    Project     = var.project_name
  }
}

resource "aws_security_group" "app_db" {
  name        = "app_db_sg"
  description = "${var.project_slug}_app_db"
  vpc_id      = var.app_vpc_id

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
}

resource "aws_secretsmanager_secret" "db_url" {
  name                    = "${var.project_slug}_database_url"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_url" {
  secret_id     = aws_secretsmanager_secret.db_url.id
  secret_string = aws_db_instance.app.address
}

resource "aws_secretsmanager_secret" "db_user" {
  name                    = "${var.project_slug}_database_username"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_user" {
  secret_id     = aws_secretsmanager_secret.db_user.id
  secret_string = aws_db_instance.app.username
}

resource "aws_secretsmanager_secret" "db_name" {
  name                    = "${var.project_slug}_database_name"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_name" {
  secret_id     = aws_secretsmanager_secret.db_name.id
  secret_string = aws_db_instance.app.name
}

resource "aws_secretsmanager_secret" "db_password" {
  name                    = "${var.project_slug}_database_password"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_password" {
  secret_id     = aws_secretsmanager_secret.db_password.id
  secret_string = random_password.password.result
}

resource "aws_secretsmanager_secret" "password_salt" {
  name                    = "${var.project_slug}_password_hash_salt"
  recovery_window_in_days = 0
}
resource "random_string" "password_salt" {
  length           = 32
  special          = true
  override_special = "_%"
}
resource "aws_secretsmanager_secret_version" "password_salt" {
  secret_id     = aws_secretsmanager_secret.password_salt.id
  secret_string = random_string.password_salt.result
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
  role          = var.lambda_iam_role_arn__create_db_with_iam_role
  s3_bucket     = var.lambda_deploy_bucket
  s3_key        = "create_db_user_with_iam_role.zip"
  handler       = "app.lambda_handler"
  runtime       = "python3.8"

  vpc_config {
    security_group_ids = var.app_security_group_ids
    subnet_ids         = var.app_subnet_ids
  }
}

data "aws_lambda_invocation" "create_db_user_with_iam_role" {
  function_name = "create_db_user_with_iam_role"
  depends_on    = [aws_lambda_function.create_db_user_with_iam_role]

  input = <<JSON
{
  "host": "${aws_db_instance.app.address}",
  "port": "${aws_db_instance.app.port}",
  "database": "${aws_db_instance.app.name}",
  "username": "${var.db_username}",
  "password": "${random_password.password.result}"
}
JSON
}

# TODO/OQ: will we need a lambda invokation for the replica as well?
