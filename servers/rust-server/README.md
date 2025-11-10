# Finwar Rust Server - AI Coding Instructions

## Project Overview

Finwar is a finance bot tournament server simulating a stock market. Bots join the server, trade in a simulated market, and compete for profit. This is a Rust web server using Axum for HTTP handling, SeaORM for database operations, and Plotly for data visualization.

## Core Architecture

### Tech Stack

- **Web Framework**: Axum (async HTTP server)
- **Database**: PostgreSQL with SeaORM for ORM operations
- **Templates**: Askama (Jinja2-like templating)
- **Data Processing**: Polars for CSV data manipulation
- **Visualization**: Plotly for interactive charts

## Development Workflows

### Prerequis

```bash
# Install SeaORM CLI
cargo install sea-orm-cli
```

### Database Operations

```bash
# Start PostgreSQL container
docker-compose up -d timescaledb

# Run migrations (from migration/ directory)
sea-orm-cli migrate down
sea-orm-cli migrate up

# Generate new migration
sea-orm-cli migrate generate <migration_name>

# Generate the entities
sea-orm-cli generate entity -o entity 2> /dev/null

# Reset database
sea-orm-cli migrate reset
```

### Environment Setup

- Set `FINWAR_MARKET_DB_USER=finwar`, `FINWAR_MARKET_DB_PASSWORD=password`, and `FINWAR_MARKET_DB_NAME=finwar` as an env variable or in a .env
- Default server runs on `0.0.0.0:4444`
- Stock data loaded from `./local/data/Stocks/` directory

### Running the Server

#### Local Development

```bash
cargo run
```

#### Docker

**Build the image:**

```bash
docker build -t finwar-rust-server -f ./docker/Dockerfile .
```

**Run only the server container:**

```bash
docker run -p 4444:4444 -e FINWAR_MARKET_DB_USER=finwar FINWAR_MARKET_DB_PASSWORD=password FINWAR_MARKET_DB_NAME=finwar finwar-rust-server
```

**Or use docker-compose (recommended):**

```bash
# From project root
docker-compose up rust-server
```

### to create a simple bot

```sh
curl -X POST http://localhost:4444/api/enroll -H "Content-Type: application/json" -d '{"name":"bot0"}'
```

or in powershell:

```powershell
Invoke-RestMethod -Uri http://localhost:4444/api/enroll -Method POST -ContentType "application/json" -Body '{"name":"bot0"}'
```

## Configuration

| Environment Variable                  | Command Line.        | Description                                                                  | Default                 |
| :------------------------------------ | :------------------  | :--------------------------------------------------------------------------- | :---------------------: |
| `FINWAR_MARKET_LOG_LEVEL`             | `--log-level`        | The log level of the application.                                            | `info`                  |
| `FINWAR_MARKET_HOST`                  | `--host`             | The HTTP server address.                                                     | `0.0.0.0`               |
| `FINWAR_MARKET_PORT`                  | `--port`             | The HTTP server port                                                         | `4444`                  |
| `FINWAR_MARKET_INTERVAL_SECONDS`      | `--interval-seconds` | The internal time passed (in seconds) in the Market each real second.        | `60`                    |
| `FINWAR_MARKET_TRACKED_SYMBOL`        | `--tracked-symbols`  | The tracked stock ticker symbol on which the Market simulation is based on.  | `AAPL`                  |
| `FINWAR_MARKET_DB_USER`               | `--db-user`          | The username used for the database authentication.                           |                         |
| `FINWAR_MARKET_DB_USER_FILE`          | `--db-user-file`     | The path of the file containing the database auth username.                  |                         |
| `FINWAR_MARKET_DB_PASSWORD`           | `--db-password`      | The password used for the database authentication.                           |                         |
| `FINWAR_MARKET_DB_PASSWORD_FILE`      | `--db-password-file` | The path of the file containing the database auth password.                  |                         |
| `FINWAR_MARKET_DB_HOST`               | `--db-host`          | The host or address of the database to connect to.                           | `localhost`             |
| `FINWAR_MARKET_DB_PORT`               | `--db-port`          | The port of the database to connect to.                                      | `5432`                  |
| `FINWAR_MARKET_DB_NAME`               | `--db-name`          | The name of the database to connect to.                                      | `postgres`              |
