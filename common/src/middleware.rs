//! Common middleware for Axum applications.

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    cors::{CorsLayer, Any},
    compression::CompressionLayer,
};
use tracing::Level;

/// Create a CORS middleware layer with default settings.
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

/// Create a tracing middleware layer for request/response logging.
pub fn create_tracing_layer() -> TraceLayer<Request> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

/// Create a compression middleware layer.
pub fn create_compression_layer() -> CompressionLayer {
    CompressionLayer::new()
}

/// Authentication middleware that extracts JWT token from Authorization header.
/// 
/// This is a placeholder - actual implementation would use the auth crate.
pub async fn auth_middleware<T>(
    State(_state): State<Arc<T>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Get the authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(header[7..].to_string())
            } else {
                None
            }
        });

    // If there's a token, add it to request extensions
    if let Some(token) = auth_header {
        request.extensions_mut().insert(token);
    }

    next.run(request).await
}

// Define the interface expected from the auth service
pub trait AuthServiceTrait {
    // Add other auth methods as needed
}

// Import actual auth service implementation from the auth crate
// This will be implemented later
