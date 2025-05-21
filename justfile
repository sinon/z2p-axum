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
    DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter 
    cargo nextest run --all-targets --no-fail-fast

t:test

init_db:
    sh ./scripts/init_db.sh

migrate:
	DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo sqlx migrate run
run:
	DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo run

docker_build:
    docker build --tag z2p-axum --file Dockerfile .

docker_run:
    docker run -p 8000:8000 z2p-axum