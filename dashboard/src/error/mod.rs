//! Error handling for the dashboard
//! 
//! This module contains error types and handling for the dashboard.

use thiserror::Error;

/// Dashboard-specific errors
#[derive(Debug, Error)]
pub enum DashboardError {
    /// API request error
    #[error("API request failed: {0}")]
    ApiError(String),
    
    /// Authentication error
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),
}

/// Result type for dashboard operations
pub type Result<T> = std::result::Result<T, DashboardError>;
