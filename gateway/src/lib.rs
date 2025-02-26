//! Main gateway module for NexaAgents REST API

pub mod error;
pub mod routes;
pub mod middleware;
pub mod state;

use hyper::Server;

pub use error::GatewayError;

use axum::Router;
use std::net::SocketAddr;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    // Add shared state here
}

/// Initialize the gateway server
#[tracing::instrument]
pub async fn start_server(settings: &common::config::Settings) {
    let span = tracing::info_span!("gateway_startup");
    let _enter = span.enter();
    
    // Initialize logging from common crate
    common::logging::init_logging(settings);
    tracing::info!("Initializing gateway server");
    
    let app = Router::new()
        .with_state(AppState {});
    
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    tracing::info!(%addr, "Binding server to address");
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    
    tracing::info!("Gateway server started successfully");
}
