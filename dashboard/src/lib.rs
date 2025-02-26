//! Nexa Gateway Control Dashboard
//! 
//! This crate provides a web-based control dashboard for the Nexa Gateway
//! built with Leptos, a reactive web framework for Rust.

use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use axum::{
    Router,
    routing::get,
    http::{Request, Response, StatusCode, Uri},
    extract::Extension,
};
use std::sync::Arc;

mod components;
mod error;
mod pages;
mod api;

/// The main dashboard component that sets up the application
#[component]
pub fn Dashboard() -> impl IntoView {
    // Set up metadata for the application
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/dashboard.css"/>
        <Title text="Nexa Gateway Dashboard"/>
        <Meta name="description" content="Control dashboard for Nexa Gateway"/>
        
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|| view! { <pages::home::HomePage/> }/>
                    <Route path="/metrics" view=|| view! { <pages::metrics::MetricsPage/> }/>
                    <Route path="/settings" view=|| view! { <pages::settings::SettingsPage/> }/>
                    <Route path="/logs" view=|| view! { <pages::logs::LogsPage/> }/>
                    <Route path="/*any" view=|| view! { <pages::not_found::NotFoundPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Configure the server-side rendering for the dashboard
#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    // Register server functions here when needed
    // _ = api::status::get_status.register();
}

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}

/// Create the dashboard application router
pub fn create_app() -> Result<Router, anyhow::Error> {
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
    
    Ok(router)
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

// Different entry points depending on whether we're building for SSR or CSR
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            mount_to_body(|| view! { <Dashboard/> });
        }
    }
    else if #[cfg(feature = "csr")] {
        pub fn main() {
            console_error_panic_hook::set_once();
            mount_to_body(|| view! { <Dashboard/> });
        }
    }
}
