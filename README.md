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

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## ⚙ Configuration

The service can be configured using the following environment variables.

Database connection:

- `HTTP2SQL_DB_HOST`: The host of the database to connect to. (default: http2sql-db)
- `HTTP2SQL_DB_PORT`: The port of the database to connect to. (default: 3306)
- `HTTP2SQL_DB_NAME`: The name of the database to connect to. (default: http2sql-db)
- `HTTP2SQL_DB_USER`: The user to use to connect to the database. (default: http2sql-db)
- `HTTP2SQL_DB_PASSWORD`: The password to use to connect to the database. (default: http2sql-db)

Server configuration:

- `HTTP2SQL_SERVER_PORT`: The port to listen on for incoming HTTP requests. (default: 8080)
- `HTTP2SQL_SERVER_WORKERS`: The number of worker threads to use for handling incoming HTTP requests. (default: all available cores)

## 🚀 Deployment

```bash
curl -o compose.yaml https://raw.githubusercontent.com/excoffierleonard/http2sql/refs/heads/main/compose.yaml && docker compose up -d
```

## 📖 API Documentation

API documentation is available in [docs/api.md](docs/api.md).

## 🧪 Development

Useful commands for development:

```bash
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.