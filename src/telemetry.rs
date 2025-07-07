use std::str::FromStr;

use miette::{Error, Result, miette};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

/// # Errors
///
/// - Invalid log-level value supplied
pub fn init_tracing_subscriber(log_level: &str) -> Result<(), Error> {
    let level_filter =
        LevelFilter::from_str(log_level).map_err(|x| miette!("Invalid Log level: {:?}", x))?;
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        format!(
            "{}={level_filter},tower_http={level_filter},axum::rejection={level_filter}",
            env!("CARGO_CRATE_NAME")
        )
        .into()
    });
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter)
        .init();
    Ok(())
}
