use axum::{
    routing::get,
    Router,
    extract::Extension,
    http::{Request, Response, StatusCode},
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
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
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

async fn leptos_handler(
    Extension(state): Extension<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> Response<axum::body::Body> {
    // Render the app to HTML using leptos options from state
    let html = leptos_axum::render_app_to_string(
        state.leptos_options.clone(),
        || view! { <Dashboard/> },
    );
    
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(axum::body::Body::from(html))
        .unwrap()
}