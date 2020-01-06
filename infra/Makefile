ami:
	# hack to get around packer breaking ssh-agent by adding 6+ keys
	ssh-add -D
	packer build amis/westrikworld_production.json
	# add back default key
	ssh-add

apply_packer_infra:
	terraform apply -target=module.build_resources

apply:
	terraform apply

check:
	packer validate amis/westrikworld_production.json
	terraform validate
