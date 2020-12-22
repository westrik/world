<p align="center">
<a href="https://westrik.world"><img width="473" height="62" src=".github/images/logo.svg" alt="westrikworld"></a><br />
<a href="https://github.com/westrik/world/actions?query=workflow%3ARust"><img alt="rust build status" src="https://github.com/westrik/world/workflows/Rust/badge.svg"></a>
<a href="https://github.com/westrik/world/actions?query=workflow%3AWeb"><img alt="web build status" src="https://github.com/westrik/world/workflows/Web/badge.svg"></a>
</p>


-------------


## Development


#### Set up local dev-env

Run the dev-env setup script to install dependencies, set up the test databases in Postgres, and configure nginx:

```
cd world
./scripts/setup_dev_env.sh
```

This script will also generate a `.env` file in the root of the project directory.

After the setup script completes, run `make run_server` and `make run_client` in two separate windows, then open [local.westrik.world](https://local.westrik.world) in your browser.




#### Dependencies

The following packages will be installed automatically by `setup_dev_env.sh`.

_Development:_

- [Rust](https://www.rust-lang.org/tools/install) (nightly)
- Yarn
- Postgres 12
- Nginx
- OpenSSL 1.1

_Infrastructure:_

- Terraform 0.13+
- Ansible
- Packer
- AWS CLI
- AWS [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html) (needs Docker)





#### Adding migrations

(First, install `diesel_cli` if it is not already installed):
```sh
cargo install diesel_cli --no-default-features --features postgres
```

Generate `up.sql` and `down.sql` in a new sub-folder of `core/migrations`.

```sh
cd core
diesel migration generate create_users
```

Put some SQL in the generated files.

Next, test the migration in both directions (and apply the migration):

```sh
diesel migration run
diesel migration redo
```

**Note**: If the migration adds a new model, it will need to be added in a few other places:


- `integration_tests::db::destroy_test_db()` - drop the new table
- `core::resource_identifier::ResourceType` (iff an API ID is needed)


## Deployment

- New releases are built by a GitHub Actions worker and deployed to S3 (see `.github/workflows/release_s3.yml`).
- In AWS: CodePipeline picks up the S3 event, deploys to EC2 using CodeDeploy, then updates the S3 bucket backing CloudFront.
