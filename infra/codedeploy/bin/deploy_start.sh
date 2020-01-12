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
DATABASE_URL=$(aws secretsmanager get-secret-value --secret-id "database_url" | jq '.SecretString')
PASSWORD_HASH_SALT=$(aws secretsmanager get-secret-value --secret-id "password_hash_salt" | jq '.SecretString')
echo "[Service]" >> $SECRETS_ENV_FILE
echo "Environment='DATABASE_URL=$DATABASE_URL'" >> $SECRETS_ENV_FILE
echo "Environment='PASSWORD_HASH_SALT=$PASSWORD_HASH_SALT'" >> $SECRETS_ENV_FILE
systemctl daemon-reload

chown root:root $SERVER_BIN_FILE
chmod 744 $SERVER_BIN_FILE
chmod +x $SERVER_BIN_FILE
systemctl start app
