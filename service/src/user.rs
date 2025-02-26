//! User management service implementation

use crate::{Service, ServiceError};
use anyhow::Result;
use async_trait::async_trait;
use nexa_gateway_core::models::User;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{info, error};

/// Service for user-related operations
pub struct UserService {
    db: Arc<PgPool>,
}

impl UserService {
    /// Create a new user service
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    /// Retrieve a user by ID
    pub async fn get_user(&self, id: &str) -> Result<User, ServiceError> {
        info!("Fetching user with ID: {}", id);
        
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&*self.db)
        .await
        .map_err(|e| {
            error!("Database error fetching user {}: {:?}", id, e);
            ServiceError::DatabaseError(e.to_string())
        })?;

        user.ok_or_else(|| ServiceError::NotFound(format!("User {} not found", id)))
    }

    // Additional user-related methods would go here
}

#[async_trait]
impl Service for UserService {
    async fn initialize(&self) -> Result<(), ServiceError> {
        info!("Initializing user service");
        Ok(())
    }
}
