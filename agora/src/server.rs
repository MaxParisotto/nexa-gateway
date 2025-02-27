use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use tracing::{debug, error, info};

/// WebSocket server for real-time communication
pub struct WebSocketServer {
    /// Port to listen on
    port: u16,
    /// Channel for receiving messages from other system components
    receiver: mpsc::Receiver<String>,
    /// Collection of active connections
    #[allow(dead_code)]
    connections: Vec<mpsc::Sender<String>>,
}

impl WebSocketServer {
    /// Create a new WebSocket server
    pub fn new(port: u16, receiver: mpsc::Receiver<String>) -> Self {
        Self {
            port,
            receiver,
            connections: Vec::new(),
        }
    }
    
    /// Run the WebSocket server
    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(&addr).await?;
        
        info!("WebSocket server listening on {}", addr);
        
        loop {
            tokio::select! {
                // Handle new connections
                Ok((stream, peer_addr)) = listener.accept() => {
                    info!("New WebSocket connection from {}", peer_addr);
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream).await {
                            error!("Error handling WebSocket connection: {}", e);
                        }
                    });
                }
                
                // Handle messages from other components
                Some(message) = self.receiver.recv() => {
                    debug!("Received message from channel: {}", message);
                    // In a real implementation, we would broadcast this message to relevant connections
                }
                
                else => break,
            }
        }
        
        Ok(())
    }
    
    /// Handle a WebSocket connection
    async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = accept_async(stream).await?;
        
        Self::process_websocket(ws_stream).await
    }
    
    /// Process WebSocket messages
    async fn process_websocket(mut ws_stream: WebSocketStream<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
        // Process incoming messages from the WebSocket client
        while let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(msg) => {
                    if msg.is_text() {
                        let text = msg.to_text()?;
                        debug!("Received WebSocket message: {}", text);
                        
                        // For testing, we'll simply echo back the received message
                        ws_stream.send(WsMessage::Text(text.to_string().into())).await?;
                    } else if msg.is_close() {
                        info!("WebSocket connection closed");
                        break;
                    }
                }
                Err(e) => {
                    error!("Error receiving WebSocket message: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
} 