//! Configuration management for Nexa Gateway.

use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

/// Authentication-related configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    /// JWT secret key for signing tokens
    pub jwt_secret: String,
    /// JWT token expiration time in hours
    pub jwt_expiration: u64,
}

/// Database configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
}

/// Vector database (Qdrant) configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VectorDBConfig {
    /// Qdrant server URL
    pub url: String,
    /// API key for authentication (if needed)
    pub api_key: Option<String>,
}

/// API server configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    /// Host address to bind the server to
    pub host: String,
    /// Port to listen on
    pub port: u16,
}

/// WebSocket server configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebSocketConfig {
    /// Host address to bind the WebSocket server to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Maximum message size in bytes
    pub max_message_size: usize,
}

/// Global application settings.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub agora: AgoraSettings,
    // Add other configuration sections as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgoraSettings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    /// Load configuration from file and environment variables.
    ///
    /// # Arguments
    /// * `config_path` - Path to the configuration file.
    ///
    /// # Returns
    /// Settings object or ConfigError
    pub fn new(config_path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let config_path = config_path.as_ref();
        info!("Loading configuration from {}", config_path.display());

        let settings = Config::builder()
            // Start with default configuration
            .add_source(File::with_name("config/default"))
            // Add environment-specific configuration
            .add_source(
                File::with_name(&format!(
                    "config/{}",
                    std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "production".into())
                ))
                .required(false),
            )
            // Add local overrides if the file exists
            .add_source(File::with_name("config/local").required(false))
            // Override with environment variables (e.g., APP_SERVER__PORT=8080)
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}

pub fn load_config() -> Result<Config> {
    // Use real production environment or config file
    let config = Config::builder()
        .add_source(config::Environment::default())
        .add_source(config::File::with_name("config/production"))
        .build()?
        .try_deserialize()?;
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("default.yaml");

        let config_content = r#"
            environment: production
            auth:
                jwt_secret: test_secret
                jwt_expiration: 24
            database:
                url: postgres://user:pass@localhost/testdb
                max_connections: 10
            vectordb:
                url: http://localhost:6333
                api_key: null
            server:
                host: 0.0.0.0
                port: 8080
            websocket:
                host: 0.0.0.0
                port: 8081
                max_message_size: 1048576
        "#;

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();

        std::env::set_var("CONFIG_PATH", config_path.to_str().unwrap());
        let settings = Settings::new(&dir.path().join("default")).unwrap();

        assert_eq!(settings.environment, "production");
        assert_eq!(settings.auth.jwt_secret, "test_secret");
        assert_eq!(settings.server.port, 8080);
    }
}
