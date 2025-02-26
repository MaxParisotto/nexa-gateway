//! Logs API for the dashboard
//! 
//! This module provides functions for fetching and managing system logs.

use leptos::*;
use serde::{Deserialize, Serialize};

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

/// Log filter options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilter {
    pub level: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i32>,
}

/// Get system logs
#[server]
pub async fn get_logs(filter: Option<LogFilter>) -> Result<Vec<LogEntry>, ServerFnError> {
    // In a real application, this would fetch logs from the gateway API
    // For now, we'll return mock data
    let logs = vec![
        LogEntry {
            timestamp: "2025-02-26 11:30:12".to_string(),
            level: "INFO".to_string(),
            message: "Gateway started successfully".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 11:30:15".to_string(),
            level: "INFO".to_string(),
            message: "Connected to database".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 11:32:45".to_string(),
            level: "INFO".to_string(),
            message: "User admin logged in".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 11:35:22".to_string(),
            level: "WARN".to_string(),
            message: "High CPU usage detected (78%)".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 11:38:17".to_string(),
            level: "INFO".to_string(),
            message: "Cache cleared".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 11:40:03".to_string(),
            level: "ERROR".to_string(),
            message: "Failed to connect to external API: timeout".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 11:42:51".to_string(),
            level: "INFO".to_string(),
            message: "Scheduled maintenance started".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 11:43:12".to_string(),
            level: "INFO".to_string(),
            message: "Configuration updated".to_string(),
        },
    ];
    
    // Apply filters if provided
    if let Some(filter) = filter {
        let filtered_logs = logs.into_iter().filter(|log| {
            // Filter by log level if specified
            if let Some(level) = &filter.level {
                if level != "All" && log.level != *level {
                    return false;
                }
            }
            
            // Filter by search term if specified
            if let Some(search) = &filter.search {
                if !log.message.to_lowercase().contains(&search.to_lowercase()) {
                    return false;
                }
            }
            
            true
        }).collect::<Vec<_>>();
        
        // Apply limit if specified
        if let Some(limit) = filter.limit {
            return Ok(filtered_logs.into_iter().take(limit as usize).collect());
        }
        
        return Ok(filtered_logs);
    }
    
    Ok(logs)
}

/// Clear system logs
#[server]
pub async fn clear_logs() -> Result<bool, ServerFnError> {
    // In a real application, this would clear logs via the gateway API
    // For now, we'll just return success
    Ok(true)
}

/// Export logs to a file
#[server]
pub async fn export_logs(format: String) -> Result<String, ServerFnError> {
    // In a real application, this would generate a file and return a download URL
    // For now, we'll just return a mock URL
    Ok(format!("/api/logs/export.{}", format))
}
