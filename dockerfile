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

# Copy .sqkx file here to have db schema available for the build
COPY .sqlx .sqlx/

COPY src src/

# Update the timestamp of the main file to force a rebuild
RUN touch src/main.rs

# Final build
RUN cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM scratch

WORKDIR /http2sql

ENV DATABASE_URL="mysql://http2sql:http2sql@db:3306/http2sql"

COPY --from=builder /http2sql/target/x86_64-unknown-linux-musl/release/http2sql .

EXPOSE 8080

CMD ["./http2sql"]