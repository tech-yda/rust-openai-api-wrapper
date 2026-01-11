.PHONY: dev dev-ui up down build test db clean lint fmt migrate db-reset help

# デフォルトターゲット
help:
	@echo "Available commands:"
	@echo "  make dev       - Start DB container and run API locally"
	@echo "  make dev-ui    - Start UI dev server"
	@echo "  make up        - Start all containers"
	@echo "  make down      - Stop all containers"
	@echo "  make build     - Build release binary"
	@echo "  make test      - Run tests (requires DB)"
	@echo "  make db        - Start DB container only"
	@echo "  make clean     - Stop containers and remove volumes"
	@echo "  make lint      - Run clippy"
	@echo "  make fmt       - Format code"
	@echo "  make migrate   - Run database migrations"
	@echo "  make db-reset  - Reset database (drops all data)"

# 開発環境起動（DBコンテナ + ローカルRust API）
dev:
	docker-compose up -d db
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app cargo run -p api

# UI開発サーバー起動
dev-ui:
	cd ui && pnpm dev

# 全てコンテナで起動
up:
	docker-compose up --build

# コンテナ停止
down:
	docker-compose down

# ビルド
build:
	cargo build --workspace --release

# テスト（DBが必要）
test:
	docker-compose up -d db
	@sleep 2
	cargo test --workspace

# DBコンテナのみ起動
db:
	docker-compose up -d db

# クリーンアップ
clean:
	docker-compose down -v

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
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app sqlx migrate run

# マイグレーション作成
migrate-create:
	@read -p "Migration name: " name; \
	sqlx migrate add $$name

# DB初期化（開発用）
db-reset:
	docker-compose down -v
	docker-compose up -d db
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app sqlx migrate run

# sqlx準備（オフラインモード用）
sqlx-prepare:
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app cargo sqlx prepare
