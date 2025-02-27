use anyhow::{Result, Context};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn, error};

/// Model information from OpenAI / LM Studio API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    #[serde(default)]
    pub object: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub owned_by: String,
    #[serde(default)]
    pub permission: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response from OpenAI / LM Studio models API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

/// Fetch available models from LM Studio
pub async fn fetch_available_models(url: &str) -> Result<Vec<String>> {
    info!("Fetching available models from LM Studio at {}", url);
    
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .context("Failed to build HTTP client")?;
    
    // Normalize URL to ensure it ends with v1
    let base_url = if url.ends_with('/') {
        format!("{}v1", url)
    } else {
        format!("{}/v1", url)
    };
    
    let models_url = format!("{}/models", base_url);
    
    let response = client
        .get(&models_url)
        .send()
        .await
        .context("Failed to send request to LM Studio")?;
    
    match response.status() {
        StatusCode::OK => {
            // Try to parse as ModelsResponse (OpenAI format)
            match response.json::<ModelsResponse>().await {
                Ok(models_response) => {
                    let model_names: Vec<String> = models_response.data
                        .into_iter()
                        .map(|model| model.id)
                        .collect();
                    
                    info!("Found {} models from LM Studio", model_names.len());
                    if !model_names.is_empty() {
                        info!("Available models: {}", model_names.join(", "));
                    } else {
                        warn!("No models returned from LM Studio");
                    }
                    
                    Ok(model_names)
                },
                Err(e) => {
                    // Try to parse as a raw JSON value for debugging
                    warn!("Failed to parse models response as OpenAI format: {}", e);
                    
                    // Get raw response text for debugging
                    let response_text = match client.get(&models_url).send().await {
                        Ok(res) => match res.text().await {
                            Ok(text) => format!("Response body: {}", text),
                            Err(_) => "Could not read response text".to_string()
                        },
                        Err(_) => "Could not get response".to_string()
                    };
                    
                    info!("Raw API response: {}", response_text);
                    
                    // Return default model
                    Ok(vec!["local".to_string()])
                }
            }
        },
        StatusCode::NOT_FOUND => {
            warn!("Models endpoint not found at {}. LM Studio may not have loaded any models yet.", models_url);
            // If endpoint doesn't exist, return a default model
            Ok(vec!["local".to_string()])
        },
        status => {
            error!("Failed to fetch models: HTTP {}", status);
            // Try another approach - sometimes LM Studio uses a model route
            match client.get(&format!("{}/model", base_url)).send().await {
                Ok(alt_response) if alt_response.status().is_success() => {
                    info!("Found model endpoint instead of models endpoint");
                    Ok(vec!["local".to_string()])
                },
                _ => {
                    // Return a default model on error
                    Ok(vec!["local".to_string()])
                }
            }
        }
    }
}

/// Test the connection to LM Studio
pub async fn test_connection(url: &str) -> Result<bool> {
    info!("Testing connection to LM Studio at {}", url);
    
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .context("Failed to build HTTP client")?;
    
    // Normalize URL
    let base_url = if url.ends_with('/') {
        format!("{}v1", url)
    } else {
        format!("{}/v1", url)
    };
    
    let models_url = format!("{}/models", base_url);
    
    match client.get(&models_url).send().await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() || status == StatusCode::NOT_FOUND {
                // NOT_FOUND is acceptable as some LM Studio deployments might not have the /models endpoint
                info!("Successfully connected to LM Studio");
                Ok(true)
            } else {
                warn!("Connection to LM Studio failed with status: {}", status);
                Ok(false)
            }
        },
        Err(e) => {
            error!("Connection to LM Studio failed: {}", e);
            Ok(false)
        }
    }
}

/// Get a default model if available or fall back to "local"
pub fn get_default_model(models: &[String]) -> String {
    if models.is_empty() {
        return "local".to_string();
    }
    
    // Try to find a good default model in preference order
    for preferred in &["gpt-3.5-turbo", "gpt-4", "llama", "mistral", "claude"] {
        for model in models {
            if model.to_lowercase().contains(preferred) {
                return model.clone();
            }
        }
    }
    
    // Otherwise, use the first available model
    models[0].clone()
} 