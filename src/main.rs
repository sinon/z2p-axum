use std::net::SocketAddr;

use miette::{IntoDiagnostic, Result};
use newsletter_api::{
    configuration::get_configuration, startup::generate_routes, telemetry::init_tracing_subscriber,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing_subscriber();
    tracing::debug!("Retrieving service configuration...");
    let configuration = get_configuration().expect("Failed to read configuration.");
    tracing::debug!("Connecting to postgres");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .into_diagnostic()?;
    tracing::debug!("Running DB migrations");
    sqlx::migrate!()
        .run(&connection_pool)
        .await
        .into_diagnostic()?;

    let router = generate_routes(connection_pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
