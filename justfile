set dotenv-load
build:
    CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend
format:
        @cargo fmt --version
        cargo fmt
lint:
        @cargo clippy --version
        cargo clippy -- -D warnings
        cargo doc
test:
    cargo nextest run --all-targets --no-fail-fast

t:test

lox_run:build
    cargo run run test.lox 

docker_build:
    docker build --tag z2p-axum --file Dockerfile .

docker_run:
    docker run -p 8000:8000 z2p-axum