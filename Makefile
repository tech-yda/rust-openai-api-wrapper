.PHONY: dev dev-api dev-ui up down build test db clean lint fmt migrate db-reset logs help

# デフォルトターゲット
help:
	@echo "Available commands:"
	@echo "  make dev       - Start all services (DB + API + UI) in containers"
	@echo "  make dev-api   - Start DB + API containers only"
	@echo "  make dev-ui    - Start UI container only"
	@echo "  make down      - Stop all containers"
	@echo "  make logs      - Show container logs"
	@echo "  make build     - Build release binary"
	@echo "  make test      - Run tests (requires DB)"
	@echo "  make db        - Start DB container only"
	@echo "  make clean     - Stop containers and remove volumes"
	@echo "  make lint      - Run clippy"
	@echo "  make fmt       - Format code"
	@echo "  make migrate   - Run database migrations"
	@echo "  make db-reset  - Reset database (drops all data)"

# 開発環境起動（全サービスをコンテナで起動）
dev:
	docker-compose up --build

# API + DB のみ起動
dev-api:
	docker-compose up --build db api

# UI のみ起動（APIが既に起動していること前提）
dev-ui:
	docker-compose up --build ui

# 全サービス停止
down:
	docker-compose down

# ログ表示
logs:
	docker-compose logs -f

# ビルド
build:
	cargo build --workspace --release

# テスト（DBが必要）
test:
	docker-compose up -d db
	@sleep 2
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app cargo test --workspace

# DBコンテナのみ起動
db:
	docker-compose up -d db

# クリーンアップ（ボリューム含む）
clean:
	docker-compose down -v
	rm -rf ui/node_modules ui/.output ui/dist
	rm -rf target

# Lint
lint:
	cargo clippy --workspace -- -D warnings

# フォーマット
fmt:
	cargo fmt

# フォーマットチェック
fmt-check:
	cargo fmt -- --check

# マイグレーション実行
migrate:
	docker-compose up -d db
	@sleep 2
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app sqlx migrate run --source backend/core/src/db/migrations

# マイグレーション作成
migrate-create:
	@read -p "Migration name: " name; \
	sqlx migrate add --source backend/core/src/db/migrations $$name

# DB初期化（開発用）
db-reset:
	docker-compose down -v
	docker-compose up -d db
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app sqlx migrate run --source backend/core/src/db/migrations

# sqlx準備（オフラインモード用）
sqlx-prepare:
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app cargo sqlx prepare
