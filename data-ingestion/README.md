# FinWar Data Ingestion

Data pipeline to ingest stock market datasets into a Time Series Database.

## Getting started

### Defines the required environment variables

```sh
export POLYGON_API_KEY=...

export POSTGRES_USER="finwar-data-ingestion"
export POSTGRES_PASSWORD="MyPassword123"
export POSTGRES_HOST="127.0.0.1"
export POSTGRES_PORT="5432"
export POSTGRES_DB="market"
```


> [!TIP]
> You can get a Polygon API key for free by creating an account on [polygon.io](https://polygon.io/).

### Installing dependencies

Ensure [`uv`](https://docs.astral.sh/uv/) is installed following their [official documentation](https://docs.astral.sh/uv/getting-started/installation/).

Create a virtual environment, and install the required dependencies using _sync_:

```bash
uv sync
```

### Run the database

```bash
docker compose up -d
```

### Running Dagster

Start the Dagster UI web server:

```bash
uv run dg dev
```

Open http://localhost:3000 in your browser to see the project.
