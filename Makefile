.PHONY: dev dev-api dev-ui up down build test db clean lint fmt migrate db-reset help

# デフォルトターゲット
help:
	@echo "Available commands:"
	@echo "  make dev       - Start DB + API + UI (all in one)"
	@echo "  make dev-api   - Start DB + API only"
	@echo "  make dev-ui    - Start UI only"
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

# 開発環境起動（DB + API + UI 全て起動）
dev:
	docker-compose up -d db
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	@echo "Starting API server on :8080 and UI on :3000..."
	@DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app PORT=8080 cargo run -p api & \
	cd ui && pnpm dev

# API開発サーバーのみ起動
dev-api:
	docker-compose up -d db
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	DATABASE_URL=postgres://postgres:postgres@localhost:5433/chat_app PORT=8080 cargo run -p api

# UI開発サーバーのみ起動
dev-ui:
	cd ui && pnpm dev

# 全てコンテナで起動
up:
	docker-compose up --build

# 全サービス停止（ローカルプロセス + Docker）
down:
	@pkill -9 -f "target/debug/api" 2>/dev/null || true
	@pkill -9 -f "vite" 2>/dev/null || true
	@lsof -ti :3000 | xargs kill -9 2>/dev/null || true
	@lsof -ti :8080 | xargs kill -9 2>/dev/null || true
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
