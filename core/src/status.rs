use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub uptime: u64,
    pub active_connections: u32,
    pub requests_per_second: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub agent_id: String,
    pub status: String,
    pub tasks_completed: u32,
    pub tasks_pending: u32,
    pub last_activity: String,
    pub resource_usage: f32,
}

pub async fn get_system_metrics() -> anyhow::Result<SystemMetrics> {
    // This would normally fetch real metrics
    Ok(SystemMetrics {
        cpu_usage: 25.5,
        memory_usage: 1024.0,
        uptime: 3600,
        active_connections: 42,
        requests_per_second: 123.45,
    })
}

pub async fn get_agent_metrics() -> anyhow::Result<Vec<AgentMetrics>> {
    // This would normally fetch real metrics
    Ok(vec![
        AgentMetrics {
            agent_id: "agent-1".to_string(),
            status: "Running".to_string(),
            tasks_completed: 100,
            tasks_pending: 5,
            last_activity: "2023-08-15T14:30:00Z".to_string(),
            resource_usage: 15.2,
        },
        AgentMetrics {
            agent_id: "agent-2".to_string(),
            status: "Idle".to_string(),
            tasks_completed: 75,
            tasks_pending: 0,
            last_activity: "2023-08-15T14:25:00Z".to_string(),
            resource_usage: 5.1,
        },
    ])
}
