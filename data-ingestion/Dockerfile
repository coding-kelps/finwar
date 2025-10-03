FROM docker.io/library/python:3.13.7-slim AS builder

ENV UV_COMPILE_BYTECODE=1 UV_LINK_MODE=copy UV_PYTHON_DOWNLOADS=0

COPY --from=ghcr.io/astral-sh/uv:0.8.22 /uv /uvx /bin/

WORKDIR /app

COPY pyproject.toml uv.lock ./

RUN uv sync --no-dev --locked --no-install-project

COPY . /app

RUN uv sync --no-dev --locked

FROM docker.io/library/python:3.13.7-slim AS runtime

RUN groupadd --system --gid 999 appuser \
 && useradd --system --gid 999 --uid 999 --create-home appuser

COPY --from=builder --chown=appuser:appuser /app /app
 
ENV PATH="/app/.venv/bin:$PATH"

USER appuser

WORKDIR /app

EXPOSE 4000

CMD ["dagster", "code-server", "start", "-h", "0.0.0.0", "-p", "4000", "-f", "src/finwar_data_ingestion/definitions.py"]
