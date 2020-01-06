ami:
	# hack to get around packer breaking ssh-agent by adding 6+ keys
	ssh-add -D
	packer build amis/westrikworld_production.json
	# add back default key
	ssh-add

apply:
	terraform apply

check:
	terraform plan
