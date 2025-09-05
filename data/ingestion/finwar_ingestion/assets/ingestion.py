import dagster as dg
from dagster_duckdb import DuckDBResource
from dagster import AssetExecutionContext

@dg.asset(
    kinds={"python", "duckdb"},
    group_name="ingestion",
    code_version="0.1.0",
    description="""
    """,
    tags = {},
)
def download_dataset(context: AssetExecutionContext, duckdb: DuckDBResource) -> dg.MaterializeResult:
    return dg.MaterializeResult(
        metadata={
        }
    )
