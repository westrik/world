# westrikworld infrastructure

## Assumptions

- Route 53 zone exists for `root_domain_name` (in `variables.tf`)
- Valid `$AWS_ACCESS_KEY_ID`  and `$AWS_SECRET_ACCESS_KEY` are set (should belong to an IAM user with the necessary privileges).
- Terraform (0.13+), Packer, and Ansible are installed

## Usage

To deploy an instance of [westrik.world](https://westrik.world) to `us-east-1`, run:

```sh
make init_state        # create encrypted S3 bucket for TF state
terraform init
make apply_core_infra  # set up security groups, networking, S3 buckets, and IAM roles
make ami               # build app instance AMI (with Packer)
make package_lambdas   # build Lambdas and upload to S3 (with SAM CLI + Docker)
make apply             # apply all remaining changes
```

(TODO: deploys in different regions)
