# Dockerfile for testing market_lightstreamer_channel example
# This helps verify that ig-client works correctly in a Docker container

FROM rust:1.90-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# Copy the entire ig-client project
COPY . .

# Build the market_lightstreamer_channel example in release mode
RUN cargo build --release --package examples_market --bin market_lightstreamer_channel

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /workspace/target/release/market_lightstreamer_channel /app/


ENTRYPOINT ["/app/market_lightstreamer_channel"]
