use thiserror::Error;
use qdrant_client::prelude::QdrantError;
use axum::http::StatusCode;

#[derive(Debug, Error)]
pub enum VectorDbError {
    #[error("Qdrant connection error: {0}")]
    Connection(#[from] QdrantError),
    
    #[error("Collection creation error: {0}")]
    CollectionCreation(String),
    
    #[error("Collection not found: {0}")]
    CollectionNotFound(String),
    
    #[error("Point insertion error: {0}")]
    PointInsertion(String),
    
    #[error("Point search error: {0}")]
    PointSearch(String),
    
    #[error("Point update error: {0}")]
    PointUpdate(String),
    
    #[error("Point deletion error: {0}")]
    PointDeletion(String),
    
    #[error("Embedding generation error: {0}")]
    EmbeddingGeneration(String),
    
    #[error("Invalid configuration: {0}")]
    Config(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl VectorDbError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            VectorDbError::Connection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::CollectionCreation(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::CollectionNotFound(_) => StatusCode::NOT_FOUND,
            VectorDbError::PointInsertion(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::PointSearch(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::PointUpdate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::PointDeletion(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::EmbeddingGeneration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::Config(_) => StatusCode::BAD_REQUEST,
            VectorDbError::Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VectorDbError::Deserialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl axum::response::IntoResponse for VectorDbError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = axum::Json(serde_json::json!({
            "error": self.to_string(),
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qdrant_client::prelude::QdrantError;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            VectorDbError::Connection(QdrantError::from("test".to_string())).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );

        assert_eq!(
            VectorDbError::CollectionNotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );

        assert_eq!(
            VectorDbError::Config("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
    }
}
