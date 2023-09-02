#!/bin/bash

set -x
set -eo pipefail

# Load environment variables from .env
set -o allexport
source .env
set +o allexport

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: pg_isready is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install --version='~0.6' sqlx-cli \
    --no-default-features --features rustls,postgres"
    echo >&2 "to install it."
    exit 1
    exit 1
fi

docker-compose -f ./scripts/docker-compose.yaml up -d 

export PGPASSWORD="${POSTGRES_PASSWORD}"
until psql -h "${POSTGRES_HOST}" -U "${POSTGRES_USER}" -p ${POSTGRES_PORT} -d "postgres" -c '\q'; do 
    echo >&2 "Postgres is unavailable - sleeping"
    sleep 2
done

>&2 echo "Postgres is up - executing commands"
sqlx database create
sqlx migrate run

>&2 echo Postgres has been migrated, ready to go!
