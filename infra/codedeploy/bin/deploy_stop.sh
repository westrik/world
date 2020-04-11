#!/bin/bash

set -euo pipefail

if (systemctl -q is-active app); then
  echo "stopping app service..."
  systemctl stop app
fi
