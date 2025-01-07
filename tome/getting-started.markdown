# Getting started

## Requirements

Please make sure you have the following tools installed on your system:

- [Docker](https://docker.com)
- [Git](https://git-scm.com)

## Running the Docker services

To get Jade up and running, please follow these steps:

- 1.) Clone this repository.
- 2.) Change directory into the repository's root.
- 3.) Set the following environment variables in the provided `docker-compose.yml`: 
    - `POSTGRES_PASSWORD`: The password for your PostgreSQL database.
    - `API_DOMAIN`: The domain from which your JAde's API will be running.
    - `SMTP_SERVER`: The address for SMTP services from a mail provider of your choice.
    - `INSTANCE_DOMAIN`: This is the domain of your Kleah instance.
- 4.) Start the containers with the command: `docker compose up -d`.

## Running unit tests

If you want to run the provided unit tests, you will also have to install PostgresSQL for your platform. To run the tests, run the following commands in sequence:

```Bash
sqlx database create --database-url postgres://postgres:$POSTGRES_PWD@localhost:5432/jade
sqlx migrate run
cargo test
```

The `POSTGRES_PWD` environment variable is the password for the PostgreSQL user `postgres`.