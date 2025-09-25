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
docker-compose up -d

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
- Set `DATABASE_URL=postgres://finwar:password@localhost/finwar` 
- Default server runs on `0.0.0.0:4444`
- Stock data loaded from `./local/data/Stocks/` directory

### Running the Server
```bash
cargo run  # Runs migrations automatically on startup
```