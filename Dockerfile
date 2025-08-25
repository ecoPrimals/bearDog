# BearDog Production Container
# Multi-stage build for optimal security and performance

# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/beardog

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY benchmarks/ ./benchmarks/

# Build dependencies (cached layer)
RUN cargo build --release --workspace

# Copy source code
COPY . .

# Build the application
RUN cargo build --release --bin beardog-cli

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false beardog

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /usr/src/beardog/target/release/beardog-cli /usr/bin/beardog-cli

# Copy configuration files
COPY configs/ ./configs/
COPY docs/ ./docs/

# Create necessary directories
RUN mkdir -p /app/logs /app/data \
    && chown -R beardog:beardog /app

# Switch to non-root user
USER beardog

# Expose ports
EXPOSE 8080 8443

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD beardog-cli health-check || exit 1

# Default command
CMD ["beardog-cli", "server", "--config", "/app/configs/beardog-config.toml"]

# Metadata
LABEL maintainer="BearDog Project <beardog@ecoprimal.io>"
LABEL version="1.0.0"
LABEL description="BearDog - Secure Decentralized Cryptographic Infrastructure"
LABEL org.opencontainers.image.source="https://github.com/ecoprimal/beardog"
LABEL org.opencontainers.image.documentation="https://beardog.ecoprimal.io/docs"
LABEL org.opencontainers.image.licenses="MIT" 