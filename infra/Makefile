init_state:
	cd modules/state; terraform init; terraform apply -auto-approve

ami:
	packer build amis/westrikworld_production.json

apply_packer_infra:
	terraform apply -target=module.ami_sandbox

apply:
	terraform apply

check:
	packer validate amis/westrikworld_production.json
	terraform validate

fmt:
	terraform fmt
	terraform fmt modules/ami_sandbox
	terraform fmt modules/api
	terraform fmt modules/deploy_buckets
	terraform fmt modules/deploy_lambdas
	terraform fmt modules/database
	terraform fmt modules/network
	terraform fmt modules/state
