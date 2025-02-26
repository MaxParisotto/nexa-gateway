//! Main entry point for the API server

use common::config::Settings;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    common::logging::init_logging("api", "info");
    
    info!("Starting API server");
    
    // Load configuration
    let _settings = Settings::new("config/default")?;
    
    // Start the server
    info!("API server started successfully");
    
    Ok(())
}
