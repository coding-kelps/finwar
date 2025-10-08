# FinWar Data Ingestion

Data pipeline to ingest stock market datasets into a Time Series Database.

## Getting started

### Prerequisites

1. Make sure the Database is running (to startup the database look [here](../README.md#running-only-the-database))
2. Defines the required environment variables

```sh
export POLYGON_API_KEY=...

export POSTGRES_USER="finwar"
export POSTGRES_PASSWORD="password"
export POSTGRES_HOST="127.0.0.1"
export POSTGRES_PORT="5432"
export POSTGRES_DB="finwar"
```


> [!TIP]
> You can get a Polygon API key for free by creating an account on [polygon.io](https://polygon.io/).

### Installing dependencies

Ensure [`uv`](https://docs.astral.sh/uv/) is installed following their [official documentation](https://docs.astral.sh/uv/getting-started/installation/).

Create a virtual environment, and install the required dependencies using _sync_:

```bash
uv sync
```


### Running Dagster

#### Interactive Mode

Start the Dagster UI web server:

```bash
uv run dg dev
```

Open http://localhost:3000/asset-groups/ in your browser to see the project.

#### Through the CLI

You can also launch the pipeline directly from the CLI, as such:

```bash
uv run dg launch --assets download_stocks_intraday_history,load_stocks_intraday_history --partition 'AAPL|2025-09-21'
```

*here the pipeline is launched to collect the intraday history data from Apple Inc. on the last week of september.*

> [!WARNING]  
> The Polygon API free account only goes up to 5 req/min so beware in the number of parition you run.
