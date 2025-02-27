use serde::{Deserialize, Serialize};
use anyhow::Result;
use sysinfo::{System, Networks};
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::debug;

// Global counters for network traffic estimation
static REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TOKEN_COUNTER: AtomicUsize = AtomicUsize::new(0);
static LAST_METRICS_TIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static LAST_REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);
static LAST_TOKEN_COUNT: AtomicUsize = AtomicUsize::new(0);

/// System metrics struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage_gb: f32,
    pub uptime: u64,
    pub active_connections: u32,
    pub requests_per_second: f32,
    pub tokens_per_second: f32,
}

/// Agent metrics struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub id: String,
    pub name: String,
    pub status: String,
    pub task_count: u32,
    pub uptime: u64,
}

/// Increment request counter
pub fn increment_request_counter() {
    REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);
}

/// Increment token counter
pub fn increment_token_counter(tokens: usize) {
    TOKEN_COUNTER.fetch_add(tokens, Ordering::SeqCst);
}

/// Get system metrics
pub fn get_system_metrics() -> Result<SystemMetrics> {
    // Create and refresh system info
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Calculate uptime in seconds
    let boot_time = System::boot_time();
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let uptime_seconds = current_time - boot_time;
    
    // Calculate CPU usage
    let cpu_usage = sys.global_cpu_usage() as f32;
    
    // Calculate memory usage
    let total_memory = sys.total_memory() as f64;
    let used_memory = total_memory - (sys.free_memory() as f64);
    let memory_usage_gb = (used_memory / (1024.0 * 1024.0 * 1024.0)) as f32;
    
    // Calculate network activity for active connections estimation
    let mut networks = Networks::new();
    networks.refresh(true);
    
    // Estimate active connections based on network activity and CPU usage
    // This is just a heuristic - more accurate methods would integrate with actual service stats
    let network_activity: usize = networks.iter()
        .map(|(_, network)| (network.received() as usize) + (network.transmitted() as usize))
        .sum();
    
    // Combine CPU activity and network activity to make a rough guess
    // at the number of active connections
    let active_connections = ((network_activity / 1024) as f32 * 0.1 + cpu_usage * 0.5) as u32;
    
    // Calculate requests and tokens per second
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let last_time = LAST_METRICS_TIME.swap(now, Ordering::SeqCst);
    let current_requests = REQUEST_COUNTER.load(Ordering::SeqCst);
    let current_tokens = TOKEN_COUNTER.load(Ordering::SeqCst);
    
    let last_requests = LAST_REQUEST_COUNT.swap(current_requests, Ordering::SeqCst);
    let last_tokens = LAST_TOKEN_COUNT.swap(current_tokens, Ordering::SeqCst);
    
    let time_diff = if last_time == 0 { 1 } else { now - last_time };
    let time_diff = if time_diff == 0 { 1 } else { time_diff };
    
    let requests_per_second = ((current_requests - last_requests) as f32) / (time_diff as f32);
    let tokens_per_second = ((current_tokens - last_tokens) as f32) / (time_diff as f32);
    
    // Create and return metrics
    let metrics = SystemMetrics {
        cpu_usage,
        memory_usage_gb,
        uptime: uptime_seconds,
        active_connections,
        requests_per_second,
        tokens_per_second,
    };
    
    debug!("System metrics: {:?}", metrics);
    Ok(metrics)
}

/// Get agent metrics (sample data)
pub fn get_agent_metrics() -> Result<Vec<AgentMetrics>> {
    // Sample data - in a real implementation, this would fetch from a DB or service discovery
    let agents = vec![
        AgentMetrics {
            id: "agent-1".to_string(),
            name: "TextProcessor".to_string(),
            status: "Running".to_string(),
            task_count: 12,
            uptime: 3600,
        },
        AgentMetrics {
            id: "agent-2".to_string(),
            name: "ImageGenerator".to_string(),
            status: "Running".to_string(),
            task_count: 5,
            uptime: 1800,
        },
        AgentMetrics {
            id: "agent-3".to_string(),
            name: "DataAnalyzer".to_string(),
            status: "Stopped".to_string(),
            task_count: 0,
            uptime: 0,
        },
    ];
    
    Ok(agents)
} 