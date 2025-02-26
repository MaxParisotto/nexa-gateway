//! Vector database crate for Nexa Gateway
//!
//! This crate manages interactions with Qdrant for embeddings and semantic vector searches.

pub mod client;
pub mod models;
pub mod error;
pub mod operations;

pub use error::VectorDbError;

/// Result type for vector database operations
pub type VectorDbResult<T> = Result<T, VectorDbError>;

/// Vector database service
#[derive(Clone)]
pub struct VectorDbService {
    client: client::QdrantClient,
}

impl VectorDbService {
    /// Create a new vector database service
    pub async fn new(url: &str, api_key: Option<String>) -> VectorDbResult<Self> {
        let client = client::QdrantClient::new(url, api_key).await?;
        Ok(Self { client })
    }
    
    /// Insert a vector into the database
    pub async fn insert_vector(&self, collection: &str, id: &str, vector: Vec<f32>, payload: serde_json::Value) -> VectorDbResult<()> {
        operations::insert_vector(&self.client, collection, id, vector, payload).await
    }
    
    /// Search for similar vectors
    pub async fn search_similar(&self, collection: &str, vector: Vec<f32>, limit: u64) -> VectorDbResult<Vec<models::SearchResult>> {
        operations::search_similar(&self.client, collection, vector, limit).await
    }
    
    /// Delete a vector from the database
    pub async fn delete_vector(&self, collection: &str, id: &str) -> VectorDbResult<()> {
        operations::delete_vector(&self.client, collection, id).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
