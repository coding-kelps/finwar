FROM python:3.12.10-bookworm AS builder

WORKDIR /app

ENV UV_COMPILE_BYTECODE=1

RUN pip install uv==0.8.15

COPY pyproject.toml uv.lock .

RUN uv sync --no-dev --locked --no-editable

FROM python:3.12.10-slim-bookworm AS runtime

ENV VIRTUAL_ENV=/app/.venv \
    PATH="/app/.venv/bin:$PATH"

COPY --from=builder ${VIRTUAL_ENV} ${VIRTUAL_ENV}

WORKDIR /opt/dagster

RUN adduser --disabled-password --gecos "" finwar

USER finwar

EXPOSE 4000

ENTRYPOINT ["dagster", "api", "grpc", "-h", "0.0.0.0", "-p", "4000", "-m", "finwar_ingestion"]
