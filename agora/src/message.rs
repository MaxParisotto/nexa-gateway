use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Message {
    /// Subscription message to a topic
    #[serde(rename = "subscribe")]
    Subscribe {
        /// Topic to subscribe to
        topic: String,
    },
    
    /// Unsubscribe message from a topic
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        /// Topic to unsubscribe from
        topic: String,
    },
    
    /// General message to a topic
    #[serde(rename = "message")]
    TopicMessage {
        /// Target topic
        topic: String,
        /// Message content
        message: String,
        /// Optional metadata for the message
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        metadata: HashMap<String, String>,
    },
    
    /// System notification
    #[serde(rename = "system")]
    SystemNotification {
        /// Notification level (info, warning, error)
        level: String,
        /// Notification message
        message: String,
    },
    
    /// Heartbeat to maintain connection
    #[serde(rename = "heartbeat")]
    Heartbeat {
        /// Client ID
        client_id: String,
        /// Timestamp
        timestamp: i64,
    },
    
    /// Acknowledgment of a received message
    #[serde(rename = "ack")]
    Acknowledgment {
        /// Message ID being acknowledged
        message_id: String,
        /// Status (success, error)
        status: String,
        /// Optional error message
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
    
    /// Generic message type for testing purposes
    #[serde(rename = "test")]
    Test {
        /// Test message
        message: String,
    },
}

impl Message {
    /// Create a new test message
    pub fn new_test(message: &str) -> Self {
        Message::Test {
            message: message.to_string(),
        }
    }
    
    /// Create a new topic message
    pub fn new_topic_message(topic: &str, message: &str) -> Self {
        Message::TopicMessage {
            topic: topic.to_string(),
            message: message.to_string(),
            metadata: HashMap::new(),
        }
    }
    
    /// Create a new subscription message
    pub fn new_subscribe(topic: &str) -> Self {
        Message::Subscribe {
            topic: topic.to_string(),
        }
    }
    
    /// Generate a unique ID for this message
    pub fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
} 