# HTTP2SQL

REST API service in Rust that transforms HTTP requests into SQL queries.

## 📚 Table of Contents

- [Prerequisites](#-prerequisites)
- [Configuration](#-configuration)
- [Deployment](#-deployment)
- [API Documentation](#-api-documentation)
- [Development](#-development)
- [License](#-license)

## 🛠 Prerequisites

For deployment:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

For development:

- [Rust](https://www.rust-lang.org/learn/get-started)
- [SQLx CLI](https://crates.io/crates/sqlx-cli)
- [Docker](https://docs.docker.com/get-docker/)

## ⚙ Configuration

The service can be configured using the following environment variables.

Database connection:

- `DATABASE_URL`: The host of the database to connect to. This variable is required for both deployment and development.

Server configuration:

- `HTTP2SQL_SERVER_PORT`: The port to listen on for incoming HTTP requests. (default: 8080)

## 🚀 Deployment

```bash
curl -o compose.yaml https://raw.githubusercontent.com/excoffierleonard/http2sql/refs/heads/main/compose.yaml && \
docker compose up -d
```

## 📖 API Documentation

API documentation is available in [docs/api.md](docs/api.md).

## 🧪 Development

Useful commands for development:

- Full build:

```bash
chmod +x ./scripts/build.sh && \
./scripts/build.sh
```

- Deployment tests:

```bash
chmod +x ./scripts/deploy-tests.sh && \
./scripts/deploy-tests.sh
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
