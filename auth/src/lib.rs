//! Authentication crate for Nexa Gateway
//!
//! This crate handles authentication (JWT-based) and permissions.

pub mod jwt;
pub mod permissions;
pub mod middleware;
pub mod error;

pub use error::AuthError;

/// Result type for authentication operations
pub type AuthResult<T> = Result<T, AuthError>;

/// Authentication service
#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
    jwt_expiry: u64,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(jwt_secret: String, jwt_expiry: u64) -> Self {
        Self {
            jwt_secret,
            jwt_expiry,
        }
    }
    
    /// Validate a JWT token
    pub async fn validate_token(&self, token: &str) -> AuthResult<bool> {
        // Delegate to JWT module
        jwt::validate_token(token, &self.jwt_secret).await
    }
    
    /// Generate a JWT token
    pub async fn generate_token(&self, user_id: &str, role: &str) -> AuthResult<String> {
        // Delegate to JWT module
        jwt::generate_token(user_id, role, &self.jwt_secret, self.jwt_expiry).await
    }
    
    /// Check if a user has a specific permission
    pub async fn check_permission(&self, token: &str, permission: &str) -> AuthResult<bool> {
        // First validate token
        let claims = jwt::decode_token(token, &self.jwt_secret).await?;
        
        // Then check permission
        permissions::check_permission(&claims.role, permission)
    }
}
