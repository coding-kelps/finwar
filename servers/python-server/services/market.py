from datetime import datetime
import random


def get_stock_price():
    return random.random() * datetime.now().second


def get_stock_quantity():
    return random.randint(0, 9)
