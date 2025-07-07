use axum::{Router, body::Body, routing::get};
use hyper::Request;
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
