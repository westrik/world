#!/usr/bin/env bash

set -euo pipefail

apt-get update
# general system dependencies
apt-get install -y sudo curl
# other toolchain dependencies
apt-get install -y build-essential lsb-release
# OpenSSL (needed for postgres and tectonic)
apt-get install -y libssl-dev
# postgres 12 (needed to build diesel w/ pg enabled)
sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
curl https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
apt-get update
apt-get install -y postgresql-12 postgresql-client-12
# dependencies used by tectonic (crate used for PDF generation)
apt-get -y install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev zlib1g-dev
