# Rando Bot

## Overview

The Rando Bot is an example trading bot for the FinWar Market API.

## Quickstart

### Run the bot

#### With `uv`

```sh
uv sync
uv run finwar_rando
```

### Build the container image

```sh
docker build -f docker/Dockerfile -t finwar/rando-bot .
```

## Configuration

| Name                         | Description                                        | Default                 |
| ---------------------------- | -------------------------------------------------- | ----------------------- |
| FINWAR_MARKET_API_ENDPOINT   | The endpoint of the FinWar Market API.             | `http://localhost:4444` |
