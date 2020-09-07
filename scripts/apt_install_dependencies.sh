#!/usr/bin/env bash

set -euo pipefail

apt-get update
# general system dependencies
apt-get install -y sudo curl
# other toolchain dependencies
apt-get install -y build-essential
# OpenSSL (needed for postgres and tectonic)
apt-get install -y libssl-dev
# postgres dependencies (needed to build diesel w/ pg enabled)
apt-get install -y libpq-dev postgresql postgresql-client
# dependencies used by tectonic (crate used for PDF generation)
apt-get -y install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev zlib1g-dev


## install rust toolchain
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- -y
fi
# this seems unhygienic (TODO: configure toolchain for project instead?)
rustup default nightly
rustup component add clippy
cargo install cargo-watch
