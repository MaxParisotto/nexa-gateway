use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub algorithm: String,
    pub access_token_expiry: u64,
    pub refresh_token_expiry: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub cors_origins: Vec<String>,
    pub rate_limit: u32,
    pub rate_limit_window: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt: JwtConfig,
    pub security: SecurityConfig,
    pub service_endpoint: String,
}

impl AuthConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config".into());

        let mut config = Config::builder()
            .add_source(File::with_name(&format!("{}/default", config_path)).required(false))
            .add_source(File::with_name(&format!("{}/{}", config_path, run_mode)).required(false))
            .add_source(Environment::with_prefix("AUTH").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    pub fn get_jwt_algorithm(&self) -> Result<jsonwebtoken::Algorithm, String> {
        match self.jwt.algorithm.as_str() {
            "HS256" => Ok(jsonwebtoken::Algorithm::HS256),
            "HS384" => Ok(jsonwebtoken::Algorithm::HS384),
            "HS512" => Ok(jsonwebtoken::Algorithm::HS512),
            _ => Err(format!("Unsupported JWT algorithm: {}", self.jwt.algorithm)),
        }
    }
}
