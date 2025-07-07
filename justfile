serve-frontend:
    cd frontend && trunk serve --open

start-backend:
    cargo build --release --manifest-path backend/Cargo.toml
    ./target/release/backend

build-frontend:
    cd frontend && trunk build --release

start:
    cd frontend && trunk build --release
    cargo run --release --manifest-path backend/Cargo.toml
    ./target/release/backend

dev:
    cd frontend && trunk build
    cargo run --manifest-path backend/Cargo.toml
    ./target/debug/backend

format:
    leptosfmt .
    cargo fmt --all
