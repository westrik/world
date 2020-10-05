output "lambda_iam_role_arn__create_db_with_iam_role" {
  value = aws_iam_role.lambda_create_db_user_with_iam_role.arn
}

output "lambda_iam_role_arn__renew_certificate" {
  value = aws_iam_role.lambda_renew_certificate.arn
}
