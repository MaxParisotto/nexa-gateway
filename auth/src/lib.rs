//! Authentication module for the Nexa gateway.
//! Provides JWT token generation, password hashing, and authorization functionalities.

use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use common::config::Settings;

/// Authentication-related errors.
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AuthError {
    /// Returned when user credentials are invalid.
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    /// Returned when the provided token is invalid.
    #[error("Invalid token")]
    InvalidToken,
    
    /// Returned when the user does not have permission.
    #[error("Permission denied")]
    PermissionDenied,
    
    /// Returned when the token has expired.
    #[error("Token expired")]
    TokenExpired,
    
    /// Returned for general internal errors.
    #[error("Internal error")]
    InternalError,
    
    /// Returned when an Argon2 operation fails.
    #[error("Argon2 error: {0}")]
    Argon2Error(String),
}

/// JWT claims structure containing user information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// User ID
    pub sub: String,
    /// Expiration timestamp
    pub exp: usize,
    /// User roles
    pub roles: Vec<String>,
}

/// Authentication service for handling JWT tokens and password operations.
pub struct AuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    algorithm: Algorithm,
    token_expiration: Duration,
}

impl AuthService {
    /// Creates a new authentication service.
    ///
    /// # Arguments
    /// * `config` - The application settings containing JWT configuration.
    pub fn new(config: &Settings) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.auth.jwt_secret.as_ref()),
            decoding_key: DecodingKey::from_secret(config.auth.jwt_secret.as_ref()),
            algorithm: Algorithm::HS256,
            token_expiration: Duration::hours(config.auth.jwt_expiration as i64),
        }
    }

    /// Generates a JWT token for a user.
    ///
    /// # Arguments
    /// * `user_id` - The unique identifier for the user.
    /// * `roles` - The roles associated with the user.
    ///
    /// # Returns
    /// A Result with the JWT token string or an AuthError.
    pub fn generate_token(&self, user_id: &str, roles: Vec<String>) -> Result<String, AuthError> {
        let expiration = Utc::now()
            .checked_add_signed(self.token_expiration)
            .ok_or(AuthError::InternalError)?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            roles,
        };

        encode(&Header::new(self.algorithm), &claims, &self.encoding_key)
            .map_err(|_| AuthError::InternalError)
    }

    /// Validates a JWT token.
    ///
    /// # Arguments
    /// * `token` - The JWT token to validate.ord => Ok(false),
    ///
    /// # Returns
    /// A Result with the Claims if valid or an AuthError.
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(self.algorithm),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        Ok(token_data.claims)
    }

    /// Hashes a password using Argon2.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash.
    ///
    /// # Returns
    /// A Result with the hashed password or an AuthError.
    pub fn hash_password(password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AuthError::Argon2Error(e.to_string()))
    }

    /// Verifies a password against a hash.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify.
    /// * `hash` - The hashed password to verify against.
    ///
    /// # Returns
    /// A Result with a boolean indicating if the password is correct or an AuthError.
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
        let password_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::Argon2Error(e.to_string()))?;
        
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok())
    }
}

/// Trait for checking user permissions.
#[async_trait]
pub trait PermissionChecker: Send + Sync {
    /// Checks if a user has a specific permission.
    ///
    /// # Arguments
    /// * `user_id` - The user's unique identifier.
    /// * `permission` - The permission to check for.
    ///
    /// # Returns
    /// A Result with a boolean indicating if the user has the permission or an AuthError.
    async fn check_permission(&self, user_id: &str, permission: &str) -> Result<bool, AuthError>;
}

/// A simple implementation of the PermissionChecker trait.
pub struct SimplePermissionChecker;

#[async_trait]
impl PermissionChecker for SimplePermissionChecker {
    /// Checks if a user has a specific permission.
    /// 
    /// This is a placeholder implementation that always returns true.
    async fn check_permission(&self, _user_id: &str, _permission: &str) -> Result<bool, AuthError> {
        // This is a placeholder. In a real application, you'd check against a database
        // or other permission store
        Ok(true)
    }
}
