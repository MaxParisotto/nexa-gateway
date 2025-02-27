use crate::error::VectorDbError;
use qdrant_client::{
    Qdrant,
    qdrant::{
        CreateCollection, Distance, PointStruct, VectorParams, VectorsConfig,
        point_id::PointIdOptions,
    },
    config::QdrantConfig,
};
use serde_json;
use std::collections::HashMap;

/// Client for interacting with Qdrant vector database
pub struct QdrantClient {
    client: Qdrant,
}

impl QdrantClient {
    /// Create a new QdrantClient instance
    /// 
    /// # Arguments
    /// * `url` - The URL of the Qdrant server
    /// 
    /// # Returns
    /// * `Result<QdrantClient, VectorDbError>` - A new QdrantClient or an error
    pub fn new(url: &str) -> Result<Self, VectorDbError> {
        let config = QdrantConfig::from_url(url);
        let client = Qdrant::new(config)
            .map_err(|e| VectorDbError::ConnectionError(e.to_string()))?;
            
        Ok(Self { client })
    }
    
    /// Create a new collection in Qdrant
    /// 
    /// # Arguments
    /// * `collection_name` - The name of the collection to create
    /// * `vector_size` - The dimensionality of vectors to store
    /// 
    /// # Returns
    /// * `Result<(), VectorDbError>` - Success or an error
    pub async fn create_collection(&self, collection_name: &str, vector_size: u64) -> Result<(), VectorDbError> {
        let create_collection = CreateCollection {
            collection_name: collection_name.to_string(),
            vectors_config: Some(VectorsConfig {
                config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                    VectorParams {
                        size: vector_size,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    },
                )),
            }),
            ..Default::default()
        };
        
        self.client
            .create_collection(create_collection)
            .await
            .map_err(|e| VectorDbError::OperationError(e.to_string()))?;
            
        Ok(())
    }
    
    /// Insert a point into a collection
    /// 
    /// # Arguments
    /// * `collection_name` - The name of the collection
    /// * `id` - The ID for the point
    /// * `vector` - The vector data
    /// * `payload` - Optional payload data
    /// 
    /// # Returns
    /// * `Result<(), VectorDbError>` - Success or an error
    pub async fn insert_point(
        &self,
        collection_name: &str,
        id: &str,
        vector: &[f32],
        payload: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(), VectorDbError> {
        // Create a simple payload for testing
        let mut qdrant_payload = HashMap::new();
        if let Some(p) = payload {
            for (k, v) in p {
                if let serde_json::Value::String(s) = v {
                    qdrant_payload.insert(k, qdrant_client::qdrant::Value {
                        kind: Some(qdrant_client::qdrant::value::Kind::StringValue(s)),
                    });
                }
            }
        }
        
        let point = PointStruct {
            id: Some(id.to_string().into()),
            vectors: Some(vector.to_vec().into()),
            payload: qdrant_payload,
        };
        
        // Create a qdrant::UpsertPoints request
        let request = qdrant_client::qdrant::UpsertPoints {
            collection_name: collection_name.to_string(),
            points: vec![point],
            ..Default::default()
        };
        
        self.client
            .upsert_points(request)
            .await
            .map_err(|e| VectorDbError::OperationError(e.to_string()))?;
            
        Ok(())
    }
    
    /// Search for similar vectors in a collection
    /// 
    /// # Arguments
    /// * `collection_name` - The name of the collection
    /// * `query_vector` - The vector to search for
    /// * `limit` - The maximum number of results to return
    /// 
    /// # Returns
    /// * `Result<Vec<SearchResult>, VectorDbError>` - The search results or an error
    pub async fn search(
        &self,
        collection_name: &str,
        query_vector: &[f32],
        limit: u64,
    ) -> Result<Vec<SearchResult>, VectorDbError> {
        // Create a search request
        let request = qdrant_client::qdrant::SearchPoints {
            collection_name: collection_name.to_string(),
            vector: query_vector.to_vec(),
            limit,
            with_payload: Some(qdrant_client::qdrant::WithPayloadSelector {
                selector_options: Some(qdrant_client::qdrant::with_payload_selector::SelectorOptions::Enable(true)),
            }),
            with_vectors: Some(qdrant_client::qdrant::WithVectorsSelector {
                selector_options: Some(qdrant_client::qdrant::with_vectors_selector::SelectorOptions::Enable(true)),
            }),
            ..Default::default()
        };
        
        let results = self.client
            .search_points(request)
            .await
            .map_err(|e| VectorDbError::OperationError(e.to_string()))?;
        
        let mut search_results = Vec::new();
        
        for point in results.result {
            // Convert to our own format
            let mut json_payload = HashMap::new();
            
            for (k, v) in point.payload {
                if let Some(kind) = v.kind {
                    match kind {
                        qdrant_client::qdrant::value::Kind::StringValue(s) => {
                            json_payload.insert(k, serde_json::Value::String(s));
                        },
                        _ => { /* Skip other value types for now */ }
                    }
                }
            }
            
            let id = point.id.map(|id| {
                if let Some(point_id_options) = id.point_id_options {
                    match point_id_options {
                        PointIdOptions::Uuid(uuid) => uuid,
                        PointIdOptions::Num(num) => num.to_string(),
                    }
                } else {
                    "unknown".to_string()
                }
            });
            
            search_results.push(SearchResult {
                id,
                score: point.score,
                payload: json_payload,
            });
        }
            
        Ok(search_results)
    }
    
    /// Delete a collection
    /// 
    /// # Arguments
    /// * `collection_name` - The name of the collection to delete
    /// 
    /// # Returns
    /// * `Result<(), VectorDbError>` - Success or an error
    pub async fn delete_collection(&self, collection_name: &str) -> Result<(), VectorDbError> {
        self.client
            .delete_collection(collection_name)
            .await
            .map_err(|e| VectorDbError::OperationError(e.to_string()))?;
            
        Ok(())
    }
}

/// Search result from Qdrant
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Point ID
    pub id: Option<String>,
    /// Similarity score
    pub score: f32,
    /// Associated payload
    pub payload: HashMap<String, serde_json::Value>,
}
