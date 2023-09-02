#!/bin/bash

set -o allexport
source .env set
set +o allexport

sqlx migrate revert
docker-compose -f ./scripts/docker-compose.yaml down
