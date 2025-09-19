from flask_restx import Namespace, Resource, fields
from datetime import datetime

from services.market import get_stock_price

api = Namespace("stocks", description="Stocks related operations")
stock = api.model(
    "Stock",
    {
        "price": fields.Float(required=True, description="The stock price"),
        "timestamp": fields.DateTime(
            required=True, description="The Datetime of the stock"
        ),
    },
)


@api.route("/")
class StockPrice(Resource):
    @api.marshal_with(stock, code=200)
    def get(self):
        """Get current price of a stock"""
        return {
            "price": get_stock_price(),
            "timestamp": datetime.now(),
        }, 200
