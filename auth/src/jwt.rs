//! JWT token management for authentication
//!
//! This module handles JWT token generation, validation, and decoding.

use crate::error::AuthError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Claims structure for JWT tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Role for permission checking
    pub role: String,
    /// Issuer
    pub iss: String,
    /// Username for display
    pub username: String,
    /// Issued at timestamp
    pub iat: i64,
    /// Expiration timestamp
    pub exp: i64,
}

/// Validate a JWT token
pub async fn validate_token(token: &str, secret: &str) -> Result<bool, AuthError> {
    // Decode and verify the token
    let _ = decode_token(token, secret).await?;
    
    // If no error was thrown during decoding, the token is valid
    Ok(true)
}

/// Generate a JWT token
pub async fn generate_token(user_id: &str, role: &str, username: &str, secret: &str, expiry_hours: u64) -> Result<String, AuthError> {
    let now = Utc::now();
    let expiry = now + Duration::hours(expiry_hours as i64);
    
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        iss: "nexa-gateway".to_string(),
        username: username.to_string(),
        iat: now.timestamp(),
        exp: expiry.timestamp(),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreationError)?;
    
    Ok(token)
}

/// Decode a JWT token
pub async fn decode_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _ => AuthError::InvalidToken,
    })?;
    
    Ok(token_data.claims)
}

/// Simplified JWT token creation for tests
pub fn create_jwt(user_id: &str, username: &str) -> Result<String, AuthError> {
    let now = Utc::now();
    let expiry = now + Duration::hours(24);
    
    let claims = Claims {
        sub: user_id.to_string(),
        role: "user".to_string(),
        iss: "nexa-gateway-test".to_string(),
        username: username.to_string(),
        iat: now.timestamp(),
        exp: expiry.timestamp(),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("test-secret-key".as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreationError)?;
    
    Ok(token)
}

/// Simplified JWT token validation for tests
pub fn validate_jwt(token: &str) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("test-secret-key".as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _ => AuthError::InvalidToken,
    })?;
    
    Ok(token_data.claims)
}
