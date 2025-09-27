import dagster as dg
import pandas as pd
from dagster import AssetExecutionContext
from datetime import datetime
from typing import Literal, List

from finwar_data_ingestion.resources import PostgreSQLResource, PolygonResource


MAX_POLYGON_REQ_RETRY = 3


class StocksIntradayHistoryComponent(dg.Component, dg.Model, dg.Resolvable):
	"""Represents historical stock quotation for one or more ticker symbols.

	The data is retrieved from the [Polygon.io](https://polygon.io) API and returned as a
	:class:`pandas.DataFrame`. The data also contains the trading volume.
	It is then loaded as a TimescaleDB table.

	Attributes:
	    symbols (List[str]): The ticker symbols of the stocks to download intraday history for.
	    start_date (str): The inclusive start date of the time window (YYYY-MM-DD).
	    end_date (str | None): The exclusive end date of the time window (YYYY-MM-DD). If None,
	        the current date is used as the end date.
	    time_partitioning (Literal["daily", "weekly", "monthly"]): The partitioning frequency
	        for the data retrieval. The partitioning unit should be shorter or equal to the
	        requested time window. Default is ``"monthly"``.
	    polygon_api_key (str): API key for authenticating with the Polygon.io API.
	    precision (Literal["second", "minute", "day", "week", "month", "quarter", "year"]):
	        The granularity of the bars. Must be finer than or equal to the requested
	        time window. Default is ``"day"``.
	    adjusted (bool): Whether to return data adjusted for splits. Default is ``False``.
	    table_name (str): The name of the created TimescaleDB table. Default is ``"stocks_history"``.
	    timescaledb_username (str): The username for the TimescaleDB authentication.
	    timescaledb_password (str): The password for the TimescaleDB authentication.
	    timescaledb_host (str): The host of the TimescaleDB database.
	    timescaledb_port (str): The opened port of the TimescaleDB database.
	    timescaledb_db (str): The name of the TimescaleDB database.
	"""

	symbols: List[str]
	start_date: str
	end_date: str | None = None
	time_partitioning: Literal['daily', 'weekly', 'monthly'] = 'monthly'
	polygon_api_key: str
	precision: Literal['second', 'minute', 'day', 'week', 'month', 'quarter', 'year'] = 'day'
	adjusted: bool = False
	table_name: str = 'stocks_history'
	timescaledb_username: str
	timescaledb_password: str
	timescaledb_host: str
	timescaledb_port: str | int
	timescaledb_db: str

	@classmethod
	def get_spec(cls) -> dg.ComponentTypeSpec:
		return dg.ComponentTypeSpec(
			description=cls.__doc__,
			owners=['contact@kelps.org', 'guilhem.sante@kelps.org'],
			tags=['stocks', 'intraday', 'polygon.io', 'timescaledb'],
		)

	def build_defs(self, context: dg.ComponentLoadContext) -> dg.Definitions:
		match self.time_partitioning:
			case 'daily':
				time_partitioning = dg.DailyPartitionsDefinition(
					start_date=self.start_date, end_date=self.end_date
				)
			case 'weekly':
				time_partitioning = dg.WeeklyPartitionsDefinition(
					start_date=self.start_date, end_date=self.end_date
				)
			case 'monthly':
				time_partitioning = dg.MonthlyPartitionsDefinition(
					start_date=self.start_date, end_date=self.end_date
				)
		symbol_partitioning = dg.StaticPartitionsDefinition(self.symbols)
		partitions_matrix = dg.MultiPartitionsDefinition(
			{'time': time_partitioning, 'symbol': symbol_partitioning},
		)

		@dg.asset(
			kinds={'python'},
			group_name='ingestion',
			code_version='0.2.0',
			description="""
            Extract the stocks intraday value from the Polygon API.
          """,
			partitions_def=partitions_matrix,
		)
		def download_stocks_intraday_history(
			context: AssetExecutionContext,
			polygon: PolygonResource,
		) -> dg.MaterializeResult:
			partition_keys = context.partition_key.keys_by_dimension
			time_window = time_partitioning.time_window_for_partition_key(partition_keys['time'])
			symbol = partition_keys['symbol']

			with polygon.get_client() as client:
				for attempt in range(0, MAX_POLYGON_REQ_RETRY):
					try:
						res = client.get_aggregate_bars(
							symbol=symbol,
							multiplier=1,
							adjusted=self.adjusted,
							timespan=self.precision,
							from_date=time_window.start,
							to_date=time_window.end,
							limit=50000,  # API max limit
						)

						if res['status'] == 'ERROR' or res['results'] is not None:
							raise Exception(res['error'])
						break
					except Exception as e:
						if attempt == MAX_POLYGON_REQ_RETRY:
							raise dg.Failure(
								description=f'failed to get intraday stock data from Polygon API: {e}'
							)
						else:
							context.log.warning(
								f'polygon.io API request failed (attempt n°{attempt + 1}/{MAX_POLYGON_REQ_RETRY}): {e}',
							)

			return dg.MaterializeResult(
				value=pd.DataFrame(
					[
						{
							'timestamp': datetime.fromtimestamp(bar['t'] / 1000),
							'symbol': symbol,
							'open': bar['o'],
							'high': bar['h'],
							'low': bar['l'],
							'close': bar['c'],
							'volume': bar['v'],
						}
						for bar in res['results']
					]
				),
				metadata={
					'requests.retry': attempt,
				},
			)

		@dg.asset(
			kinds={'python', 'timescaledb'},
			group_name='ingestion',
			code_version='0.2.0',
			description="""
          Load the extracted data to a TimescaleDB database.
        """,
			partitions_def=partitions_matrix,
		)
		def load_stocks_intraday_history(
			context: AssetExecutionContext,
			download_stocks_intraday_history: pd.DataFrame,
			postgresql: PostgreSQLResource,
		) -> dg.MaterializeResult:
			df = download_stocks_intraday_history
			row_count = df.shape[0]

			data = list(df[['timestamp', 'symbol', 'open']].itertuples(index=False, name=None))

			with postgresql.get_connection() as conn:
				with conn.cursor() as cur:
					cur.execute(f"""
              CREATE TABLE IF NOT EXISTS {self.table_name} (
                  time TIMESTAMPTZ NOT NULL,
                  symbol TEXT NOT NULL,
                  quotation DOUBLE PRECISION
              );
              CREATE UNIQUE INDEX IF NOT EXISTS {self.table_name}_time_symbol_idx
              ON {self.table_name} (time, symbol);
              SELECT create_hypertable('{self.table_name}', 'time', if_not_exists => TRUE);
            """)

					cur.executemany(
						f"""
              INSERT INTO {self.table_name} (
                  time,
                  symbol,
                  quotation
              )
              VALUES (
                  %s,
                  %s,
                  %s
              )
              ON CONFLICT (time, symbol)
              DO UPDATE SET
                  quotation = EXCLUDED.quotation
            """,
						data,
					)

				conn.commit()

			return dg.MaterializeResult(
				metadata={
					'row_count': dg.MetadataValue.int(row_count),
				}
			)

		return dg.Definitions(
			assets=[
				download_stocks_intraday_history,
				load_stocks_intraday_history,
			],
			resources={
				'polygon': PolygonResource(
					api_key=self.polygon_api_key,
				),
				'postgresql': PostgreSQLResource(
					username=self.timescaledb_username,
					password=self.timescaledb_password,
					host=self.timescaledb_host,
					port=str(self.timescaledb_port),
					db_name=self.timescaledb_db,
				),
			},
		)
