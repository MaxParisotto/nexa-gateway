//! Error handling for the dashboard
//! 
//! This module contains error types and handling for the dashboard.

use thiserror::Error;

/// Dashboard-specific errors
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DashboardError {
    /// API error
    #[error("API error: {0}")]
    ApiError(String),
    
    /// Not found error
    #[error("Not found: {0}")]
    NotFoundError(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Remove the conflicting Display implementation since it's already derived via thiserror::Error
// impl fmt::Display for DashboardError {...}

/// Result type for dashboard operations
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, DashboardError>;
