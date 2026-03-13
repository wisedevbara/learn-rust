# Production Dockerfile
# Multi-stage build for optimized image size

# Build stage
FROM rust:1.94.0 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY rust-toolchain.toml ./

# Create build directory
RUN mkdir -p src

# Build dependencies (cached)
RUN cargo build --release --locked || cargo build --release

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release --locked

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/app /usr/local/bin/

# Copy migrations
COPY --from=builder /app/migrations ./migrations

# Create directories for data
RUN mkdir -p /app/data && chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["app"]
