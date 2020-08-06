#!/usr/bin/env bash

set -euo pipefail

if (systemctl -q is-active app); then
  echo "stopping app service..."
  systemctl stop app
fi

if (systemctl -q is-active worker); then
  echo "stopping worker service..."
  systemctl stop worker
fi
