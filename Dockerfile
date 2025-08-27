# Multi-stage build for Allfeat Faucet
# Stage 1: Build the frontend with Rust and Trunk
FROM rust:1.89-slim AS frontend-builder

# Install Node.js (needed for Tailwind CSS)
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Add wasm target and install trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app

# Copy ALL workspace files first
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY shared/ ./shared/
COPY frontend/ ./frontend/
COPY backend/ ./backend/

# Install frontend dependencies and build
WORKDIR /app/frontend
RUN npm install
RUN trunk build --release

# Stage 2: Build the backend
FROM rust:1.89-slim AS backend-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy ALL workspace files
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY shared/ ./shared/
COPY backend/ ./backend/
COPY frontend/ ./frontend/

# Copy the built frontend from previous stage
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

# Build backend in release mode
RUN cargo build --release --manifest-path backend/Cargo.toml

# Stage 3: Runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary and frontend assets
COPY --from=backend-builder /app/target/release/allfeat-faucet-backend ./allfeat-faucet-backend
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist
COPY --from=backend-builder /app/backend/melodie_metadata.scale ./melodie_metadata.scale

# Create a non-root user
RUN useradd -r -s /bin/false faucet
RUN chown -R faucet:faucet /app
USER faucet

# Expose the application port
EXPOSE 3000

# Run the backend server
CMD ["./allfeat-faucet-backend"]
