# Chatbot Service

This project was generated using fastapi_template.

## Build proto

You need to install `grpcio-tools` to use `grpc_tools.protoc` command. then:

```bash
python -m grpc_tools.protoc -I./api/proto --python_out=./api/proto/pb2 --pyi_out=./api/proto/pb2 --grpc_python_out=./api/proto/pb2 ./api/proto/helloworld.proto
python -m grpc_tools.protoc -I./api/proto --python_out=./api/proto/pb2 --pyi_out=./api/proto/pb2 --grpc_python_out=./api/proto/pb2 ./api/proto/health.proto
```

***note:*** need change *from api.proto.pb2 import helloworld_pb2 as helloworld__pb2* in helloworld_pb2_grpc.py.

```bash

## Poetry

This project uses poetry. It's a modern dependency management
tool.

To run the project use this set of commands:

```bash
poetry install
```

```bash
poetry run python -m api
```

This will start the server on the configured host.

You can find swagger documentation at `/api/docs`.

You can read more about poetry here: https://python-poetry.org/

## Docker

You can start the project with docker using this command:

```bash
docker-compose -f deploy/docker-compose.yml --project-directory . up --dist
```

If you want to develop in docker with autoreload add `-f deploy/docker-compose.dev.yml` to your docker command.
Like this:

```bash
docker-compose -f deploy/docker-compose.yml -f deploy/docker-compose.dev.yml --project-directory . up --dist
```

This command exposes the web application on port 8000, mounts current directory and enables autoreload.

But you have to rebuild image every time you modify `poetry.lock` or `pyproject.toml` with this command:

```bash
docker-compose -f deploy/docker-compose.yml --project-directory . dist
```

## Configuration

This application can be configured with environment variables.

You can create `.env` file in the root directory and place all
environment variables here.

All environment variables should start with "API_" prefix.

For example if you see in your "api/settings.py" a variable named like
`random_parameter`, you should provide the "API_RANDOM_PARAMETER"
variable to configure the value. This behaviour can be changed by overriding `env_prefix` property
in `api.settings.Settings.Config`.

An example of .env file:
```bash
API_RELOAD="True"
API_PORT="8000"
API_ENVIRONMENT="dev"
```

You can read more about BaseSettings class here: https://pydantic-docs.helpmanual.io/usage/settings/

## Pre-commit

To install pre-commit simply run inside the shell:
```bash
pre-commit install
```

pre-commit is very useful to check your code before publishing it.
It's configured using .pre-commit-config.yaml file.

By default it runs:
* black (formats your code);
* mypy (validates types);
* isort (sorts imports in all files);
* flake8 (spots possible bugs);


You can read more about pre-commit here: https://pre-commit.com/

## Migrations

If you want to migrate your database, you should run following commands:
```bash
# Upgrade database to the last migration.
aerich upgrade
```

### Reverting migrations

If you want to revert migrations, you should run:
```bash
aerich downgrade
```

### Migration generation

To generate migrations you should run:
```bash
aerich migrate
```


## Running tests

If you want to run it in docker, simply run:

```bash
docker-compose -f deploy/docker-compose.yml -f deploy/docker-compose.dev.yml --project-directory . run --dist --rm api pytest -vv .
docker-compose -f deploy/docker-compose.yml -f deploy/docker-compose.dev.yml --project-directory . down
```

For running tests on your local machine.
1. you need to start a database.

I prefer doing it with docker:
```
docker run -p "5432:5432" -e "POSTGRES_PASSWORD=api" -e "POSTGRES_USER=api" -e "POSTGRES_DB=api" postgres:13.8-bullseye
```


2. Run the pytest.
```bash
pytest -vv .
```
