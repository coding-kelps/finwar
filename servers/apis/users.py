from flask_restx import Namespace, Resource, fields

from services.market import get_stock_price, get_stock_quantity

api = Namespace("users", description="Users that deal trades")

user = api.model(
    "User",
    {
        "id": fields.String(required=True, description="The stock price"),
        "name": fields.String(required=True, description="The Datetime of the stock"),
        "profit_and_losses": fields.Float(required=True, description="The stock price"),
        "wallet": fields.Float(required=True, description="The stock price"),
    },
)


@api.route("/ranking")
class GetRanking(Resource):
    @api.marshal_with(user, code=200)
    def get(self):
        """Get the current ranking of users"""

        return {
            "id": get_stock_quantity(),
            "name": "User",
            "profit_and_losses": get_stock_price(),
            "wallet": get_stock_price(),
        }, 200
