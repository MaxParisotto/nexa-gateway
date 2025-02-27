//! Vector database crate for Nexa Gateway
//!
//! This crate manages interactions with Qdrant for embeddings and semantic vector searches.

pub mod client;
pub mod error;

pub use error::VectorDbError;

/// Result type for vector database operations
pub type VectorDbResult<T> = Result<T, VectorDbError>;

#[cfg(test)]
mod tests {
    use super::*;
    use client::QdrantClient;
    use serde_json::json;
    use std::collections::HashMap;
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_qdrant_operations() {
        // Skip test if Qdrant is not available
        match tokio::net::TcpStream::connect("localhost:6333").await {
            Ok(_) => {},
            Err(_) => {
                println!("Skipping test_qdrant_operations as Qdrant server is not available");
                return;
            }
        }
        
        // Create client
        let client = QdrantClient::new("http://localhost:6333").expect("Failed to create client");
        
        // Create test collection
        let collection_name = format!("test_collection_{}", Uuid::new_v4());
        client.create_collection(&collection_name, 4).await.expect("Failed to create collection");
        
        // Insert test vector
        let vector = vec![0.1, 0.2, 0.3, 0.4];
        let point_id = "test-point-1";
        
        let mut payload = HashMap::new();
        payload.insert("name".to_string(), json!("test point"));
        
        client.insert_point(&collection_name, point_id, &vector, Some(payload))
            .await
            .expect("Failed to insert point");
            
        // Search for similar vectors
        let results = client.search(&collection_name, &vector, 1)
            .await
            .expect("Failed to search");
            
        assert!(!results.is_empty());
        assert_eq!(results[0].id.as_deref(), Some(point_id));
        
        // Clean up
        client.delete_collection(&collection_name).await.expect("Failed to delete collection");
    }
}
