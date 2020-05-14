# westrikworld infra: AWS Lambda

### dependency management for local development

##### install poetry

```sh
curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python
```

##### install lambda deps in virtualenv

```sh
python3 -m venv ./venv
source .venv/bin/activate
cd "$LAMBDA_FOLDER";
poetry install
```

### deploying

```sh
# Deploy lambdas to S3 via temporary EC2 instance
make deploy_lambdas
```
