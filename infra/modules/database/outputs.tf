output "lambda_result_create_db_user_with_iam_role" {
  description = "Lambda result: create IAM DB user"
  value       = data.aws_lambda_invocation.create_db_user_with_iam_role.result
  depends_on  = [aws_lambda_function.create_db_user_with_iam_role, aws_db_instance.app]
}
