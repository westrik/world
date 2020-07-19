#!/usr/bin/env bash

set -euo pipefail

# TODO: store this in secretsmanager? seems weird but it's the only non-secret env var rn
CORS_ORIGIN_URL="https://westrik.world"

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
  secret_value=$(aws secretsmanager get-secret-value --secret-id "$secret_id" | jq -r '.SecretString')
  # TODO: escape all potential problematic values
  # secret_value="${secret_value//\%/%%}"
  echo "$secret_value"
}

rds_host=$(get_secret "westrikworld_database_url")
rds_user=$(get_secret "westrikworld_database_username")
rds_password=$(get_secret "westrikworld_database_password")
rds_db_name=$(get_secret "westrikworld_database_name")
password_hash_salt=$(get_secret "westrikworld_password_hash_salt")
api_cert_data=$(get_secret "westrikworld_api_cert" | jq -r '.certificate')

systemctl stop nginx app

{
  echo "DATABASE_URL=postgres://$rds_user:$rds_password@$rds_host/$rds_db_name"
  echo "PASSWORD_HASH_SALT=$password_hash_salt"
  echo "CORS_ORIGIN_URL=$CORS_ORIGIN_URL"
} > $RAMFS_MOUNT_DIR/app.env
chown app:app $RAMFS_MOUNT_DIR/app.env
chmod 660 $RAMFS_MOUNT_DIR/app.env

echo "$api_cert_data" | jq -r '.certificate' > $RAMFS_MOUNT_DIR/cert.pem
echo "$api_cert_data" | jq -r '.certificate_chain' >> $RAMFS_MOUNT_DIR/cert.pem
echo "$api_cert_data" | jq -r '.private_key' > $RAMFS_MOUNT_DIR/privkey.pem
chmod 600 $RAMFS_MOUNT_DIR/*.pem

systemctl start nginx app
