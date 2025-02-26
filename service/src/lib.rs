//! Service layer for Nexa Gateway

use common::Result;
use async_trait::async_trait;
use tracing::info;

mod user;
mod auth;
mod error;

pub use user::UserService;
pub use auth::AuthService;
pub use error::ServiceError;

/// Base trait for all services in the system
#[async_trait]
pub trait Service: Send + Sync + 'static {
    /// Initialize the service with any required dependencies
    async fn initialize(&self) -> Result<(), ServiceError>;
}

// Export service implementations
pub mod services {
    pub use super::user::UserService;
    pub use super::auth::AuthService;
}

/// Initialize the service layer
pub async fn initialize() -> Result<()> {
    tracing::info!("Initializing service layer");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize() {
        assert!(initialize().await.is_ok());
    }
}
