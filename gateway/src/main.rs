//! Nexa Gateway - Main API server

use axum::{
    routing::{get, post},
    Router,
    extract::State,
    Json,
};
use common::{
    config::Settings,
    logging,
    middleware::{create_cors_layer, create_tracing_layer, create_compression_layer},
};
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, error};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};
use tracing_subscriber::{FmtSubscriber, EnvFilter};

mod routes;
mod handlers;
mod services;
mod error;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    config: Arc<Settings>,
    // Add other shared state like database connections, auth service, etc.
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    info!("Initializing Nexa Gateway API server");
    
    // Load configuration
    let config = Arc::new(common::Settings::new()?);
    
    let state = AppState { config };
    
    // Build our application with routes
    let app = Router::new()
        .route("/", get(routes::health_check))
        .route("/api/agents", get(routes::list_agents).post(routes::create_agent))
        .route("/api/agents/:id", get(routes::get_agent))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
