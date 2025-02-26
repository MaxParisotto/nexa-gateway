//! Standalone dashboard server
//!
//! A simple HTTP server that serves static files and basic HTML.

use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use std::path::PathBuf;

// Handler for the root path
async fn root() -> Html<String> {
    println!("Root handler called");
    Html(r#"<!DOCTYPE html>
<html>
<head>
    <title>Standalone Dashboard</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; max-width: 800px; line-height: 1.6; }
        h1 { color: #3498db; }
        .container { border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin: 20px 0; }
    </style>
</head>
<body>
    <h1>Standalone Dashboard Server</h1>
    <p>This is a test server to verify the dashboard functionality.</p>
    
    <div class="container">
        <h2>Server Status</h2>
        <p>The standalone server is running.</p>
        <p>Current time: <span id="current-time"></span></p>
    </div>
    
    <div class="container">
        <h2>Test Links</h2>
        <ul>
            <li><a href="/echo/hello">Echo Test</a></li>
            <li><a href="/static/test.txt">Static File</a></li>
        </ul>
    </div>
    
    <script>
        document.getElementById('current-time').textContent = new Date().toLocaleString();
    </script>
</body>
</html>"#.to_string())
}

// Handler for a simple echo endpoint
async fn echo(Path(text): Path<String>) -> impl IntoResponse {
    format!("You said: {}", text)
}

// Create static test files
fn create_test_files(dir: &std::path::Path) {
    // Create a test.txt file
    let test_file = dir.join("test.txt");
    std::fs::write(test_file, "This is a test file.").expect("Failed to write test.txt");
    
    // Create an index.html file
    let index_file = dir.join("index.html");
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Static Index</title>
</head>
<body>
    <h1>Static File Server</h1>
    <p>This is the index.html file served from the static directory.</p>
</body>
</html>"#;
    std::fs::write(index_file, html).expect("Failed to write index.html");
}

#[tokio::main]
async fn main() {
    // Create a directory for static files
    let static_dir = PathBuf::from("static_files");
    std::fs::create_dir_all(&static_dir).expect("Failed to create static directory");
    create_test_files(&static_dir);
    
    // Log some information
    println!("Starting standalone dashboard server");
    println!("Static files directory: {:?}", static_dir);
    
    // Create a router with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/echo/:text", get(echo))
        .nest_service("/static", get_service(ServeDir::new(&static_dir)));
    
    // Create a socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    println!("Listening on http://{}", addr);
    
    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
