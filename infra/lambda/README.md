# westrikworld infra: AWS Lambda

### install poetry 

```sh
curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python
```

### local dependency management

```sh
python3 -m venv ./venv
source .venv/bin/activate
cd "$LAMBDA_FOLDER";
poetry install
```

### packaging

```sh
cd "$LAMBDA_FOLDER"
pip install -r requirements.txt -t .
zip -r "../$LAMBDA_FOLDER" .
```

### update python code only

```sh
cd "$LAMBDA_FOLDER"
zip -g "../$LAMBDA_FOLDER" $PYTHON_FILE
cd $TF_ROOT
terraform taint "module.database.aws_lambda_function.$LAMBDA_NAME"
```
