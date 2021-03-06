#!/usr/bin/env bash

set -euxo pipefail

SERVER_BIN_FILE=/usr/bin/api_server
chown root:root $SERVER_BIN_FILE
chmod 755 $SERVER_BIN_FILE
WORKER_BIN_FILE=/usr/bin/worker
chown root:root $WORKER_BIN_FILE
chmod 755 $WORKER_BIN_FILE

systemctl daemon-reload
systemctl enable secrets.service secrets.target secrets.timer
systemctl enable app.service app.target
systemctl enable worker.service worker.target
systemctl enable nginx.service

if systemctl restart secrets; then
  echo "secrets restarted OK"
else
  echo "secrets failed to restart"
  journalctl --no-pager -u secrets -b
fi

if systemctl restart app; then
  echo "app restarted OK"
else
  echo "app failed to restart"
  journalctl --no-pager -u app -b
fi

if systemctl restart worker; then
  echo "worker restarted OK"
else
  echo "worker failed to restart"
  journalctl --no-pager -u worker -b
fi

nginx -t
if systemctl restart nginx; then
  echo "nginx restarted OK"
else
  echo "nginx failed to restart"
  journalctl --no-pager -u nginx -b
fi
