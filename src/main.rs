use dashboard::dashboard_app;

// Add the dashboard server module
mod dashboard_server;

// Update the dashboard server start function
async fn start_dashboard_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Use our new dashboard implementation
    info!("Starting dashboard server on port {}", port);
    dashboard_server::run_dashboard_server(port).await
}
