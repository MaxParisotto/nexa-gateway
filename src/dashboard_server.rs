//! Dashboard server implementation
//!
//! This module provides the dashboard server functionality for the Nexa Gateway.

use axum::{
    routing::{get, get_service},
    Router,
    response::Html,
    extract::Path,
    http::StatusCode,
};
use std::net::SocketAddr;
use std::path::{Path as FilePath, PathBuf};
use tower_http::services::ServeDir;

/// Handler for the root path
async fn root() -> Html<String> {
    println!("Dashboard: Root handler called");
    
    // Look for the working dashboard in several locations
    let potential_paths = [
        "dashboard/site/working_dashboard.html",
        "dashboard/site/index.html",
        "simple_dashboard/index.html",
    ];
    
    for path in potential_paths {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                println!("Successfully loaded dashboard from {}", path);
                return Html(content);
            },
            Err(_) => continue,
        }
    }
    
    // Fallback to the embedded HTML if no file is found
    println!("Using embedded dashboard HTML");
    Html(include_str!("dashboard_template.html").to_string())
}

/// Handler for metric pages
async fn metrics() -> Html<String> {
    println!("Dashboard: Metrics handler called");
    Html(format!(r#"
        <html>
            <head>
                <title>Nexa Gateway Metrics</title>
                <style>
                    body {{ font-family: 'Segoe UI', sans-serif; margin: 20px; max-width: 800px; margin: 0 auto; }}
                    h1 {{ color: #3498db; }}
                </style>
            </head>
            <body>
                <h1>Gateway Metrics</h1>
                <p>This is the metrics monitoring page.</p>
                <p>Current time: {}</p>
                <p><a href="/">Back to Dashboard</a></p>
            </body>
        </html>
    "#, chrono::Local::now().to_rfc3339()))
}

/// Handler for logs page
async fn logs() -> Html<String> {
    println!("Dashboard: Logs handler called");
    Html(format!(r#"
        <html>
            <head>
                <title>Nexa Gateway Logs</title>
                <style>
                    body {{ font-family: 'Segoe UI', sans-serif; margin: 20px; max-width: 800px; margin: 0 auto; }}
                    h1 {{ color: #3498db; }}
                    pre {{ background: #f5f5f5; padding: 10px; border-radius: 5px; overflow: auto; }}
                </style>
            </head>
            <body>
                <h1>Gateway Logs</h1>
                <p>Server time: {}</p>
                <pre>
2025-02-26T13:49:23.042Z  INFO  gateway: Starting API server
2025-02-26T13:49:23.043Z  INFO  gateway: Initializing routes
2025-02-26T13:49:23.043Z  INFO  gateway: Routes initialized
2025-02-26T13:49:23.044Z  INFO  gateway: Server listening on 0.0.0.0:3000
2025-02-26T13:49:23.044Z  INFO  gateway: Gateway ready
                </pre>
                <p><a href="/">Back to Dashboard</a></p>
            </body>
        </html>
    "#, chrono::Local::now().to_rfc3339()))
}

/// Handler for settings page
async fn settings() -> Html<String> {
    println!("Dashboard: Settings handler called");
    Html(format!(r#"
        <html>
            <head>
                <title>Nexa Gateway Settings</title>
                <style>
                    body {{ font-family: 'Segoe UI', sans-serif; margin: 20px; max-width: 800px; margin: 0 auto; }}
                    h1 {{ color: #3498db; }}
                    .form-group {{ margin-bottom: 15px; }}
                    label {{ display: block; margin-bottom: 5px; }}
                    input, select {{ width: 100%; padding: 8px; border-radius: 4px; border: 1px solid #ddd; }}
                    button {{ padding: 10px 15px; background: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; }}
                </style>
            </head>
            <body>
                <h1>Gateway Settings</h1>
                <p>Server time: {}</p>
                
                <form>
                    <div class="form-group">
                        <label for="port">API Port</label>
                        <input type="number" id="port" value="3000">
                    </div>
                    
                    <div class="form-group">
                        <label for="logLevel">Log Level</label>
                        <select id="logLevel">
                            <option>info</option>
                            <option>debug</option>
                            <option>warn</option>
                            <option>error</option>
                        </select>
                    </div>
                    
                    <button type="button">Save Settings</button>
                </form>
                
                <p><a href="/">Back to Dashboard</a></p>
            </body>
        </html>
    "#, chrono::Local::now().to_rfc3339()))
}

/// Handler for custom echo
async fn echo(Path(text): Path<String>) -> Html<String> {
    println!("Dashboard: Echo handler called with: {}", text);
    Html(format!("<h1>Echo</h1><p>You said: {}</p><p><a href='/'>Back to Dashboard</a></p>", text))
}

/// Not found handler
async fn not_found() -> Html<String> {
    println!("Dashboard: Not found handler called");
    Html(format!(r#"
        <html>
            <head>
                <title>Not Found</title>
                <style>
                    body {{ font-family: 'Segoe UI', sans-serif; margin: 20px; text-align: center; }}
                    h1 {{ color: #e74c3c; font-size: 48px; margin-bottom: 10px; }}
                    a {{ color: #3498db; text-decoration: none; }}
                    .container {{ max-width: 500px; margin: 100px auto; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <h1>404</h1>
                    <h2>Page Not Found</h2>
                    <p>The page you're looking for doesn't exist.</p>
                    <p><a href="/">Return to Dashboard</a></p>
                </div>
            </body>
        </html>
    "#))
}

/// Find the best path for static files
fn find_static_dir() -> PathBuf {
    let potential_paths = [
        "dashboard/site",
        "simple_dashboard",
        "static_files",
    ];
    
    for path in potential_paths {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            println!("Using static directory: {}", path);
            return path_buf;
        }
    }
    
    // Fallback to current directory
    println!("No static directory found, using current directory");
    PathBuf::from(".")
}

/// Create the dashboard router
pub fn create_dashboard_router() -> Router {
    // Find static files directory
    let static_dir = find_static_dir();
    
    Router::new()
        .route("/", get(root))
        .route("/metrics", get(metrics))
        .route("/logs", get(logs))
        .route("/settings", get(settings))
        .route("/echo/:text", get(echo))
        .nest_service("/static", get_service(ServeDir::new(&static_dir)))
        .fallback(not_found)
}

/// Start the dashboard server
pub async fn run_dashboard_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_dashboard_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("Dashboard server listening on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
        
    Ok(())
}
