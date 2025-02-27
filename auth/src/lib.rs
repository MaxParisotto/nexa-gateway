//! Authentication crate for Nexa Gateway
//!
//! This crate handles authentication (JWT-based) and permissions.

pub mod jwt;
pub mod permissions;
pub mod middleware;
pub mod error;
pub mod service;

pub use error::AuthError;
pub use service::AuthService;

/// Result type for authentication operations
pub type AuthResult<T> = Result<T, AuthError>;

#[cfg(test)]
mod tests {
    use super::*;
    use jwt::{create_jwt, validate_jwt};
    use uuid::Uuid;
    
    #[test]
    fn test_jwt() {
        let user_id = Uuid::new_v4().to_string();
        let username = "test_user";
        
        // Create JWT
        let token = create_jwt(&user_id, username).expect("Failed to create JWT");
        
        // Validate JWT
        let claims = validate_jwt(&token).expect("Failed to validate JWT");
        
        // Check claims
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.username, username);
        assert_eq!(claims.role, "user");
    }
}
