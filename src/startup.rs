use std::net::SocketAddr;

use axum::{
    body::Body,
    routing::{get, IntoMakeService},
    Error, Router,
};
use hyper::{server::conn::AddrIncoming, Request};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::Level;

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
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = uuid::Uuid::new_v4();
                tracing::span!(
                    Level::DEBUG,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(request_id)
                )
            }),
        )
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
