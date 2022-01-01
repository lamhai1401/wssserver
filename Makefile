SHELL := /bin/bash

db_url := postgres://postgres:my_password@localhost:5434/my_database

# 0
run_server:
	DATABASE_URL=$(db_url) \
		CLIENT_HOST=http://localhost:3000 RUST_BACKTRACE=full \
		JWT_KEY=77397A244326452948404D635166546A576E5A7234753778214125442A472D4A \
		cargo run --bin server
.PHONY: run_server

# 1
create_migration:
	DATABASE_URL=$(db_url) diesel migration generate $(name) --migration-dir=db/migrations
# 2
migrate:
	DATABASE_URL=$(db_url) diesel migration run --migration-dir=db/migrations

redo_migrate:
	DATABASE_URL=$(db_url) diesel migration redo --migration-dir=db/migrations

test_prepare:
	docker-compose -f docker-compose.test.yml up -d
	DATABASE_URL=postgres://root@localhost:5433/my_database_test diesel migration run --migration-dir=db/migrations

test:
	docker-compose -f docker-compose.test.yml exec database_test psql -d my_database_test --c="TRUNCATE questions"
	DATABASE_URL=postgres://root@localhost:5433/my_database_test \
		cargo test $(T) -- --nocapture --test-threads=1

# Update the phony line:
.PHONY: run_server test test_prepare

