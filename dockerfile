# Step 1: Build the application with musl target
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /http2sql

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/main.rs

COPY src src/

RUN touch src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM alpine

WORKDIR /http2sql

ENV HTTP2SQL_DB_HOST=http2sql-db
ENV HTTP2SQL_DB_PORT=3306
ENV HTTP2SQL_DB_NAME=http2sql
ENV HTTP2SQL_DB_USER=http2sql
ENV HTTP2SQL_DB_PASSWORD=http2sql

COPY --from=builder /http2sql/target/x86_64-unknown-linux-musl/release/http2sql .

CMD ["./http2sql"]