//! Service layer error definitions

use thiserror::Error;

/// Errors that can occur in service operations
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Entity not found
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    /// Authentication error
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    /// Authorization error
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// External service error
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for ServiceError {
    fn from(err: anyhow::Error) -> Self {
        ServiceError::Unknown(err.to_string())
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        ServiceError::DatabaseError(err.to_string())
    }
}
