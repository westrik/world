#!/bin/bash

bundle_name="westrikworld_app"

set -euxo pipefail

mkdir $bundle_name
cd $bundle_name

cp ../infra/codedeploy/appspec.yml .
cp -r ../infra/codedeploy/bin .
cp ../server/target/release/run_server ./bin/
cp -r ../client-web/dist ./public
cp -r ../client-web/vendor ./public/

zip -r ../$bundle_name .
