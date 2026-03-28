# SPDX-License-Identifier: AGPL-3.0-only
# BearDog Production Dockerfile — multi-stage, pure Rust

FROM rust:1.93-slim AS builder

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates/ crates/
COPY src/ src/
COPY benchmarks/ benchmarks/
COPY tests/ tests/

RUN cargo build --release --bin beardog

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN groupadd -r beardog && useradd -r -g beardog beardog
RUN mkdir -p /app && chown beardog:beardog /app

COPY --from=builder /app/target/release/beardog /app/beardog
RUN chown beardog:beardog /app/beardog

USER beardog
WORKDIR /app

ENV RUST_LOG=info

ENTRYPOINT ["./beardog"]
CMD ["server"]

LABEL maintainer="ecoPrimals"
LABEL version="0.9.0"
LABEL description="BearDog — Pure Rust Cryptographic Service Provider"
LABEL org.opencontainers.image.licenses="AGPL-3.0-only"
