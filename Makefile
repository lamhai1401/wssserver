SHELL := /bin/bash

db_url := postgres://postgres:my_password@localhost:5434/my_database

run_server:
	DATABASE_URL=$(db_url) \
		CLIENT_HOST=http://localhost:3000 RUST_BACKTRACE=full \
		JWT_KEY=77397A244326452948404D635166546A576E5A7234753778214125442A472D4A \
		cargo run --bin server
.PHONY: run_server

create_migration:
	DATABASE_URL=$(db_url) diesel migration generate $(name) --migration-dir=db/migrations

migrate:
	DATABASE_URL=$(db_url) diesel migration run --migration-dir=db/migrations

redo_migrate:
	DATABASE_URL=$(db_url) diesel migration redo --migration-dir=db/migrations