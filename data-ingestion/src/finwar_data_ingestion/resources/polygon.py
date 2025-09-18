from dagster import ConfigurableResource
from pydantic import Field

from polygon import StocksClient
from polygon.stocks.stocks import SyncStocksClient


class PolygonResource(ConfigurableResource):
	"""
	A Dagster resource wrapper over the polygon.io REST client.
	"""

	api_key: str = Field(description='The required API key to authenticate.')

	def get_client(self) -> SyncStocksClient:
		return StocksClient(
			api_key=self.api_key,
			use_async=False,
		)
