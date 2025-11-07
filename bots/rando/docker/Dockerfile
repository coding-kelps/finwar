FROM docker.io/library/python:3.13.7-slim AS builder

ENV UV_COMPILE_BYTECODE=1 UV_LINK_MODE=copy UV_PYTHON_DOWNLOADS=0

COPY --from=ghcr.io/astral-sh/uv:0.9.6 /uv /uvx /bin/

WORKDIR /app

COPY pyproject.toml uv.lock ./

RUN uv sync --no-dev --locked --no-install-project

COPY . /app

RUN uv sync --no-dev --no-editable --locked

FROM docker.io/library/python:3.13.7-slim AS runner

LABEL org.opencontainers.image.authors="guilhem.sante@kelps.org,louis.piochaud@kelps.org"
LABEL org.opencontainers.image.title="FinWar Rando Trading Bot"
LABEL org.opencontainers.image.description="Example trading bot acting randomly in the FinWar market."
LABEL org.opencontainers.image.licenses="MIT"

RUN groupadd --system --gid 999 appuser \
 && useradd --system --gid 999 --uid 999 --create-home appuser

COPY --from=builder --chown=appuser:appuser /app/.venv /app/.venv

USER appuser

WORKDIR /app

ENTRYPOINT ["/app/.venv/bin/finwar_rando"]
