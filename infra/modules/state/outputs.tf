output "tfstate_bucket" {
  description = "Name of encrypted S3 bucket for terraform state"
  value       = aws_s3_bucket.tfstate.bucket
}

output "tfstate_lock_table" {
  description = "Name of DynamoDB table for locking terraform state"
  value       = aws_dynamodb_table.tfstate_lock.name
}
