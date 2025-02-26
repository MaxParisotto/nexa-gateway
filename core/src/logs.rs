use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub source: String,
}

pub async fn get_recent_logs(limit: usize) -> anyhow::Result<Vec<LogEntry>> {
    use chrono::TimeZone;
    
    // This would normally fetch from a log storage system
    Ok(vec![
        LogEntry {
            timestamp: Utc.with_ymd_and_hms(2023, 8, 15, 14, 30, 0).unwrap(),
            level: "INFO".to_string(),
            message: "System started successfully".to_string(),
            source: "system".to_string(),
        },
        LogEntry {
            timestamp: Utc.with_ymd_and_hms(2023, 8, 15, 14, 31, 5).unwrap(),
            level: "INFO".to_string(),
            message: "Agent agent-1 registered".to_string(),
            source: "agent-registry".to_string(),
        },
        LogEntry {
            timestamp: Utc.with_ymd_and_hms(2023, 8, 15, 14, 32, 10).unwrap(),
            level: "WARNING".to_string(),
            message: "High CPU usage detected".to_string(),
            source: "monitoring".to_string(),
        },
    ][..std::cmp::min(3, limit)].to_vec())
}
