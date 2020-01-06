# westrikworld infra

## Assumptions

- valid `$AWS_ACCESS_KEY_ID`  and `$AWS_SECRET_ACCESS_KEY` are set

## Usage

- Install `terraform` and `packer`
- Run:

```sh
terraform init
# Run TF to provision VPCs, security groups, and subnets
make provision_networking
# Build AMI with Packer in one of our new VPCs
make ami
# Provision instances that use the AMI
make apply
```

To deploy in a different AZ, e.g. `us-west-2a`:

```
terraform apply -var 'setup_only=true' -var 'aws_region=us-west-2' -var 'aws_az=us-west-2a'
packer build -var 'aws_region=us-west-2' amis/westrikworld_production.json
terraform apply -var 'aws_region=us-west-2' -var 'aws_az=us-west-2a'
```

