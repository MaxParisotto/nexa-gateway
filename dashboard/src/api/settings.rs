//! Settings API for the dashboard
//! 
//! This module provides functions for fetching and updating gateway settings.

use leptos::*;
use crate::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

/// Gateway settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewaySettings {
    pub general: GeneralSettings,
    pub security: SecuritySettings,
    pub logging: LoggingSettings,
}

/// General settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub name: String,
    pub description: String,
    pub port: i32,
}

/// Security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub auth_enabled: bool,
    pub jwt_expiry: i32,
    pub rate_limit: i32,
}

/// Logging settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub log_level: String,
    pub log_retention: i32,
}

/// Get the current gateway settings
#[server]
pub async fn get_settings() -> Result<GatewaySettings, ServerFnError> {
    // In a real application, this would fetch data from the gateway API
    // For now, we'll return mock data
    Ok(GatewaySettings {
        general: GeneralSettings {
            name: "Nexa Gateway".to_string(),
            description: "Main API gateway for Nexa services".to_string(),
            port: 8080,
        },
        security: SecuritySettings {
            auth_enabled: true,
            jwt_expiry: 60,
            rate_limit: 100,
        },
        logging: LoggingSettings {
            log_level: "Info".to_string(),
            log_retention: 30,
        },
    })
}

/// Update gateway settings
#[server]
pub async fn update_settings(_settings: GatewaySettings) -> Result<bool, ServerFnError> {
    // In a real application, this would update the gateway settings via API
    // For now, we'll just return success
    Ok(true)
}

/// Reset gateway settings to defaults
#[server]
pub async fn reset_settings() -> Result<GatewaySettings, ServerFnError> {
    // In a real application, this would reset the gateway settings via API
    // For now, we'll return the default settings
    Ok(GatewaySettings {
        general: GeneralSettings {
            name: "Nexa Gateway".to_string(),
            description: "Main API gateway for Nexa services".to_string(),
            port: 8080,
        },
        security: SecuritySettings {
            auth_enabled: true,
            jwt_expiry: 60,
            rate_limit: 100,
        },
        logging: LoggingSettings {
            log_level: "Info".to_string(),
            log_retention: 30,
        },
    })
}
