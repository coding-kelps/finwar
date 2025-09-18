import dagster as dg
from dagster import AssetExecutionContext
from dagster_duckdb import DuckDBResource
from datetime import datetime
from dateutil import relativedelta
import pandas as pd

from finwar_data_ingestion.resources import PolygonResource

# I'll surely need to define a specific timezone to match
# the stocks market own timezone.
daily_partitions = dg.MonthlyPartitionsDefinition(start_date='2025-07-01')
# Example tickers from famous stocks.
ticker_partitions = dg.StaticPartitionsDefinition(['AAPL', 'MSFT', 'NVDA', 'PLTR', 'META'])
download_partitions_matrix = dg.MultiPartitionsDefinition(
	{'date': daily_partitions, 'ticker': ticker_partitions},
)


@dg.asset(
	kinds={'python'},
	group_name='ingestion',
	code_version='0.1.0',
	description="""
    Extract the stocks intraday value from the Polygon API.
  """,
	tags={'polygon.io': ''},
	partitions_def=download_partitions_matrix,
)
def download_stocks_intraday_history(
	context: AssetExecutionContext,
	polygon: PolygonResource,
) -> pd.DataFrame:
	partition_keys: dg.MultiPartitionKey = context.partition_key.keys_by_dimension

	history_start = datetime.strptime(partition_keys['date'], '%Y-%m-%d').date()
	history_end = history_start + relativedelta.relativedelta(months=1)
	ticker = partition_keys['ticker']

	with polygon.get_client() as client:
		try:
			res = client.get_aggregate_bars(
				symbol=ticker,
				multiplier=1,
				adjusted=False,
				timespan='minute',
				from_date=history_start,
				to_date=history_end,
				limit=50000,  # API max limit
			)

			if res['status'] == 'ERROR':
				raise Exception(res['error'])
		except Exception as e:
			raise dg.Failure(
				description=f'failed to get intraday stock data from Polygon API (range {history_start} - {history_end}): {e}'
			)

	return pd.DataFrame(
		[
			{
				'ticker': ticker,
				'timestamp': datetime.fromtimestamp(bar['t'] / 1000),
				'open': bar['o'],
				'high': bar['h'],
				'low': bar['l'],
				'close': bar['c'],
				'volume': bar['v'],
			}
			for bar in res['results']
		]
	)


@dg.asset(
	kinds={'python', 'duckdb'},
	partitions_def=download_partitions_matrix,
	group_name='ingestion',
	code_version='0.1.0',
	description="""
    Load the extracted data to duckdb.
  """,
)
def load_stocks_intraday_history(
	context: AssetExecutionContext,
	download_stocks_intraday_history: pd.DataFrame,
	duckdb: DuckDBResource,
) -> dg.MaterializeResult:
	df = download_stocks_intraday_history
	row_count = df.shape[0]

	with duckdb.get_connection() as con:
		con.execute("""
		CREATE TABLE IF NOT EXISTS minute_bars (
		    ticker TEXT,
		    timestamp TIMESTAMP,
		    open DOUBLE,
		    high DOUBLE,
		    low DOUBLE,
		    close DOUBLE,
		    volume BIGINT
		)
		""")

		con.execute('INSERT INTO minute_bars SELECT * FROM df')

	return dg.MaterializeResult(
		metadata={
			'row_count': dg.MetadataValue.int(row_count),
		}
	)
