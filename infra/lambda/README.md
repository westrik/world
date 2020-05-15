# westrikworld infra: AWS Lambda

### dependencies & testing

- Install [AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html) and Docker.

```sh
# from /infra/create_db_user_with_iam_role:
sam build --use-container
sam local invoke "CreateDBUserWithIAMRoleFunction" -e events/event.json
```

### deploying

```sh
# from /infra/:
export LAMBDA_DEPLOY_BUCKET=westrikworld-lambda-deploy-ABCDEF
make deploy_lambdas
```

### re-deploying

```sh
# from /infra/:
export LAMBDA_DEPLOY_BUCKET=westrikworld-lambda-deploy-ABCDEF
make deploy_lambdas
terraform taint "module.database.aws_lambda_function.create_db_user_with_iam_role"
terraform apply
```
