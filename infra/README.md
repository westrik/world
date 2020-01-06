# westrikworld infra

## Assumptions

- valid `$AWS_ACCESS_KEY_ID`  and `$AWS_SECRET_ACCESS_KEY` are set
- Route 53 zone exists for `frontend_domain_name` (in `variables.tf`)

## Usage

- Install `terraform` and `packer`
- Run:

```sh
terraform init
# Set up required infra, then build AMI with Packer
make apply_packer_infra
make ami
# Provision everything else
make apply
```

To deploy in a different AZ, e.g. `us-west-2a`:

```
terraform apply -target=module.build_resources -var 'aws_region=us-west-2' -var 'aws_az=us-west-2a'
packer build -var 'aws_region=us-west-2' amis/westrikworld_production.json
terraform apply -var 'aws_region=us-west-2' -var 'aws_az=us-west-2a'
```
