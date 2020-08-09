#!/usr/bin/env bash

set -euo pipefail

apt-get update
# general system dependencies
apt-get install -y sudo curl
# rust toolchain
rustup default nightly
rustup component add clippy
# other toolchain dependencies
apt-get install -y build-essential
# OpenSSL (needed for postgres and tectonic)
apt-get install -y libssl-dev
# postgres dependencies (needed to build diesel w/ pg enabled)
apt-get install -y libpq-dev postgresql postgresql-client
# dependencies used by tectonic (crate used for PDF generation)
apt-get -y install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev zlib1g-dev
