# Highly optimized multi-stage build for Allfeat Faucet
FROM rust:1.89-slim AS builder

# Install system dependencies in one layer
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential

# Install Rust toolchain in separate layer for caching
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-binstall

RUN cargo binstall trunk

WORKDIR /app

# Copy manifests first (best cache layer)
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY shared/Cargo.toml ./shared/
COPY frontend/Cargo.toml ./frontend/
COPY backend/Cargo.toml ./backend/

# Now copy actual source code
WORKDIR /app
COPY shared/src/ ./shared/src/
COPY frontend/src/ ./frontend/src/
COPY frontend/index.html ./frontend/
COPY frontend/Trunk.toml ./frontend/
COPY frontend/styles.css ./frontend/
COPY frontend/tailwind.config.js ./frontend/
COPY frontend/public/ ./frontend/public/
COPY backend/src/ ./backend/src/
COPY backend/melodie_metadata.scale ./backend/

# Build frontend
WORKDIR /app/frontend
RUN trunk build --release

# Build backend
WORKDIR /app
RUN cargo build --release --manifest-path backend/Cargo.toml

# Runtime stage
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy artifacts
COPY --from=builder /app/target/release/allfeat-faucet-backend ./allfeat-faucet-backend
COPY --from=builder /app/frontend/dist ./frontend/dist
COPY --from=builder /app/backend/melodie_metadata.scale ./melodie_metadata.scale

# Security: non-root user
RUN useradd -r -s /bin/false faucet \
    && chown -R faucet:faucet /app

USER faucet

EXPOSE 3000

CMD ["./allfeat-faucet-backend"]
