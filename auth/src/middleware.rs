//! Authentication middleware for Axum
//!
//! This module provides middleware for authenticating requests.

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use http::header::AUTHORIZATION;
use crate::{AuthService, AuthError};

/// Extract JWT token from request header
fn extract_token(request: &Request<Body>) -> Result<String, AuthError> {
    // Get the Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AuthError::MissingAuth)?
        .to_str()
        .map_err(|_| AuthError::InvalidToken)?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::InvalidToken);
    }

    // Extract the token part
    let token = auth_header.trim_start_matches("Bearer ").trim();
    
    if token.is_empty() {
        return Err(AuthError::InvalidToken);
    }
    
    Ok(token.to_string())
}

/// Authentication middleware
pub async fn auth_middleware<S>(
    State(auth_service): State<AuthService>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> 
where
    S: Send + Sync,
{
    // Skip authentication for certain paths
    let path = request.uri().path();
    if path == "/health" || path == "/api/login" {
        return Ok(next.run(request).await);
    }
    
    // Extract the token
    let token = match extract_token(&request) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Validate the token
    match auth_service.validate_token(&token).await {
        Ok(true) => {
            // Token is valid, proceed with the request
            Ok(next.run(request).await)
        }
        _ => {
            // Invalid token
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Permission check middleware
pub async fn require_permission<S>(
    State(auth_service): State<AuthService>,
    permission: &'static str,
    request: Request<Body>,
    next: Next
) -> Result<Response, StatusCode> 
where
    S: Send + Sync,
{
    // Extract the token
    let token = match extract_token(&request) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Check permission
    match auth_service.check_permission(&token, permission).await {
        Ok(true) => {
            // User has permission, proceed with the request
            Ok(next.run(request).await)
        }
        _ => {
            // Permission denied
            Err(StatusCode::FORBIDDEN)
        }
    }
}
