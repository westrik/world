#!/bin/bash

set -euxo pipefail

if ! [ "$(systemctl is-active --quiet nginx)" ]; then
  echo "nginx was not up? starting..."
  systemctl start nginx
fi

chmod +x /usr/bin/run_server
systemctl start app_server
