from dagster_duckdb import DuckDBResource
import dagster as dg
from dagster import EnvVar

from finwar_ingestion.assets import *
from finwar_ingestion.resources import *
from finwar_ingestion.constants import *

defs = dg.Definitions(
    assets=[
        download_dataset,
    ],
    resources={
        "duckdb": DuckDBResource(database=":memory:"),
        "postgresql": PostgreSQLResource(
            username=EnvVar("POSTGRES_USER"),
            password=EnvVar("POSTGRES_PASSWORD"),
            host=EnvVar("POSTGRES_HOST"),
            port=EnvVar("POSTGRES_PORT"),
            db_name=EnvVar("POSTGRES_DB"),
        ),
    },
)
