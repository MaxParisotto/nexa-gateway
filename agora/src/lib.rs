use common::config::Settings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use tracing::{error, info};

// Type aliases for cleaner code
type WsStream = WebSocketStream<TcpStream>;
type SubscriptionId = String;
type SubscriptionManager = Arc<Mutex<HashMap<SubscriptionId, tokio::sync::mpsc::Sender<Message>>>>;

#[derive(Debug, Error)]
pub enum AgoraError {
    #[error("Connection error: {0}")]
    StringError(String),
    
    #[error("WebSocket error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] common::error::CommonError),
    
    #[error("Subscription error: {0}")]
    SubscriptionError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgoraRequest {
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgoraResponse {
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}

pub struct AgoraServer {
    subscription_manager: SubscriptionManager,
    settings: Settings,
}

impl AgoraServer {
    pub fn new(settings: Settings) -> Self {
        AgoraServer {
            subscription_manager: Arc::new(Mutex::new(HashMap::new())),
            settings,
        }
    }
    
    // Methods for handling connections and messages
    pub async fn handle_connection(&self, client_id: String) -> Result<(), AgoraError> {
        info!("New WebSocket connection from client {}", client_id);
        // Connection handling would be implemented here
        Ok(())
    }
    
    // Method to subscribe a client to a topic
    pub fn subscribe_client(&self, topic: &str, client_id: &str) -> Result<(), AgoraError> {
        let mut manager = self.subscription_manager.lock().map_err(|_| {
            AgoraError::SubscriptionError("Failed to acquire lock".to_string())
        })?;
        
        manager.subscribe(topic, client_id);
        Ok(())
    }
    
    // Method to send a message to a topic
    pub fn send_message(&self, message: Message) -> Result<usize, AgoraError> {
        let manager = self.subscription_manager.lock().map_err(|_| {
            AgoraError::RoutingError("Failed to acquire lock".to_string())
        })?;
        
        manager.broadcast_message(message)
    }

    pub async fn run(&self) -> Result<(), AgoraError> {
        let addr = format!("{}:{}", 
            self.settings.agora.host, 
            self.settings.agora.port
        );
        
        info!("Starting Agora WebSocket server on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = accept_async(stream).await?;
            info!("New WebSocket connection established");
            
            // TODO: Handle connection in separate task
        }
        
        Ok(())
    }

    async fn get_websocket_url(&self) -> String {
        format!("ws://{}:{}", 
            self.settings.agora.host, 
            self.settings.agora.port
        )
    }
}
