set dotenv-load
build:
    CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend
format:
        @cargo fmt --version
        cargo fmt
lint:format
        @cargo clippy --version
        cargo clippy -- -D warnings
        cargo doc --no-deps
test:
    cargo nextest run --all-targets --no-fail-fast

t:test

docker_build:
    docker build --tag z2p-axum --file Dockerfile .

docker_run:
    docker run -p 8000:8000 z2p-axum