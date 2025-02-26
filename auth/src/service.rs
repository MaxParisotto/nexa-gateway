use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use std::sync::Arc;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Missing required claim")]
    MissingClaim,
    #[error("Invalid issuer")]
    InvalidIssuer,
    #[error("Invalid audience")]
    InvalidAudience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: String,
}

pub struct AuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey<'static>,
    validation: Validation,
    issuer: String,
    audience: String,
}

impl AuthService {
    pub fn new(
        secret: &str,
        issuer: String,
        audience: String,
        algorithm: Algorithm,
    ) -> Arc<Self> {
        let validation = Validation::new(algorithm);
        validation.set_issuer(&[issuer.clone()]);
        validation.set_audience(&[audience.clone()]);

        Arc::new(Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()).into_static(),
            validation,
            issuer,
            audience,
        })
    }

    pub fn generate_token(&self, user_id: String, roles: Vec<String>, expires_in: u64) -> Result<String, AuthError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AuthError::InvalidToken)?
            .as_secs() as usize;

        let claims = Claims {
            sub: user_id,
            roles,
            exp: now + expires_in as usize,
            iat: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
        };

        encode(&Header::new(self.validation.alg), &claims, &self.encoding_key)
            .map_err(|_| AuthError::InvalidToken)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidSignature => AuthError::InvalidSignature,
                _ => AuthError::InvalidToken,
            })?;

        Ok(token_data.claims)
    }
}
