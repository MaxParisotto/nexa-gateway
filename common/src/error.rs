//! Common error types and error handling utilities.

// Commented out until axum dependency is properly configured
// use axum::{
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     Json,
// };
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Common API error types.
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ApiError {
    /// Authentication errors
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Authorization errors
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Not found errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Bad request errors
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Server errors
    #[error("Internal server error: {0}")]
    InternalServer(String),

    /// Database errors
    #[error("Database error: {0}")]
    Database(String),

    /// Vector database errors
    #[error("Vector database error: {0}")]
    VectorDB(String),
}

/// Error response structure for API errors.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
}

// Commented out until axum dependency is properly configured
// impl IntoResponse for ApiError {
//     fn into_response(self) -> Response {
//         let (status_code, error_code) = match &self {
//             ApiError::Authentication(_) => (StatusCode::UNAUTHORIZED, "AUTHENTICATION_ERROR"),
//             ApiError::Authorization(_) => (StatusCode::FORBIDDEN, "AUTHORIZATION_ERROR"),
//             ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
//             ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
//             ApiError::InternalServer(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
//             ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
//             ApiError::VectorDB(_) => (StatusCode::INTERNAL_SERVER_ERROR, "VECTORDB_ERROR"),
//         };

//         let body = ErrorResponse {
//             code: error_code.to_string(),
//             message: self.to_string(),
//         };

//         (status_code, Json(body)).into_response()
//     }
// }

#[derive(Debug, thiserror::Error)]
pub enum CommonError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Environment error: {0}")]
    EnvError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}
