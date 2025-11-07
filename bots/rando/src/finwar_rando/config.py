import os
import random

FINWAR_MARKET_API_ENDPOINT = os.getenv(
    "FINWAR_MARKET_API_ENDPOINT", "http://localhost:4444"
)
BOT_NAME = "Rando-" + str(random.randint(1000, 9999))
BOT_UUID = ""
