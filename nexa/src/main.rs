//! Nexa Gateway with UI - Main entry point
//! 
//! This is the main entry point for the Nexa Gateway application.
//! It starts both the API server and the dashboard UI.

use anyhow::Result;
use futures::future::join_all;
use std::net::SocketAddr;
use tokio::task::JoinHandle;
use tracing::{info, error};
use tracing_subscriber::{FmtSubscriber, EnvFilter};

/// Main entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    info!("Starting Nexa Gateway with UI");
    
    // Start services
    let mut handles = Vec::new();
    
    // Start the API server
    info!("Starting API server on port 3000");
    let api_handle = start_api_server();
    handles.push(api_handle);
    
    // Start the dashboard UI
    info!("Starting dashboard UI on port 3001");
    let dashboard_handle = start_dashboard_ui();
    handles.push(dashboard_handle);
    
    // Wait for all services to complete (they should run indefinitely)
    let results = join_all(handles).await;
    
    // Check if any service failed
    for result in results {
        if let Err(e) = result {
            error!("Service error: {}", e);
        }
    }
    
    Ok(())
}

/// Start the API server
fn start_api_server() -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        // Create the gateway app
        let app = gateway::create_app()?;
        
        // Start the server
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        info!("API server listening on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app)
            .await?;
        
        Ok(())
    })
}
fn start_dashboard_ui() -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        // Create the dashboard app
        let app = match dashboard::create_app() {
            Ok(app) => app,
            Err(e) => {
                error!("Failed to create dashboard app: {}", e);
                return Err(anyhow::anyhow!("Failed to create dashboard app: {}", e));
            }
        };
        
        // Start the server
        let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
        info!("Dashboard UI listening on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app.into_make_service())
            .await?;
        
        Ok(())
    })
}
