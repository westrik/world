init_state:
	cd modules/state; terraform init; terraform apply -var-file="secrets.tfvars" -auto-approve

build_ami:
	packer build amis/westrikworld_production.json

apply_core_infra:
	terraform apply -target=module.core_infra -var-file="secrets.tfvars"

apply:
	terraform apply -var-file="secrets.tfvars"

check:
	packer validate amis/westrikworld_production.json
	terraform validate

fmt:
	terraform fmt
	terraform fmt tf_modules/app_cloudfront
	terraform fmt tf_modules/app_cluster
	terraform fmt tf_modules/app_cluster/autoscaling_group
	terraform fmt tf_modules/app_cluster/load_balancer
	terraform fmt tf_modules/content_buckets
	terraform fmt tf_modules/core_infra
	terraform fmt tf_modules/core_infra/ami_sandbox
	terraform fmt tf_modules/core_infra/lambda_iam_roles
	terraform fmt tf_modules/core_infra/network
	terraform fmt tf_modules/core_infra/network/security_groups
	terraform fmt tf_modules/core_infra/s3_buckets
	terraform fmt tf_modules/database
	terraform fmt tf_modules/deploy_pipeline
	terraform fmt tf_modules/secrets
	terraform fmt tf_modules/state
	terraform fmt tf_modules/user_site

package_lambdas:
	LAMBDA_FOLDER=./lambda LAMBDA_DEPLOY_BUCKET=$$LAMBDA_DEPLOY_BUCKET bash lambda/deploy_lambdas.sh
