#!/bin/bash

#colors
RED='\033[0;31m'
GREEN='\033[0;32m'
RESET='\033[0m'

#shape
CIRCLE='\u25CF'


echo -e "${CIRCLE} Executing backend start script..."

function validate_db() {
    echo -e "${CIRCLE} Validating Postgres database..."
    if ! pg_isready -d "$DATABASE_URL" -q; then
        echo -e "${RED}${CIRCLE}${RESET} ERROR: Database is not ready. Check postgres_db service"
        exit 1
    fi
    echo -e "${GREEN}${CIRCLE}${RESET} Database is ready!"

}

function run_migration() {
    echo -e "${CIRCLE} Running migration script..."
    if [ ! -d "migrations" ]; then
        echo -e "${RED}${CIRCLE}${RESET} ERROR: Migrations folder doesn't exist"
        exit 1
    fi

    if output=$(diesel setup); then
        echo -e "${GREEN}${CIRCLE}${RESET} Migration directory was configured"
        cat diesel.toml
    else
        echo -e "${RED}${CIRCLE}${RESET} ERROR: Diesel initialization failed"
        exit 1
    fi

    if output=$(diesel migration run); then
        echo -e "${GREEN}${CIRCLE}${RESET} Migration completed successfully"
    else
        echo -e "${RED}${CIRCLE}${RESET} Migrations folder doesn't exist"
        exit 1
    fi


}

function execute_bin() {
    echo -e "${GREEN}${CIRCLE}${RESET} Executing binary..."
    exec ./bullseye
}

function main {
    validate_db
    run_migration
    execute_bin
}

main "$@"