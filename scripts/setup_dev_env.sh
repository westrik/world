#!/usr/bin/env bash

set -euo pipefail

ROOT_DOMAIN_NAME="local.westrik.world"

scripts_dir="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
project_root_dir="$scripts_dir/.."

# If `sudo` does not exist, assume that we'll install it later and that we'll
#   be able to use it without a password.
if command -v sudo >/dev/null 2>&1; then
  sudo -v
  # keep-alive: update existing `sudo` time stamp until script is done
  while true; do sudo -n true; sleep 60; kill -0 "$$" || exit; done 2>/dev/null &
fi

# -----------------------------------------------------------------------------
# [STAGE 1] install dependencies
# -----------------------------------------------------------------------------

if [[ "$OSTYPE" == "darwin"* ]]; then
  bash "$scripts_dir/brew_install_dependencies.sh"
else
  if ! command -v apt >/dev/null 2>&1; then
    echo "only apt is supported (for now)"
  else
    bash "$scripts_dir/apt_install_dependencies.sh"
  fi
fi

<<<<<<< HEAD
=======
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi
# this seems unhygienic (TODO: configure toolchain for project instead?)
rustup default nightly
cargo install cargo-watch

>>>>>>> b4422df... [tools] add script to set up local dev environment (#180)
# -----------------------------------------------------------------------------
# [STAGE 2] database & environment setup
# -----------------------------------------------------------------------------

# create .env
if test -f "$project_root_dir/.env"; then
  echo ".env already exists, skipping DB setup"
else
  bash "$scripts_dir/setup_database.sh"

  echo "ROOT_DOMAIN_NAME=\"$ROOT_DOMAIN_NAME\"" >> .env
  echo "PASSWORD_HASH_SALT=\"$(gpg --gen-random --armor 0 32)\"" >> .env
fi

# generate self-signed certs with OpenSSL
sudo mkdir -p /etc/ssl/{certs,private}
if ! test -f "/etc/ssl/certs/nginx-selfsigned.crt"; then
  sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout "/etc/ssl/private/nginx-selfsigned.key" -out "/etc/ssl/certs/nginx-selfsigned.crt"
fi
if ! test -f "/etc/ssl/certs/nginx-selfsigned-api.crt"; then
  sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout "/etc/ssl/private/nginx-selfsigned-api.key" -out "/etc/ssl/certs/nginx-selfsigned-api.crt"
fi
if ! test -f "/etc/ssl/certs/dhparam.pem"; then
  sudo openssl dhparam -out /etc/ssl/certs/dhparam.pem 2048
fi

# configure nginx
if [[ "$OSTYPE" == "darwin"* ]]; then
  if ! test -f /usr/local/etc/nginx/selfsigned.conf; then
    sudo ln -s "$project_root_dir/infra/nginx/selfsigned.conf" /usr/local/etc/nginx/selfsigned.conf
  fi
  if ! test -f /usr/local/etc/nginx/world.conf; then
    sudo ln -s "$project_root_dir/infra/nginx/local.conf" /usr/local/etc/nginx/world.conf
  fi
  nginx_conf="/usr/local/etc/nginx/nginx.conf"
  # TODO: fix the way this config is imported
  grep -qxF 'include world.conf;' $nginx_conf || echo 'include world.conf;' >> $nginx_conf
  sudo nginx -t && sudo brew services restart nginx
fi
