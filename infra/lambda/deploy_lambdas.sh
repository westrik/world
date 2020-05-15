#!/usr/bin/env bash

set -eo pipefail

if [ -z "$LAMBDA_DEPLOY_BUCKET" ]; then
  printf "Enter the deploy bucket name: "
  read -r LAMBDA_DEPLOY_BUCKET
fi
set -u

OUTPUT_FILE=sam_output_$(date +"%Y-%m-%d").yml

function log() {
  printf "\e[1;34m%s\e[0m\n" "$1"
}

function deploy_lambda() {
  lambda_name=$1

  cd "$LAMBDA_FOLDER/$lambda_name"

  log "Building ${lambda_name} with AWS SAM CLI..."
  sam build --use-container

  log "Packaging to s3://${LAMBDA_DEPLOY_BUCKET}..."
  sam package --use-json --s3-bucket="$LAMBDA_DEPLOY_BUCKET" --output-template-file="$OUTPUT_FILE" >/dev/null

  # SAM uses a random name for the ZIP file it saves to S3.
  # This is a problem since we want to deploy the Lambda with Terraform, which requires a known file name.
  # [HACK]: copy the ZIP file created by SAM to a known location.
  random_uri=$(jq -r '.Resources[] | .Properties.CodeUri' "$OUTPUT_FILE"); rm -f "$OUTPUT_FILE"
  known_uri="s3://$LAMBDA_DEPLOY_BUCKET/$lambda_name.zip"
  log "Copying to $known_uri"
  aws s3 cp "$random_uri" "$known_uri" >/dev/null

  log "Finished deploying $lambda_name!"
  printf "\n"

  cd -
}

LAMBDA_NAMES=(
  create_db_user_with_iam_role
  renew_certificate
)
for lambda in "${LAMBDA_NAMES[@]}"; do
  deploy_lambda "$lambda"
done
