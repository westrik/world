#!/bin/bash

SERVICE_CONF="/etc/systemd/system/app.service"
SERVICE_D="$SERVICE_CONF.d"
SECRETS_ENV_FILE="$SERVICE_D/production.conf"
SERVER_BIN_FILE=/usr/bin/run_server

set -euxo pipefail

if ! [ "$(systemctl is-active --quiet nginx)" ]; then
  echo "nginx was not up? starting..."
  systemctl start nginx
fi

if [ -f "$SECRETS_ENV_FILE" ]; then
  rm -f $SECRETS_ENV_FILE
fi
mkdir -p $SERVICE_D
touch $SECRETS_ENV_FILE
chown root:root $SECRETS_ENV_FILE
chmod 644 $SECRETS_ENV_FILE

chown root:root $SERVICE_CONF
chmod 644 $SERVICE_CONF

aws configure set region us-east-1
RDS_HOST=$(aws secretsmanager get-secret-value --secret-id "database_url" | jq -r '.SecretString')
RDS_USER=$(aws secretsmanager get-secret-value --secret-id "database_username" | jq -r '.SecretString')
RDS_PASSWORD=$(aws secretsmanager get-secret-value --secret-id "database_password" | jq -r '.SecretString')
RDS_DB_NAME=$(aws secretsmanager get-secret-value --secret-id "database_name" | jq -r '.SecretString')
PASSWORD_HASH_SALT=$(aws secretsmanager get-secret-value --secret-id "password_hash_salt" | jq -r '.SecretString')
echo "[Service]" >> $SECRETS_ENV_FILE
echo "Environment=\"DATABASE_URL=postgres://$RDS_USER:$RDS_PASSWORD@$RDS_HOST/$RDS_DB_NAME\"" >> $SECRETS_ENV_FILE
echo "Environment=\"PASSWORD_HASH_SALT=$PASSWORD_HASH_SALT\"" >> $SECRETS_ENV_FILE
echo "Environment=\"CORS_ORIGIN_URL=https://westrikworld.com\"" >> $SECRETS_ENV_FILE
systemctl daemon-reload

chown root:root $SERVER_BIN_FILE
chmod 744 $SERVER_BIN_FILE
chmod +x $SERVER_BIN_FILE
systemctl start app
