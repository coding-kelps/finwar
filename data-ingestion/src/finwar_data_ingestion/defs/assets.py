import dagster as dg
from dagster import AssetExecutionContext
from datetime import datetime
from dateutil import relativedelta
import pandas as pd

from finwar_data_ingestion.resources import PolygonResource, PostgreSQLResource

# I'll surely need to define a specific timezone to match
# the stocks market own timezone.
daily_partitions = dg.MonthlyPartitionsDefinition(start_date='2025-07-01')
# Example ticker symbol from famous stocks.
symbol_partitions = dg.StaticPartitionsDefinition(['AAPL', 'MSFT', 'NVDA', 'PLTR', 'META'])
download_partitions_matrix = dg.MultiPartitionsDefinition(
	{'date': daily_partitions, 'symbol': symbol_partitions},
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
	partition_keys = context.partition_key.keys_by_dimension
	history_start = datetime.strptime(partition_keys['date'], '%Y-%m-%d').date()
	history_end = history_start + relativedelta.relativedelta(months=1)
	symbol = partition_keys['symbol']

	with polygon.get_client() as client:
		try:
			res = client.get_aggregate_bars(
				symbol=symbol,
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
	)


@dg.asset(
	kinds={'python', 'timescaledb'},
	partitions_def=download_partitions_matrix,
	group_name='ingestion',
	code_version='0.1.0',
	description="""
    Load the extracted data to a TimescaleDB database.
  """,
)
def load_stocks_intraday_history(
	context: AssetExecutionContext,
	download_stocks_intraday_history: pd.DataFrame,
	postgresql: PostgreSQLResource,
) -> dg.MaterializeResult:
	df = download_stocks_intraday_history
	row_count = df.shape[0]

	data = [
		(
			row[0],
			row[1],
			float(row[2]),
		)
		for row in df.values
	]

	with postgresql.get_connection() as conn:
		with conn.cursor() as cur:
			cur.execute("""
				CREATE TABLE IF NOT EXISTS stock_history (
				    time TIMESTAMPTZ NOT NULL,
				    symbol TEXT NOT NULL,
				    quotation DOUBLE PRECISION
				);
				CREATE UNIQUE INDEX IF NOT EXISTS stock_history_time_symbol_idx
		    ON stock_history (time, symbol);
				SELECT create_hypertable('stock_history', 'time', if_not_exists => TRUE);
			""")

			cur.executemany(
				"""
        INSERT INTO stock_history (
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
