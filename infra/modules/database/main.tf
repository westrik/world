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

data "aws_iam_user" "admin_user" {
  user_name = "mwestrik-mbp"
}

data "aws_iam_policy_document" "rds_access_to_kms" {
  statement {
    sid = "1"

    effect = "Allow"

    principals {
      type        = "AWS"
      identifiers = [data.aws_iam_user.admin_user.arn]
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


resource "aws_db_instance" "app" {
  engine               = "postgres"
  engine_version       = "11.5"
  instance_class       = "db.t3.micro"
  parameter_group_name = "default.postgres11"

  identifier = "${var.project_name}-app"
  name       = "${var.project_name}_app"

  username = var.db_username
  password = random_password.password.result

  allocated_storage  = 5
  storage_type       = "gp2"
  storage_encrypted  = true
  kms_key_id         = aws_kms_key.app_db.arn
  ca_cert_identifier = "rds-ca-2019"

  skip_final_snapshot = true # TODO: remove and set final_snapshot_identifier

  db_subnet_group_name   = aws_db_subnet_group.app.name
  vpc_security_group_ids = [aws_security_group.app.id]

  iam_database_authentication_enabled = true

  // TODO: use as read replica for DB in us-west-1
  //  replicate_source_db = ""

  enabled_cloudwatch_logs_exports = ["postgresql", "upgrade"]
  //  monitoring_role_arn = "IAM_ROLE_ARN"
  //  deletion_protection = true

  maintenance_window = "Mon:00:00-Mon:03:00"
  backup_window      = "03:00-06:00"

  backup_retention_period = 0 # TODO: disable after testing

  tags = {
    Name        = "app_db"
    Environment = "production"
    Project     = var.project_name
  }
}

resource "aws_db_subnet_group" "app" {
  name       = "app_db_subnet_group"
  subnet_ids = var.app_subnets

  tags = {
    Name        = "app_db_subnet_group"
    Environment = "production"
  }
}

resource "aws_security_group" "app" {
  name        = "app_db_sg"
  description = "${var.project_name}_app_db"
  vpc_id      = var.app_vpc

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
}

resource "aws_secretsmanager_secret" "db_url" {
  name                    = "westrikworld_database_url"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_url" {
  secret_id     = aws_secretsmanager_secret.db_url.id
  secret_string = aws_db_instance.app.address
}

resource "aws_secretsmanager_secret" "db_user" {
  name                    = "westrikworld_database_username"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_user" {
  secret_id     = aws_secretsmanager_secret.db_user.id
  secret_string = aws_db_instance.app.username
}

resource "aws_secretsmanager_secret" "db_name" {
  name                    = "westrikworld_database_name"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_name" {
  secret_id     = aws_secretsmanager_secret.db_name.id
  secret_string = aws_db_instance.app.name
}

resource "aws_secretsmanager_secret" "db_password" {
  name                    = "westrikworld_database_password"
  recovery_window_in_days = 0
}
resource "aws_secretsmanager_secret_version" "db_password" {
  secret_id     = aws_secretsmanager_secret.db_password.id
  secret_string = random_password.password.result
}

resource "aws_secretsmanager_secret" "password_salt" {
  name                    = "westrikworld_password_hash_salt"
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
  role          = aws_iam_role.lambda_create_db_user_with_iam_role.arn
  filename      = "./lambda/create_db_user_with_iam_role.zip"
  handler       = "create_db_user_with_iam_role.lambda_handler"
  runtime       = "python3.7"

  vpc_config {
    security_group_ids = var.app_security_groups
    subnet_ids         = var.app_subnets
  }
}

data "aws_lambda_invocation" "create_db_user_with_iam_role" {
  function_name = aws_lambda_function.create_db_user_with_iam_role.function_name
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

resource "aws_iam_role_policy_attachment" "role_attach_lambda_vpc" {
  role       = aws_iam_role.lambda_create_db_user_with_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}

resource "aws_iam_role_policy_attachment" "role_attach_lambda_rds" {
  role       = aws_iam_role.lambda_create_db_user_with_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonRDSFullAccess"
}
