# FinWar

🤖⚡️🤖 A finbot arena server.

Based on the dataset from : [Huge Stock Market Dataset](https://www.kaggle.com/datasets/borismarjanovic/price-volume-data-for-all-us-stocks-etfs/data)

## Prerequisites

- **Docker**: Required to run the database. [Install Docker](https://docs.docker.com/get-docker/)
- **Docker Compose**: Usually included with Docker Desktop. [Install Docker Compose](https://docs.docker.com/compose/install/)

## Getting Started

### With Docker Compose (Recommended)

Start both database and Rust server:

```bash
cat .env.example > .env
docker-compose up --build
```

The Rust server is now available on `http://localhost:4444`

### Running only the Database

Start the database using Docker Compose:

```bash
docker-compose up -d timescaledb
```

This will start a PostgreSQL + TimescaleDB instance on port 5432 with default credentials:

- **User**: finwar
- **Password**: password
- **Database**: finwar

To stop the database:

```bash
docker-compose down
```

### Running Servers Individually

For detailed instructions on running each server locally:

- **Rust Server**: See [servers/rust-server/README.md](servers/rust-server/README.md)
- **Python Server**: See [servers/python-server/README.md](servers/python-server/README.md)
