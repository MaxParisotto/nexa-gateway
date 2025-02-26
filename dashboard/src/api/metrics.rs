//! Metrics API for the dashboard
//! 
//! This module provides functions for fetching metrics data.

use leptos::*;
use crate::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

/// Time series data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: String,
    pub value: f64,
}

/// Metrics data for a specific metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsData {
    pub name: String,
    pub unit: String,
    pub data: Vec<DataPoint>,
}

/// Get metrics data for a specific metric
#[server]
pub async fn get_metrics(metric_name: String, _time_range: String) -> Result<MetricsData, ServerFnError> {
    // In a real application, this would fetch data from the gateway API
    // For now, we'll return mock data
    
    // Generate some sample data points
    let data_points = match metric_name.as_str() {
        "cpu" => vec![
            DataPoint { timestamp: "2025-02-20".to_string(), value: 23.5 },
            DataPoint { timestamp: "2025-02-21".to_string(), value: 25.2 },
            DataPoint { timestamp: "2025-02-22".to_string(), value: 28.7 },
            DataPoint { timestamp: "2025-02-23".to_string(), value: 22.1 },
            DataPoint { timestamp: "2025-02-24".to_string(), value: 24.5 },
            DataPoint { timestamp: "2025-02-25".to_string(), value: 26.8 },
            DataPoint { timestamp: "2025-02-26".to_string(), value: 29.2 },
        ],
        "memory" => vec![
            DataPoint { timestamp: "2025-02-20".to_string(), value: 512.0 },
            DataPoint { timestamp: "2025-02-21".to_string(), value: 524.3 },
            DataPoint { timestamp: "2025-02-22".to_string(), value: 498.7 },
            DataPoint { timestamp: "2025-02-23".to_string(), value: 532.1 },
            DataPoint { timestamp: "2025-02-24".to_string(), value: 545.6 },
            DataPoint { timestamp: "2025-02-25".to_string(), value: 528.9 },
            DataPoint { timestamp: "2025-02-26".to_string(), value: 510.2 },
        ],
        "requests" => vec![
            DataPoint { timestamp: "2025-02-20".to_string(), value: 156.0 },
            DataPoint { timestamp: "2025-02-21".to_string(), value: 142.0 },
            DataPoint { timestamp: "2025-02-22".to_string(), value: 164.0 },
            DataPoint { timestamp: "2025-02-23".to_string(), value: 178.0 },
            DataPoint { timestamp: "2025-02-24".to_string(), value: 153.0 },
            DataPoint { timestamp: "2025-02-25".to_string(), value: 162.0 },
            DataPoint { timestamp: "2025-02-26".to_string(), value: 170.0 },
        ],
        "latency" => vec![
            DataPoint { timestamp: "2025-02-20".to_string(), value: 42.0 },
            DataPoint { timestamp: "2025-02-21".to_string(), value: 38.0 },
            DataPoint { timestamp: "2025-02-22".to_string(), value: 45.0 },
            DataPoint { timestamp: "2025-02-23".to_string(), value: 40.0 },
            DataPoint { timestamp: "2025-02-24".to_string(), value: 37.0 },
            DataPoint { timestamp: "2025-02-25".to_string(), value: 43.0 },
            DataPoint { timestamp: "2025-02-26".to_string(), value: 41.0 },
        ],
        _ => vec![],
    };
    
    // Determine the unit based on the metric name
    let unit = match metric_name.as_str() {
        "cpu" => "%",
        "memory" => "MB",
        "requests" => "req/min",
        "latency" => "ms",
        _ => "",
    };
    
    Ok(MetricsData {
        name: metric_name,
        unit: unit.to_string(),
        data: data_points,
    })
}
