#!/usr/bin/env bash

set -eo pipefail
if [ -z "$PROJECT_ROOT" ]; then
  echo "Expected PROJECT_ROOT to be set"
  exit 1
fi
set -u

cd "$PROJECT_ROOT/infra/lambda"

# Note: this script assumes that lambda names won't include spaces
LAMBDAS=(
  create_db_user_with_iam_role
  renew_certificate
)

# TODO: spin up EC2 instance w/ SSH key
# OQ: where does the VPC ID come from?
# - requires :22 to be open OR needs an additional bastion instance
# OQ: can this be a lambda instead?????
# - more secure (no SSH), faster to deploy

for lambda in "${LAMBDAS[@]}"; do
  echo "Deploying $lambda";
  SOURCE_BUNDLE="$lambda.tar.gz"
  tar -czf "$SOURCE_BUNDLE" "$lambda"

  # TODO: use AWS CLI to trigger deploy lambda

  # - OR: -

  # TODO: scp bundle to the EC2 instance
  # TODO: run deploy script for this lambda
  #  first:
  #    - echo lambda name
  #  later:
  #    - scp lambda source code
  #    - pip install
  #    - tar up the whole bundle
  #    - send the bundle to S3

  rm -f "$SOURCE_BUNDLE"
done

# TODO: kill the instance
