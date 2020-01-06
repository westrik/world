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

To deploy in a different region, e.g. `us-west-2`:

```
# first, change aws_region and aws_az in variables.tf, then:
make provision_networking
packer build -var 'aws_region=us-west-2' amis/westrikworld_production.json
make apply
```

