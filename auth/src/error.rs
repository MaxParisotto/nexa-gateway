//! Error types for authentication
//!
//! This module defines authentication-related errors.

use thiserror::Error;

/// Authentication errors
#[derive(Error, Debug)]
pub enum AuthError {
    /// JWT token has expired
    #[error("Token has expired")]
    TokenExpired,
    
    /// Invalid JWT token
    #[error("Invalid token")]
    InvalidToken,
    
    /// Error creating JWT token
    #[error("Failed to create token")]
    TokenCreationError,
    
    /// Missing authentication
    #[error("Authentication required")]
    MissingAuth,
    
    /// Invalid credentials
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    /// Invalid role
    #[error("Invalid role")]
    InvalidRole,
    
    /// User lacks permission
    #[error("Permission denied")]
    PermissionDenied,
    
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}
