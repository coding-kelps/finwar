from dagster import ConfigurableResource
from pydantic import Field

from massive import RESTClient


class MassiveResource(ConfigurableResource):
	"""
	A Dagster resource wrapper over the massive.com REST client.
	"""

	api_key: str = Field(description='The required API key to authenticate.')

	def get_client(self) -> RESTClient:
		return RESTClient(
			api_key=self.api_key,
		)
