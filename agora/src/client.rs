use crate::{AgoraError, Message};
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub sender: mpsc::Sender<Message>,
    pub topics: Vec<String>,
}

impl Client {
    pub fn new(sender: mpsc::Sender<Message>) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            topics: Vec::new(),
        }
    }

    pub async fn send_message(&self, message: Message) -> Result<(), AgoraError> {
        self.sender
            .send(message)
            .await
            .map_err(|e| AgoraError::ClientError(format!("Failed to send message: {}", e)))
    }

    pub fn subscribe(&mut self, topic: &str) {
        if !self.topics.contains(&topic.to_string()) {
            self.topics.push(topic.to_string());
        }
    }

    pub fn unsubscribe(&mut self, topic: &str) {
        self.topics.retain(|t| t != topic);
    }

    pub fn is_subscribed(&self, topic: &str) -> bool {
        self.topics.contains(&topic.to_string())
    }
}
