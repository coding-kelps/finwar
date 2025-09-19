from flask_restx import Api

from .stocks import api as stocks
from .trades import api as trades
from .users import api as users

api = Api(
    title="FinWar",
    version="1.0",
    description="The API of the FinWar project consisting of a trading agent battle with finantial data",
)

api.add_namespace(stocks, path="/stock")
api.add_namespace(trades, path="/trade")
api.add_namespace(users, path="/user")
