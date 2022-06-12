#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}"  -c '\l';


PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "9d853544-9fe1-450b-bbb6-98df68b213b4" -c '\dt';

PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "9d853544-9fe1-450b-bbb6-98df68b213b4" -c 'select * from subscriptions;';

