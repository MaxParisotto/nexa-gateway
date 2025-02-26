use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub capabilities: Vec<AgentCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub id: String,
    pub status: String,
    pub last_heartbeat: String,
    pub is_online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub name: String,
    pub description: String,
    pub parameters: Vec<String>,
}

pub async fn list_agents() -> anyhow::Result<Vec<AgentInfo>> {
    // This would normally fetch from a database
    Ok(vec![
        AgentInfo {
            id: "agent-1".to_string(),
            name: "Text Processor".to_string(),
            description: "Processes text inputs for various NLP tasks".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![
                AgentCapability {
                    name: "summarize".to_string(),
                    description: "Summarizes text".to_string(),
                    parameters: vec!["text".to_string(), "max_length".to_string()],
                },
                AgentCapability {
                    name: "translate".to_string(),
                    description: "Translates text".to_string(),
                    parameters: vec!["text".to_string(), "target_language".to_string()],
                },
            ],
        },
        AgentInfo {
            id: "agent-2".to_string(),
            name: "Image Analyzer".to_string(),
            description: "Analyzes image content".to_string(),
            version: "0.9.5".to_string(),
            capabilities: vec![
                AgentCapability {
                    name: "detect_objects".to_string(),
                    description: "Detects objects in images".to_string(),
                    parameters: vec!["image_url".to_string()],
                },
            ],
        },
    ])
}

pub async fn get_agent_status(agent_id: &str) -> anyhow::Result<AgentStatus> {
    // This would normally check the actual status
    Ok(AgentStatus {
        id: agent_id.to_string(),
        status: "Running".to_string(),
        last_heartbeat: "2023-08-15T14:30:00Z".to_string(),
        is_online: true,
    })
}
