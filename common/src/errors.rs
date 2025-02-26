// Commented out until axum dependency is properly configured
// use axum::{
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     Json,
// };
// Removed unused import: serde_json::json
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Vector database error: {0}")]
    VectorDB(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

// Commented out until axum dependency is properly configured
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             Self::Auth(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
//             Self::Authorization(_) => (StatusCode::FORBIDDEN, self.to_string()),
//             Self::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
//             Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
//             Self::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".into()),
//             Self::VectorDB(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Vector database error occurred".into()),
//             Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
//             Self::ExternalService(_) => (StatusCode::BAD_GATEWAY, "External service error".into()),
//             Self::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string()),
//         };

//         let body = Json(json!({
//             "error": {
//                 "message": error_message,
//                 "code": status.as_u16(),
//             }
//         }));

//         (status, body).into_response()
//     }
// }

pub type Result<T> = std::result::Result<T, AppError>;
