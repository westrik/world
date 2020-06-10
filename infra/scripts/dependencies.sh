#!/usr/bin/env bash

set -euo pipefail

apt-get update
# general system dependencies
apt-get install -y sudo curl
# install rust toolchain
rustup default nightly
# other toolchain dependencies
apt-get install -y build-essential
# postgres dependencies (needed to build diesel w/ pg enabled)
apt-get install -y libpq-dev postgresql postgresql-client
# dependencies used by tectonic (for PDF generation)
apt-get -y install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
