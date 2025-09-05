# finwar-ingestion

Data pipeline to ingest stock market datasets into a Time Series Database.


## Requirements

To work locally with this project you will need to install [uv](https://docs.astral.sh/uv/)
in `0.8.15`, you can install it with `pip` as such:

```sh
pip install uv==0.8.15
```

## Working with the local environment

You can setup the local environment with `docker compose` as such:

```sh
cd docker_compose
docker compose up
```

You can then access at the Dagster WebUI at http://localhost:3000.

> [!TIP]
> The Docker compose environment is made so that the code is mounted in the container as a volume. You don't need to restart the container nor the whole docker compose to observe a code change in the Dagster WebUI (kind of like a Hot reload for a frontend server).
