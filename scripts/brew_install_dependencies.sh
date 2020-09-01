#!/usr/bin/env bash

set -euo pipefail

dev_deps=(
  "yarn"
  "postgres"
  "nginx"
  "openssl@1.1"
  "gpg"
)
infra_deps=(
  "terraform"
  "ansible"
  "packer"
  "awscli"
  "aws-sam-cli"
)

brew update
brew tap aws/tap  # for aws-sam-cli

for dep in "${dev_deps[@]}" "${infra_deps[@]}"; do
  brew install "$dep" || brew upgrade "$dep"
done


## install rust toolchain
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi
# this seems unhygienic (TODO: configure toolchain for project instead?)
rustup default nightly
rustup component add clippy
cargo install cargo-watch
