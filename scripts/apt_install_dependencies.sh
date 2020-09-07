#!/usr/bin/env bash

set -euo pipefail

apt-get update

# General system dependencies
apt-get install -y \
    sudo \
    curl

# Other toolchain dependencies
apt-get install -y \
    build-essential \
    lsb-release \

# OpenSSL (needed for postgres and tectonic)
apt-get install -y libssl-dev

# Dependencies used by tectonic (crate used for PDF generation)
apt-get install -y \
    libfontconfig1-dev \
    libgraphite2-dev \
    libharfbuzz-dev \
    libicu-dev \
    zlib1g-dev

# Postgres 12 (needed to build diesel w/ pg enabled)
sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
curl https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
apt-get update
apt-get install -y \
    postgresql-12 \
    postgresql-client-12 \
    libpq-dev
