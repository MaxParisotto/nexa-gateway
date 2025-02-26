//! Status API for the dashboard
//! 
//! This module provides functions for fetching gateway status information.

use leptos::*;
use serde::{Deserialize, Serialize};

/// Gateway status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStatus {
    pub status: String,
    pub uptime: String,
    pub version: String,
    pub active_connections: i32,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

/// Get the current gateway status
#[server]
pub async fn get_status() -> Result<GatewayStatus, ServerFnError> {
    // In a real application, this would fetch data from the gateway API
    // For now, we'll return mock data
    Ok(GatewayStatus {
        status: "Online".to_string(),
        uptime: "2 days, 5 hours".to_string(),
        version: "0.1.0".to_string(),
        active_connections: 42,
        cpu_usage: 23.5,
        memory_usage: 512.0,
    })
}
