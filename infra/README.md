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
terraform apply        # apply all remaining changes
```

(TODO: deploys in different regions)



----------

### replacing CodeDeploy + CodePipeline setup with Consul + Nomad

- [Consul reference architecture](https://learn.hashicorp.com/tutorials/consul/reference-architecture?in=consul/datacenter-deploy)
- [Nomad reference architecture](https://learn.hashicorp.com/tutorials/nomad/production-reference-architecture-vm-with-consul?in=nomad/production)

- [Consul deployment guide](https://learn.hashicorp.com/tutorials/consul/deployment-guide)
- [Nomad deployment guide](https://learn.hashicorp.com/tutorials/nomad/production-deployment-guide-vm-with-consul)
- [Consul cloud auto-join](https://www.consul.io/docs/install/cloud-auto-join)
- [enable TLS encryption for Consul](https://learn.hashicorp.com/tutorials/consul/tls-encryption-secure)
- [enable TLS encryption for Nomad](https://learn.hashicorp.com/tutorials/nomad/security-enable-tls)
- [Nomad blue/green deployments](https://learn.hashicorp.com/tutorials/nomad/job-blue-green-and-canary-deployments)
