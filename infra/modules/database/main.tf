// Provision an RDS database

resource "aws_db_subnet_group" "default" {
  name       = "ww_prod_app_db_subnet_group"
  subnet_ids = var.app_subnets

  tags = {
    Name        = "ww_prod_app_db_subnet_group"
    Environment = "production"
  }
}

/*
TODO:
  - add subnets covering two AZs to DB Subnet Group
*/

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

  //  storage_encrypted = true
  //  kms_key_id = "KMS_ENCRYPTION_KEY_ARN"

  //  iam_database_authentication_enabled = true

  //  monitoring_role_arn = "IAM_ROLE_ARN"

  //  deletion_protection = true

  tags = {
    Name        = "ww_prod_app_db_instance"
    Environment = "production"
  }
}
