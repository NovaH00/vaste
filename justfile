CARGO_TARGET_DIR := "./.build/target"

build-all:
    CARGO_TARGET_DIR={{CARGO_TARGET_DIR}} cargo build --workspace

build-backend:
    CARGO_TARGET_DIR={{CARGO_TARGET_DIR}} cargo build -p backend

build-frontend:
    CARGO_TARGET_DIR={{CARGO_TARGET_DIR}} cargo build -p frontend
    cd frontend && trunk build --release --dist ../.build/dist

backend:
    cargo run --bin backend

frontend:
    cargo run --bin frontend

clean:
    rm -rf .build
