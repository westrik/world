#!/usr/bin/env bash

set -euxo pipefail

SERVER_BIN_FILE=/usr/bin/api_server

chown root:root $SERVER_BIN_FILE
chmod 755 $SERVER_BIN_FILE

systemctl daemon-reload
systemctl enable secrets.service secrets.target secrets.timer
systemctl enable app.service app.target
systemctl enable nginx.service

if systemctl restart secrets; then
  echo "secrets restarted OK"
else
  echo "secrets failed to restart"
  systemctl status secrets
fi

if systemctl restart app; then
  echo "app restarted OK"
else
  echo "app failed to restart"
  systemctl status app
fi

if systemctl restart nginx; then
  echo "nginx restarted OK"
else
  echo "nginx failed to restart"
  systemctl status nginx
fi
