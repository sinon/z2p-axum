use std::{net::SocketAddr, str::FromStr};

use miette::{IntoDiagnostic, Result};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use z2p_axum::{
    configuration::get_configuration, startup::generate_routes, telemetry::init_tracing_subscriber,
};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing_subscriber("info")?;
    tracing::debug!("Retrieving service configuration...");
    let configuration = get_configuration().expect("Failed to read configuration.");
    tracing::debug!("Connecting to postgres");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool.");
    tracing::debug!("Running DB migrations");
    sqlx::migrate!()
        .run(&connection_pool)
        .await
        .into_diagnostic()?;

    let router = generate_routes(connection_pool);
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let addr = SocketAddr::from_str(&address).expect("Failed to connect to address");
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
