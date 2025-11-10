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

- Set `DATABASE_URL=postgres://finwar:password@localhost/finwar` as an env variable or in a .env
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
docker run -p 4444:4444 -e DATABASE_URL=postgresql://finwar:password@host.docker.internal:5432/finwar finwar-rust-server
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

| Name                                  | Description                                                           | Default                 |
| :------------------------------------ | :-------------------------------------------------------------------- | :---------------------: |
| FINWAR_MARKET_ADDR                    | The HTTP server address.                                              | `0.0.0.0`               |
| FINWAR_MARKET_PORT                    | The HTTP server port                                                  | `4444`                  |
| FINWAR_MARKET_INTERVAL_SECONDS        | The internal time passed (in seconds) in the Market each real second. | `60`                    |
| FINWAR_MARKET_TARGET_SYMBOL           | The target stock symbol on which the Market simulation is based on.   | `AAPL`                  |
| FINWAR_MARKET_DATABASE_URL            | The URL with authentication information of the backend database.      |                         |
