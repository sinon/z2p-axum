# Newsletter API

Following along with the Zero to Production but using `axum` instead of `actix-web`.

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres


docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=password POSTGRES_DB=newsletter -d postgres
DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo sqlx migrate run
DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo sqlx prepare

DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo test
DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo run
```

## Debugging

`RUST_LOG="info,tower_http=debug"`RUST_LOG="info,tower_http=debug"`
