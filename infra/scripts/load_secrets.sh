#!/usr/bin/env bash

set -euo pipefail

APPLICATION_NAME="westrikworld"
RAMFS_MOUNT_DIR=/secrets
RAMFS_SIZE=64M

mkdir -p $RAMFS_MOUNT_DIR
if ! mountpoint -q -- $RAMFS_MOUNT_DIR; then
  mount -t tmpfs -o size=$RAMFS_SIZE,mode=0700,uid=1000,gid=1 ramfs $RAMFS_MOUNT_DIR
else
  # TODO: don't delete everything, but check for changes and selectively reload
  #     - then, this can be run e.g. every 15 seconds
  #     - then, we can add a thing to pull the RDS creds via IAM
  # shellcheck disable=SC2115
  rm -rf "${RAMFS_MOUNT_DIR}/*"
fi

# TODO: pass this in as variable from ansible (or does us-east-1 work from everywhere?)
aws configure set region us-east-1

function get_secret() {
  secret_id=$1
  secret_value=$(aws secretsmanager get-secret-value --secret-id "$APPLICATION_NAME""_""$secret_id" | jq -r '.SecretString')
  # TODO: escape all potential problematic values
  # secret_value="${secret_value//\%/%%}"
  echo "$secret_value"
}

api_cert_data=$(get_secret "api_cert" | jq -r '.certificate')
content_bucket_name=$(get_secret "content_bucket_name")
cors_origin_url=$(get_secret "cors_origin_url")
iam_role_arn=$(get_secret "ec2_app_host_role_arn")
outbound_email_sender=$(get_secret "outbound_email_sender")
password_hash_salt=$(get_secret "password_hash_salt")
rds_db_name=$(get_secret "database_name")
rds_host=$(get_secret "database_url")
rds_password=$(get_secret "database_password")
rds_user=$(get_secret "database_username")
sendgrid_api_key=$(get_secret "sendgrid_api_key")
service_proxy_lambda_arn=$(get_secret "service_proxy_lambda_arn")

systemctl stop nginx app worker

{
  echo "CONTENT_BUCKET_NAME=$content_bucket_name"
  echo "CORS_ORIGIN_URL=$cors_origin_url"
  echo "DATABASE_URL=postgres://$rds_user:$rds_password@$rds_host/$rds_db_name"
  echo "IAM_ROLE_ARN=$iam_role_arn"
  echo "OUTBOUND_EMAIL_SENDER=$outbound_email_sender"
  echo "PASSWORD_HASH_SALT=$password_hash_salt"
  echo "PGSSLROOTCERT=/etc/ssl/certs/rds-ca-2019-root.crt"
  echo "SENDGRID_API_KEY=$sendgrid_api_key"
  echo "SERVICE_PROXY_LAMBDA_ARN=$service_proxy_lambda_arn"
} > $RAMFS_MOUNT_DIR/app.env
chown app:app $RAMFS_MOUNT_DIR/app.env
chmod 660 $RAMFS_MOUNT_DIR/app.env

echo "$api_cert_data" | jq -r '.certificate' > $RAMFS_MOUNT_DIR/cert.pem
echo "$api_cert_data" | jq -r '.certificate_chain' >> $RAMFS_MOUNT_DIR/cert.pem
echo "$api_cert_data" | jq -r '.private_key' > $RAMFS_MOUNT_DIR/privkey.pem
chmod 600 $RAMFS_MOUNT_DIR/*.pem

systemctl start nginx app worker
