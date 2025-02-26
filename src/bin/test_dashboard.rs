//! Test dashboard binary
//! 
//! This binary exists solely to test the dashboard web server functionality

use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use std::path::PathBuf;

// Simple handler that returns a basic HTML page
async fn index() -> Html<String> {
    println!("Index handler called");
    
    // Basic HTML to verify the server is working
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Dashboard Test</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1 { color: #3498db; }
        .info { background: #e9f7fe; padding: 10px; border-radius: 4px; margin: 10px 0; }
    </style>
</head>
<body>
    <h1>Test Dashboard Server</h1>
    <p>This is being served from the test_dashboard binary.</p>
    
    <div class="info">
        <h2>Server Information</h2>
        <p>Current time: <span id="time"></span></p>
    </div>
    
    <div class="info">
        <h2>Test Links</h2>
        <p>Try these links to test static file serving:</p>
        <ul>
            <li><a href="/static/test.txt">Static Text File</a></li>
            <li><a href="/static/dashboard.css">CSS File</a></li>
            <li><a href="/static/pkg/dashboard.js">JS File</a></li>
            <li><a href="/hello/world">Path Handler</a></li>
        </ul>
    </div>
    
    <script>
        // Update time
        function updateTime() {
            document.getElementById('time').textContent = new Date().toLocaleString();
        }
        updateTime();
        setInterval(updateTime, 1000);
        console.log("Test dashboard loaded");
    </script>
</body>
</html>"#;
    
    Html(html.to_string())
}

// Simple handler for path-based routes
async fn path_handler(Path(path): Path<String>) -> impl IntoResponse {
    println!("Path handler called for: {}", path);
    format!("You requested path: {}", path)
}

#[tokio::main]
async fn main() {
    println!("Starting test dashboard server");

    // Find and create the test files directory
    let site_dir = ensure_test_directory();
    println!("Using site directory: {}", site_dir.display());
    
    // Create basic files for testing
    create_test_files(&site_dir);

    // List files in directory
    match std::fs::read_dir(&site_dir) {
        Ok(entries) => {
            println!("Site directory contents:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  - {:?}", entry.path());
                }
            }
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    // Build our application with a route
    let static_service = ServeDir::new(&site_dir);
    let app = Router::new()
        .route("/", get(index))
        .route("/hello/:path", get(path_handler))
        .nest_service("/static", get_service(static_service.clone()))
        .fallback_service(get_service(static_service));

    // Run it on port 3001
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Test dashboard listening on {}", addr);
    
    // Start the server
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
        
    println!("Server started, waiting for connections");
    
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

// Ensure the test directory exists
fn ensure_test_directory() -> PathBuf {
    // Create a test directory in the current directory
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let site_dir = current_dir.join("test_site");
    
    // Create the directory and required subdirectories
    std::fs::create_dir_all(&site_dir).expect("Failed to create test site directory");
    std::fs::create_dir_all(site_dir.join("pkg")).expect("Failed to create pkg directory");
    
    site_dir
}

// Create some test files
fn create_test_files(site_dir: &PathBuf) {
    // Create a test text file
    let test_txt = site_dir.join("test.txt");
    std::fs::write(&test_txt, "This is a test file.").expect("Failed to create test.txt");
    
    // Create a simple CSS file
    let css_file = site_dir.join("dashboard.css");
    std::fs::write(&css_file, "body { font-family: Arial; color: #333; }").expect("Failed to create dashboard.css");
    
    // Create a simple JS file
    let js_file = site_dir.join("pkg").join("dashboard.js");
    std::fs::write(&js_file, "console.log('Dashboard JS loaded');").expect("Failed to create dashboard.js");
    
    // Create an index.html file
    let index_file = site_dir.join("index.html");
    let index_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>Test Index</title>
</head>
<body>
    <h1>Test Index</h1>
    <p>This is a test index.html file.</p>
</body>
</html>"#;
    std::fs::write(&index_file, index_content).expect("Failed to create index.html");
}
