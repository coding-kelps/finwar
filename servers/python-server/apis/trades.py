from flask_restx import Namespace, Resource, fields
from datetime import datetime

from services.market import get_stock_price, get_stock_quantity

api = Namespace("trades", description="Trades related to stock")

trade = api.model(
    "Trade",
    {
        "total_stocks": fields.Integer(
            required=True, description="Number of stock to trade"
        ),
        "wallet": fields.Float(
            required=True, description="The money left to trade stoccks"
        ),
        "timestamp": fields.DateTime(
            required=True, description="The Datetime of the transaction"
        ),
    },
)


@api.route("/buy/<int:quantity>")
class BuyStock(Resource):
    @api.marshal_with(trade, code=201)
    def post(self, quantity):
        """Buy a stock"""
        return {
            "total_stocks": get_stock_quantity(),
            "wallet": get_stock_price(),
            "timestamp": datetime.now(),
        }, 201


@api.route("/sell/<int:quantity>")
class SellStock(Resource):
    @api.marshal_with(trade, code=201)
    def post(self, quantity):
        """Sell a stock"""
        return {
            "total_stocks": get_stock_quantity(),
            "wallet": get_stock_price(),
            "timestamp": datetime.now(),
        }, 201
