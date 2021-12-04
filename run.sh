#!/bin/bash

cargo install diesel_cli --no-default-features --features postgres

DATABASE_URL=postgres://postgres:my_password@localhost:5434/my_database diesel setup --migration-dir=db/migrations