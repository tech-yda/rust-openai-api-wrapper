# Build stage
FROM rust:1.83 AS builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY backend/core/Cargo.toml ./backend/core/
COPY backend/api/Cargo.toml ./backend/api/
COPY backend/cli/Cargo.toml ./backend/cli/

# Create dummy source files for dependency caching
RUN mkdir -p backend/core/src backend/api/src backend/cli/src \
    && echo "pub fn main() {}" > backend/core/src/lib.rs \
    && echo "fn main() {}" > backend/api/src/main.rs \
    && echo "fn main() {}" > backend/cli/src/main.rs

# Build dependencies
RUN cargo build --release -p api

# Remove dummy files
RUN rm -rf backend

# Copy actual source code
COPY backend ./backend
COPY migrations ./migrations

# Touch files to rebuild with real source
RUN touch backend/core/src/lib.rs backend/api/src/main.rs

# Build
RUN cargo build --release -p api

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /usr/local/bin/

RUN useradd -m appuser
USER appuser

EXPOSE 8080

CMD ["api"]
