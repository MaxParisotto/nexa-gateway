use axum::{
    routing::get,
    Router,
    extract::Extension,
    http::{Request, Response, StatusCode, Uri},
};
use dashboard::{Dashboard, register_server_functions};
use leptos::*;
use std::{sync::Arc, net::SocketAddr};
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    common::logging::init_logging("dashboard", "info");
    
    // Register server functions for SSR
    register_server_functions();
    
    // Create Leptos options
    let leptos_options = LeptosOptions::builder()
        .output_name("dashboard")
        .site_root("dashboard/site")
        .site_pkg_dir("pkg")
        .build();
    
    // Create a shared application state
    let app_state = AppState { leptos_options: leptos_options.clone() };
    
    // Set up the Axum router with Leptos integration
    let router = Router::new()
        .route("/{*path}", get(leptos_handler))
        .layer(Extension(Arc::new(app_state)));
    
    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Dashboard server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
        
    axum::serve(listener, router).await.unwrap();
}

/// Application state shared across all routes
#[derive(Clone)]
struct AppState {
    leptos_options: LeptosOptions,
}

/// Handler for Leptos routes
async fn leptos_handler(
    Extension(state): Extension<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> Response<axum::body::Body> {
    // Render the app to HTML
    let html = leptos::ssr::render_to_string(|| view! { <Dashboard/> });
    
    // Return the HTML response
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(axum::body::Body::from(html.to_string()))
        .unwrap()
}
