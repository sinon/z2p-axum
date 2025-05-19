use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool, types::Uuid};
use tokio::net::TcpListener;
use z2p_axum::configuration::DatabaseSettings;
use z2p_axum::configuration::get_configuration;
use z2p_axum::telemetry::init_tracing_subscriber;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let log_level = match std::env::var("TEST_LOG_LEVEL") {
        Ok(l) => l,
        Err(_) => "off".to_string(),
    };
    init_tracing_subscriber(&log_level).unwrap();
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[cfg(test)]
pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    // Drop listener to free-up the selected port
    drop(listener);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let server = z2p_axum::startup::run(port, connection_pool.clone())
        .await
        .unwrap();
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[cfg(test)]
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database

    use secrecy::{ExposeSecret, SecretBox};
    let maintenance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: SecretBox::new(Box::new("password".to_string())),
        port: config.port.clone(),
        host: config.host.clone(),
    };
    let mut connection =
        PgConnection::connect(&maintenance_settings.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[track_caller]
#[cfg(test)]
pub fn expect_string(value: &serde_json::Value) -> &str {
    value
        .as_str()
        .unwrap_or_else(|| panic!("expected string, got {value:?}"))
}

#[track_caller]
#[cfg(test)]
pub fn expect_uuid(value: &serde_json::Value) -> Uuid {
    expect_string(value)
        .parse::<Uuid>()
        .unwrap_or_else(|e| panic!("failed to parse UUID from {value:?}: {e}"))
}
