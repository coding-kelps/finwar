from pathlib import Path

import dagster as dg
from dagster_duckdb import DuckDBResource

from finwar_data_ingestion.resources import PolygonResource, PostgreSQLResource


@dg.definitions
def defs() -> dg.Definitions:
	return dg.Definitions.merge(
		dg.Definitions(
			resources={
				'polygon': PolygonResource(
					api_key=dg.EnvVar('POLYGON_API_KEY'),
				),
				'postgresql': PostgreSQLResource(
					username=dg.EnvVar('POSTGRES_USER'),
					password=dg.EnvVar('POSTGRES_PASSWORD'),
					host=dg.EnvVar('POSTGRES_HOST'),
					port=dg.EnvVar('POSTGRES_PORT'),
					db_name=dg.EnvVar('POSTGRES_DB'),
				),
				'duckdb': DuckDBResource(database='./stocks.duckdb'),
			}
		),
		dg.load_from_defs_folder(project_root=Path(__file__).parent.parent),
	)
