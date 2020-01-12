#!/bin/bash

SERVICE_D="/etc/systemd/system/app.service.d"
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
chmod 700 $SECRETS_ENV_FILE
# TODO: run aws cli to pull db url and pw salt from secrets manager
aws configure set region us-east-1
DATABASE_URL=$(aws secretsmanager get-secret-value --secret-id "database_url" | jq '.SecretString')
PASSWORD_HASH_SALT=$(aws secretsmanager get-secret-value --secret-id "password_hash_salt" | jq '.SecretString')
echo "[Service]" >> $SECRETS_ENV_FILE
echo "Environment=\"DATABASE_URL=$DATABASE_URL\"" >> $SECRETS_ENV_FILE
echo "Environment=\"PASSWORD_HASH_SALT=$PASSWORD_HASH_SALT\"" >> $SECRETS_ENV_FILE
systemctl daemon-reload

chmod +x $SERVER_BIN_FILE
systemctl start app
