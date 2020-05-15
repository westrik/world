init_state:
	cd modules/state; terraform init; terraform apply -auto-approve

ami:
	packer build amis/westrikworld_production.json

apply_core_infra:
	terraform apply -target=module.core_infra

apply:
	terraform apply

check:
	packer validate amis/westrikworld_production.json
	terraform validate

fmt:
	terraform fmt
	terraform fmt modules/app_cloudfront
	terraform fmt modules/app_instances
	terraform fmt modules/app_load_balancer
	terraform fmt modules/core_infra
	terraform fmt modules/core_infra/ami_sandbox
	terraform fmt modules/core_infra/lambda_iam_roles
	terraform fmt modules/core_infra/network
	terraform fmt modules/core_infra/network/security_groups
	terraform fmt modules/core_infra/s3_buckets
	terraform fmt modules/database
	terraform fmt modules/deploy_pipeline
	terraform fmt modules/state

deploy_lambdas:
	LAMBDA_FOLDER=./lambda LAMBDA_DEPLOY_BUCKET=$$LAMBDA_DEPLOY_BUCKET bash lambda/deploy_lambdas.sh
