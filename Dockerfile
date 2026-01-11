# ビルドステージ
FROM rust:1.83 AS builder

WORKDIR /app

# 依存関係のキャッシュ用に先にCargo.tomlだけコピー
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# ソースコードをコピーしてビルド
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

# 実行ステージ（軽量イメージ）
FROM debian:bookworm-slim

# 必要なランタイム依存関係をインストール
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# バイナリをコピー
COPY --from=builder /app/target/release/rust-openai-api-wrapper /usr/local/bin/

# 非rootユーザーで実行
RUN useradd -m appuser
USER appuser

EXPOSE 3000

CMD ["rust-openai-api-wrapper"]
