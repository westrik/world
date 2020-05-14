# westrikworld infra

## Assumptions

- valid `$AWS_ACCESS_KEY_ID`  and `$AWS_SECRET_ACCESS_KEY` are set
- Route 53 zone exists for `frontend_domain_name` (in `variables.tf`)

## Usage

- Install `terraform` and `packer`
- Run:

```sh

make init_state        # create encrypted S3 bucket for TF state
terraform init
make apply_core_infra  # set up security groups, networking, S3 buckets, and IAM roles
make ami               # build app instance AMI (with Packer)
make deploy_lambdas    # build & deploy Lambdas (with SAM CLI + Docker)
terraform apply        # apply all remaining changes
```

To deploy in a different AZ, e.g. `us-west-2a`:

```
terraform apply -target=module.build_resources -var 'aws_region=us-west-2' -var 'aws_az=us-west-2a'
packer build -var 'aws_region=us-west-2' amis/westrikworld_production.json
terraform apply -var 'aws_region=us-west-2' -var 'aws_az=us-west-2a'
```
