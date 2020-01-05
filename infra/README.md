# westrikworld infra

## Usage

- Install `terraform` and `packer`
- Set `$AWS_ACCESS_KEY_ID`  and `$AWS_SECRET_ACCESS_KEY`
- Run:

```sh
terraform init
packer build amis/westrikworld_production.json
terraform plan # preview changes
terraform apply
```
