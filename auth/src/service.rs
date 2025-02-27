use crate::error::AuthError;
use crate::jwt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User credentials for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// User information returned after successful authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: String,
}

/// AuthService for handling authentication and authorization
pub struct AuthService {
    /// JWT secret key
    #[allow(dead_code)]
    secret: String,
    /// Token expiry in hours
    #[allow(dead_code)]
    token_expiry: u64,
}

impl AuthService {
    /// Create a new AuthService
    pub async fn new() -> Result<Self, AuthError> {
        // In a real implementation, this would load from config
        // For testing, we use a static secret
        Ok(Self {
            secret: "test-secret-key".to_string(),
            token_expiry: 24,
        })
    }
    
    /// Authenticate a user with credentials
    pub async fn authenticate(&self, credentials: Credentials) -> Result<(UserInfo, String), AuthError> {
        // For testing, any credentials work with a random user ID
        let user_id = Uuid::new_v4().to_string();
        
        // Create a token for the user using our simple helper
        let token = jwt::create_jwt(&user_id, &credentials.username)?;
        
        // Return user info and token
        Ok((
            UserInfo {
                id: user_id,
                username: credentials.username,
                role: "user".to_string(),
            },
            token
        ))
    }
    
    /// Validate a token
    pub async fn validate(&self, token: &str) -> Result<bool, AuthError> {
        // Use our simple helper to validate
        let _ = jwt::validate_jwt(token)?;
        Ok(true)
    }
    
    /// Validate a token (alias for compatibility)
    pub async fn validate_token(&self, token: &str) -> Result<bool, AuthError> {
        self.validate(token).await
    }
    
    /// Check if a user has permission
    pub async fn check_permission(&self, token: &str, _permission: &str) -> Result<bool, AuthError> {
        // For testing, we'll always return true if the token is valid
        self.validate(token).await
    }
    
    /// Refresh an existing token
    pub async fn refresh_token(&self, token: &str) -> Result<String, AuthError> {
        // Validate the existing token
        let valid = self.validate(token).await?;
        
        if valid {
            // In a real implementation, this would decode the token and create a new one
            // We would extract the user ID, username, etc.
            // For testing, we'll just simulate this
            let user_id = Uuid::new_v4().to_string();
            let username = "test_user";
            
            jwt::create_jwt(&user_id, username)
        } else {
            Err(AuthError::InvalidToken)
        }
    }
}
