//! Logging configuration and utilities.

use std::str::FromStr;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
    layer::{SubscriberExt, Layer},
    util::SubscriberInitExt,
};

use crate::config::Settings;

/// Initialize the logging system.
///
/// # Arguments
/// * `service_name` - Name of the service for logging identification.
/// * `default_level` - Default log level if not specified in environment.
pub fn init_logging(service_name: &str, default_level: &str) {
    // Allow setting log level via RUST_LOG environment variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "{}={},tower_http=info,axum=info",
            service_name,
            default_level
        ))
    });

    // Parse log level for console output formatting
    let log_level = Level::from_str(default_level).unwrap_or(Level::INFO);

    // Configure and install the tracing subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_ansi(true),
        )
        .init();

    info!(
        service = service_name,
        level = %log_level,
        "Logging initialized"
    );
}

pub fn setup_logging(_config: &Settings) -> Result<(), String> {
    // Default to INFO level
    let level = Level::INFO;
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("{}", level)));
        
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .boxed();
        
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|e| format!("Failed to set up logging: {}", e))?;
    
    Ok(())
}

pub fn log_error(err: &dyn std::error::Error) {
    error!(error = %err, "Operation failed");
}

pub fn log_info(message: &str) {
    info!(message);
}

pub fn log_warning(message: &str) {
    warn!(message);
}

pub fn log_debug(message: &str) {
    debug!(message);
}
