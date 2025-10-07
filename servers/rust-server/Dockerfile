FROM rust:1.89-slim-bookworm AS builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY local ./local
COPY templates ./templates
COPY entity ./entity
COPY migration ./migration
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/finwar-market /app/finwar-market
COPY templates ./templates
COPY local ./local

ENV RUST_LOG=info

EXPOSE 4444















CMD ["/app/finwar-market"]
 