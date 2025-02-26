//! Common data models shared across services.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Base model for all entities with common fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseModel {
    /// Unique identifier
    pub id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Pagination parameters for list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (0-based)
    pub page: usize,
    /// Number of items per page
    pub page_size: usize,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 10,
        }
    }
}

/// Paginated response containing items and pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Items in the current page
    pub items: Vec<T>,
    /// Total number of items
    pub total: usize,
    /// Current page number
    pub page: usize,
    /// Number of items per page
    pub page_size: usize,
    /// Total number of pages
    pub total_pages: usize,
}

/// User role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    /// Administrator with full access
    Admin,
    /// Regular user
    User,
    /// Read-only access
    ReadOnly,
}

/// Basic user information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Unique user identifier
    pub id: String,
    /// User's email address
    pub email: String,
    /// User's display name
    pub name: String,
    /// User's roles
    pub roles: Vec<Role>,
}

// Common data models used across crates

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub model: String,
    pub parameters: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub id: Uuid,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

// Add other common data models as needed
