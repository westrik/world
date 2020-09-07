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
