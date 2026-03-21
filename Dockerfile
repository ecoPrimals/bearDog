# BearDog Production Dockerfile
# Multi-stage build for optimized production container

# Build stage
FROM rust:1.93-slim AS builder

# Install minimal build dependencies (pure Rust — no C libs needed)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency manifests first for better caching
COPY Cargo.toml Cargo.lock ./
COPY crates/*/Cargo.toml ./crates/*/

# Create dummy main files for dependency building
RUN find crates -name Cargo.toml -exec dirname {} \; | \
    xargs -I {} mkdir -p {}/src && \
    find crates -name Cargo.toml -exec dirname {} \; | \
    xargs -I {} touch {}/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --workspace

# Copy source code
COPY . .

# Build the actual application with all optimizations
RUN cargo build --release --workspace --features="production,quantum-resistant,simd"

# Runtime stage
FROM debian:bookworm-slim

# Install minimal runtime dependencies (pure Rust binary — no C libs)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd -r beardog && useradd -r -g beardog beardog

# Create application directories
RUN mkdir -p /app/configs /app/logs /app/data && \
    chown -R beardog:beardog /app

# Copy built binaries
COPY --from=builder /app/target/release/beardog* /app/
COPY --from=builder /app/configs/ /app/configs/

# Set ownership
RUN chown -R beardog:beardog /app

# Switch to non-root user
USER beardog

# Set working directory
WORKDIR /app

# Environment variables
ENV RUST_LOG=info
ENV BEARDOG_CONFIG_PATH=/app/configs/production.toml
ENV BEARDOG_ENABLE_QUANTUM=true
ENV BEARDOG_ENABLE_SIMD=true

# Expose ports
EXPOSE 8080 9090

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:9090/health || exit 1

# Default command
CMD ["./beardog-core"]

# Metadata
LABEL maintainer="BearDog Project <beardog@ecoprimal.io>"
LABEL version="0.9.0"
LABEL description="BearDog - ecoPrimals Cryptographic Service Provider"
LABEL org.opencontainers.image.source="https://github.com/ecoprimal/beardog"
LABEL org.opencontainers.image.documentation="https://beardog.ecoprimal.io/docs"
LABEL org.opencontainers.image.licenses="AGPL-3.0-only"