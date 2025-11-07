import requests
import random
import time

from .config import FINWAR_MARKET_API_ENDPOINT, BOT_NAME, BOT_UUID


def enroll():
    r = requests.post(
        f"{FINWAR_MARKET_API_ENDPOINT}/api/enroll", json={"name": BOT_NAME}
    )
    response = r.text
    print(f"Enrolled: {response}")
    uuid = response.split("UUID: ")[1].strip() if "UUID: " in response else None
    return uuid


if not BOT_UUID:
    BOT_UUID = enroll()
    if not BOT_UUID:
        print("Failed to enroll")
        exit(1)
    print(f"Using BOT_UUID: {BOT_UUID}")


def get_price():
    r = requests.get(f"{FINWAR_MARKET_API_ENDPOINT}/api/price")
    return float(r.text)


def buy(investment):
    r = requests.post(
        f"{FINWAR_MARKET_API_ENDPOINT}/api/buy",
        json={"bot_uuid": BOT_UUID, "investment": investment},
    )
    print(f"BUY: {r.text}")


def sell(quantity):
    r = requests.post(
        f"{FINWAR_MARKET_API_ENDPOINT}/api/sell",
        json={"bot_uuid": BOT_UUID, "quantity": quantity},
    )
    print(f"SELL: {r.text}")


def run():
    while True:
        try:
            action = random.choice(["buy", "sell"])

            if action == "buy":
                investment = random.uniform(300, 1000)
                buy(investment)
            else:
                quantity = random.randint(1, 5)
                sell(quantity)

            time.sleep(random.uniform(2, 10))
        except Exception as e:
            print(f"Error: {e}")
            time.sleep(5)
