#!/usr/bin/env bash

set -euo pipefail

bundle_name="westrikworld_app"

mkdir $bundle_name
cd $bundle_name

cp ../infra/codedeploy/appspec.yml .
cp -r ../infra/codedeploy/bin .
cp ../target/release/world_api_server ./bin/api_server
cp ../target/release/world_worker ./bin/worker
cp -r ../web-client/dist ./public
cp -r ../web-client/vendor ./public/

zip -r ../$bundle_name .
