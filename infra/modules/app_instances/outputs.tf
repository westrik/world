output "instance_ids" {
  value = aws_instance.app.*.id
}

