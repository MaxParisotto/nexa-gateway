//! Gateway crate for Nexa AI Orchestration Platform.
//!
//! This crate provides the main REST API server functionality for the platform.

pub mod error;
pub mod routes;
pub mod middleware;
pub mod state;
pub mod handlers;
pub mod services;
pub mod status;
pub mod agent;
pub mod logs;
pub mod config;
pub mod llm;
// Remove device module reference as it's not relevant to the project
// pub mod device;

// Add the tests module
#[cfg(test)]
pub mod tests;

// Re-export common modules that might be needed
pub use common;

// Export error type from the error module
pub use error::AppError as GatewayError;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    // Add config to state
    pub config: Arc<common::config::Settings>,
    // Add other shared state here as needed
}

/// Create the gateway application router
pub fn create_app() -> Result<Router, anyhow::Error> {
    tracing::info!("Creating gateway application");
    
    // Load configuration from common crate
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/default".to_string());
    let settings = common::config::Settings::new(config_path)
        .unwrap_or_else(|_| {
            // Create default settings as fallback
            common::config::Settings {
                environment: "development".to_string(),
                auth: common::config::AuthConfig {
                    jwt_secret: "dev-secret-key".to_string(),
                    jwt_expiration: 24,
                },
                server: common::config::ServerSettings {
                    host: "127.0.0.1".to_string(),
                    port: 8000,
                },
                database: common::config::DatabaseSettings {
                    url: "postgres://postgres:postgres@localhost:5432/nexa_dev".to_string(),
                    max_connections: 10,
                },
                agora: common::config::AgoraSettings {
                    host: "127.0.0.1".to_string(),
                    port: 9000,
                },
            }
        });
    
    // Initialize the app state
    let state = AppState {
        config: Arc::new(settings),
    };
    
    // Build the router with routes
    let app = Router::new()
        .route("/", axum::routing::get(routes::health_check))
        .route("/health", axum::routing::get(routes::health_check))
        .route("/api/agents", axum::routing::get(routes::list_agents).post(routes::create_agent))
        .route("/api/agents/{id}", axum::routing::get(routes::get_agent))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);
    
    Ok(app)
}

/// Initialize the gateway server
#[tracing::instrument]
pub async fn start_server(settings: &common::config::Settings) {
    let span = tracing::info_span!("gateway_startup");
    let _enter = span.enter();
    
    // Initialize logging from common crate
    common::logging::init_logging("gateway", "info");
    tracing::info!("Initializing gateway server");
    
    // Create the application
    let app = create_app().expect("Failed to create gateway application");
    
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    tracing::info!(%addr, "Binding server to address");
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
        
    axum::serve(listener, app)
        .await
        .unwrap();
    
    tracing::info!("Gateway server started successfully");
}
