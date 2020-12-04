#!/usr/bin/env bash

set -euo pipefail

## install rust toolchain
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
fi
# this seems unhygienic (TODO: configure toolchain for project instead?)
rustup default nightly
rustup component add clippy
cargo install cargo-watch
