use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::broadcast;

use crate::{AgoraError, Message};
use tracing::{debug, warn};

const CHANNEL_CAPACITY: usize = 1000;

#[derive(Debug, Clone)]
pub struct Topic {
    name: String,
    sender: broadcast::Sender<Message>,
}

impl Topic {
    pub fn new(name: &str) -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self {
            name: name.to_string(),
            sender,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Message> {
        self.sender.subscribe()
    }

    pub fn publish(&self, message: Message) -> Result<usize, AgoraError> {
        self.sender
            .send(message)
            .map_err(|e| AgoraError::MessageError(format!("Failed to publish message: {}", e)))
    }
}

#[derive(Debug)]
pub struct TopicManager {
    topics: RwLock<HashMap<String, Topic>>,
}

impl TopicManager {
    pub fn new() -> Self {
        Self {
            topics: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_topic(&self, name: &str) -> Result<(), AgoraError> {
        let mut topics = self.topics
            .write()
            .map_err(|_| AgoraError::TopicNotFound("Failed to acquire write lock".into()))?;
            
        if !topics.contains_key(name) {
            topics.insert(name.to_string(), Topic::new(name));
            debug!("Created topic: {}", name);
        }
        Ok(())
    }

    pub fn get_topic(&self, name: &str) -> Result<Topic, AgoraError> {
        let topics = self.topics
            .read()
            .map_err(|_| AgoraError::TopicNotFound("Failed to acquire read lock".into()))?;
            
        topics
            .get(name)
            .cloned()
            .ok_or_else(|| {
                let msg = format!("Topic not found: {}", name);
                warn!("{}", msg);
                AgoraError::TopicNotFound(msg)
            })
    }

    pub fn get_or_create_topic(&self, name: &str) -> Result<Topic, AgoraError> {
        // First try to get the topic
        match self.get_topic(name) {
            Ok(topic) => Ok(topic),
            Err(_) => {
                // If not found, create and get again
                self.create_topic(name)?;
                self.get_topic(name)
            }
        }
    }

    pub fn delete_topic(&self, name: &str) -> Result<(), AgoraError> {
        let mut topics = self.topics
            .write()
            .map_err(|_| AgoraError::TopicNotFound("Failed to acquire write lock".into()))?;
            
        if topics.remove(name).is_some() {
            debug!("Removed topic: {}", name);
        }
        
        Ok(())
    }

    pub fn list_topics(&self) -> Result<Vec<String>, AgoraError> {
        let topics = self.topics
            .read()
            .map_err(|_| AgoraError::TopicNotFound("Failed to acquire read lock".into()))?;
            
        Ok(topics.keys().cloned().collect())
    }
}
