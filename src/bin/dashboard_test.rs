//! Dashboard test application
//!
//! A simple test binary to verify the dashboard server works correctly.

use axum::{
    routing::get,
    Router,
    response::Html,
    extract::Path,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Starting dashboard test server");

    // Create a basic router with HTML responses
    let app = Router::new()
        .route("/", get(|| async { 
            Html(r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Dashboard Test</title>
                    <style>
                        body { 
                            font-family: Arial, sans-serif; 
                            max-width: 800px; 
                            margin: 0 auto; 
                            padding: 20px;
                        }
                        h1 { color: #3498db; }
                        .card {
                            border: 1px solid #ddd;
                            padding: 15px;
                            margin: 15px 0;
                            border-radius: 5px;
                            background: #f9f9f9;
                        }
                        ul { padding-left: 20px; }
                    </style>
                </head>
                <body>
                    <h1>Dashboard Test Server</h1>
                    <p>This is a simple test server to verify that Axum works correctly.</p>
                    
                    <div class="card">
                        <h2>Status</h2>
                        <p>The server is running.</p>
                        <p>Current time: <span id="time"></span></p>
                    </div>
                    
                    <div class="card">
                        <h2>Test Links</h2>
                        <ul>
                            <li><a href="/hello">Hello Test</a></li>
                            <li><a href="/echo/world">Echo Test</a></li>
                        </ul>
                    </div>
                    
                    <script>
                        document.getElementById('time').textContent = new Date().toLocaleString();
                        console.log("Dashboard test server loaded");
                    </script>
                </body>
                </html>
            "#)
        }))
        .route("/hello", get(|| async {
            Html("<h1>Hello!</h1><p>This is a simple test page.</p><p><a href='/'>Back</a></p>")
        }))
        .route("/echo/:text", get(|Path(text): Path<String>| async move {
            Html(format!("<h1>Echo</h1><p>You said: {}</p><p><a href='/'>Back</a></p>", text))
        }));

    // Bind to a specific port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on http://{}", addr);
    
    // Start the server
    match axum::Server::bind(&addr).serve(app.into_make_service()).await {
        Ok(_) => println!("Server shutdown gracefully"),
        Err(e) => eprintln!("Server error: {}", e),
    }
}
