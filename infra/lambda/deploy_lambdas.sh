#!/usr/bin/env bash

set -eo pipefail
if [ -z "$PROJECT_ROOT" ]; then
  echo "Expected PROJECT_ROOT to be set"
  exit 1
fi
set -u

# Note: this script assumes that lambda names won't include spaces
LAMBDAS=(
  create_db_user_with_iam_role
  renew_certificate
)

# TODO: spin up EC2 instance
# OQ: where does the VPC ID come from?

for lambda in "${LAMBDAS[@]}"; do
  echo "Deploying $lambda";
  SOURCE_BUNDLE="$lambda.tar.gz"
  tar -czf "$SOURCE_BUNDLE" "$lambda"
  # TODO: scp bundle to the EC2 instance
  rm -f "$SOURCE_BUNDLE"
  # TODO: run deploy script for this lambda
  #  first:
  #    - echo lambda name
  #  later:
  #    - scp lambda source code
  #    - pip install
  #    - tar up the whole bundle
  #    - send the bundle to S3
done

# TODO: kill the instance
