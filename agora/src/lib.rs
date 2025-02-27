//! Agora crate for real-time WebSocket communication
//!
//! This crate provides WebSocket server functionality for real-time
//! communication, topic subscriptions, and message routing.

pub mod client;
pub mod topic;
pub mod server;
pub mod message;

use common::config::Settings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
// Removed unused import: tokio::net::TcpStream
use tokio_tungstenite::tungstenite::protocol::Message;
// Removed unused import: WebSocketStream
use tracing::{error, info};

// Type aliases for cleaner code
// Removed unused type alias: WsStream
type SubscriptionId = String;
type SubscriptionManager = Arc<Mutex<HashMap<SubscriptionId, tokio::sync::mpsc::Sender<String>>>>;

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
    
    #[error("Routing error: {0}")]
    RoutingError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Client error: {0}")]
    ClientError(String),
    
    #[error("Message error: {0}")]
    MessageError(String),
    
    #[error("Topic not found: {0}")]
    TopicNotFound(String),
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
        let _manager = self.subscription_manager.lock().map_err(|_| {
            AgoraError::SubscriptionError("Failed to acquire lock".to_string())
        })?;
        
        // Placeholder for subscription logic
        // manager.subscribe(topic, client_id);
        info!("Client {} subscribed to topic {}", client_id, topic);
        Ok(())
    }
    
    // Method to send a message to a topic
    pub fn send_message(&self, _message: String) -> Result<usize, AgoraError> {
        let _manager = self.subscription_manager.lock().map_err(|_| {
            AgoraError::RoutingError("Failed to acquire lock".to_string())
        })?;
        
        // Placeholder for broadcast logic
        // manager.broadcast_message(message)
        info!("Broadcasting message to subscribers");
        Ok(0) // Return 0 subscribers for now
    }

    pub async fn run(&self) -> Result<(), AgoraError> {
        let addr = format!("{}:{}", 
            self.settings.agora.host, 
            self.settings.agora.port
        );
        
        info!("Starting Agora WebSocket server on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        while let Ok((_stream, _)) = listener.accept().await {
            // Commented out until tokio-tungstenite is properly configured
            // let ws_stream = accept_async(stream).await?;
            info!("New WebSocket connection established");
            
            // TODO: Handle connection in separate task
        }
        
        Ok(())
    }

    // Commented out unused method
    // async fn get_websocket_url(&self) -> String {
    //     format!("ws://{}:{}", 
    //         self.settings.agora.host, 
    //         self.settings.agora.port
    //     )
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use server::WebSocketServer;
    use tokio::sync::mpsc;
    use tokio_tungstenite::connect_async;
    use futures::{SinkExt, StreamExt};
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_websocket_echo() {
        // Create a channel for the server
        let (_tx, rx) = mpsc::channel::<String>(100);
        
        // Use a random port to avoid conflicts
        let port = 9000 + rand::random::<u16>() % 1000;
        let server = WebSocketServer::new(port, rx);
        
        // Start the server in a background task
        tokio::spawn(async move {
            if let Err(e) = server.run().await {
                eprintln!("Server error: {:?}", e);
            }
        });
        
        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Connect a test client
        let url = format!("ws://localhost:{}/ws", port);
        let (mut ws_stream, _) = connect_async(url)
            .await
            .expect("Failed to connect to WebSocket server");
        
        // Create a test message
        let test_uuid = Uuid::new_v4().to_string();
        let test_message = serde_json::json!({
            "type": "test",
            "payload": {
                "message": format!("Test message {}", test_uuid)
            }
        }).to_string();
        
        // Send the message
        ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(test_message.clone().into()))
            .await
            .expect("Failed to send message");
        
        // Server should echo back the message
        let msg = ws_stream.next().await
            .expect("No response from server")
            .expect("Failed to receive message");
        
        if let tokio_tungstenite::tungstenite::Message::Text(received) = msg {
            assert_eq!(received, test_message);
        } else {
            panic!("Unexpected message type");
        }
        
        // Clean up
        ws_stream.close(None).await.expect("Failed to close WebSocket connection");
    }
}
