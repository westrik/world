#!/usr/bin/env bash

set -euxo pipefail

SERVER_BIN_FILE=/usr/bin/run_server

systemctl daemon-reload
systemctl enable secrets.service secrets.target secrets.timer
systemctl enable app.service app.target
systemctl enable nginx.service

chown root:root $SERVER_BIN_FILE
chmod 755 $SERVER_BIN_FILE

systemctl restart secrets app nginx
