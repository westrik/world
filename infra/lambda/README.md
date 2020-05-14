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

- Install [AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html).

```sh
sam build --use-container
sam package --s3-bucket [...]
```
