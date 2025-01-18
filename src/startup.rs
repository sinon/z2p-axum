use std::net::SocketAddr;

use axum::{
    routing::{get, IntoMakeService},
    Error, Router,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::routes::{create_subscriber, health_check};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn generate_routes(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/healthcheck", get(health_check))
        .route("/api/subscriber", axum::routing::post(create_subscriber))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

pub async fn run(
    port: u16,
    db_pool: PgPool,
) -> Result<axum::Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server: hyper::Server<hyper::server::conn::AddrIncoming, IntoMakeService<Router>> =
        axum::Server::bind(&addr).serve(generate_routes(db_pool).into_make_service());
    Ok(server)
}
