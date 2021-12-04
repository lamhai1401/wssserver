SHELL := /bin/bash

db_url := postgres://postgres:my_password@localhost:5434/my_database

run_server:
	DATABASE_URL=$(db_url) \
		CLIENT_HOST=http://localhost:3000 RUST_BACKTRACE=full \
		JWT_KEY=77397A244326452948404D635166546A576E5A7234753778214125442A472D4A \
		cargo run --bin wssserver
.PHONY: run_server