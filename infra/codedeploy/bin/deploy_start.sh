#!/usr/bin/env bash

set -euo pipefail

SERVER_BIN_FILE=/usr/bin/api_server

chown root:root $SERVER_BIN_FILE
chmod 755 $SERVER_BIN_FILE

systemctl daemon-reload
systemctl enable secrets.service secrets.target secrets.timer
systemctl enable app.service app.target
systemctl enable nginx.service

if systemctl restart secrets; then
  print "secrets restarted OK"
else
  print "secrets failed to restart"
  systemctl status secrets
fi

if systemctl restart app; then
  print "app restarted OK"
else
  print "app failed to restart"
  systemctl status app
fi

if systemctl restart nginx; then
  print "nginx restarted OK"
else
  print "nginx failed to restart"
  systemctl status nginx
fi
