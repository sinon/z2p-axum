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