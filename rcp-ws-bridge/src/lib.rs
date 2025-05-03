use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

use rcp_client::Client;

mod error;
pub use error::Error;

/// WebSocket message types that can be exchanged with browser clients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    /// Authentication request from browser
    Auth { token: String },
    /// RCP command from browser
    Command { service: String, data: serde_json::Value },
    /// Data update for browser
    Update { service: String, data: serde_json::Value },
    /// Error message for browser
    Error { code: String, message: String },
}

/// Configuration for the WebSocket bridge
pub struct BridgeConfig {
    /// The address to bind the WebSocket server to
    pub ws_addr: SocketAddr,
    /// The RCP server address to connect to
    pub rcp_addr: String,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            ws_addr: "127.0.0.1:9002".parse().unwrap(),
            rcp_addr: "127.0.0.1:9001".to_string(),
        }
    }
}

/// The WebSocket to RCP bridge
pub struct WsBridge {
    config: BridgeConfig,
}

impl WsBridge {
    /// Create a new WebSocket bridge with the given configuration
    pub fn new(config: BridgeConfig) -> Self {
        Self { config }
    }

    /// Start the WebSocket bridge server
    pub async fn start(&self) -> Result<()> {
        // Create a TCP listener
        let listener = TcpListener::bind(&self.config.ws_addr).await?;
        info!("WebSocket bridge listening on {}", self.config.ws_addr);

        // Accept connections
        while let Ok((stream, addr)) = listener.accept().await {
            info!("New WebSocket connection from {}", addr);
            let rcp_addr = self.config.rcp_addr.clone();
            
            // Spawn a new task for each connection
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, rcp_addr).await {
                    error!("Error handling WebSocket connection: {}", e);
                }
            });
        }

        Ok(())
    }
}

/// Handle a single WebSocket connection
async fn handle_connection(stream: TcpStream, rcp_addr: String) -> Result<()> {
    // Accept the WebSocket connection
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    // Create message channels
    let (to_ws_tx, mut to_ws_rx) = mpsc::channel::<Message>(32);
    
    // Client connection state
    let client = Arc::new(Mutex::new(None));
    let client_clone = client.clone();
    
    // Handle incoming messages from the WebSocket
    let ws_handler = async move {
        while let Some(msg) = ws_receiver.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            };
            
            // Handle different message types
            match msg {
                Message::Text(text) => {
                    debug!("Received text message: {}", text);
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(ws_msg) => {
                            match ws_msg {
                                WsMessage::Auth { token } => {
                                    // Create RCP client and authenticate
                                    let mut client_config = rcp_client::ClientConfig::default();
                                    client_config.server_addr = rcp_addr.clone();
                                    
                                    match Client::connect(client_config).await {
                                        Ok(mut new_client) => {
                                            match new_client.authenticate(&token).await {
                                                Ok(_) => {
                                                    info!("Client authenticated successfully");
                                                    let mut client_guard = client_clone.lock().await;
                                                    *client_guard = Some(new_client);
                                                    
                                                    // Send success response
                                                    let response = WsMessage::Update { 
                                                        service: "auth".to_string(),
                                                        data: serde_json::json!({"status": "success"})
                                                    };
                                                    if let Err(e) = to_ws_tx.send(Message::Text(serde_json::to_string(&response)?)).await {
                                                        error!("Failed to send auth response: {}", e);
                                                    }
                                                }
                                                Err(e) => {
                                                    error!("Authentication failed: {}", e);
                                                    let response = WsMessage::Error {
                                                        code: "AUTH_FAILED".to_string(),
                                                        message: format!("Authentication failed: {}", e)
                                                    };
                                                    if let Err(e) = to_ws_tx.send(Message::Text(serde_json::to_string(&response)?)).await {
                                                        error!("Failed to send auth error: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to connect to RCP server: {}", e);
                                            let response = WsMessage::Error {
                                                code: "CONNECTION_FAILED".to_string(),
                                                message: format!("Failed to connect to RCP server: {}", e)
                                            };
                                            if let Err(e) = to_ws_tx.send(Message::Text(serde_json::to_string(&response)?)).await {
                                                error!("Failed to send connection error: {}", e);
                                            }
                                        }
                                    }
                                }
                                WsMessage::Command { service, data } => {
                                    // Forward command to RCP client
                                    let mut client_guard = client_clone.lock().await;
                                    if let Some(client) = client_guard.as_mut() {
                                        // TODO: Implement command forwarding to appropriate service
                                        debug!("Forwarding command to service {}: {:?}", service, data);
                                        
                                        // This is a placeholder - actual implementation would depend on service types
                                        let response = WsMessage::Update {
                                            service: service.clone(),
                                            data: serde_json::json!({"status": "received"})
                                        };
                                        if let Err(e) = to_ws_tx.send(Message::Text(serde_json::to_string(&response)?)).await {
                                            error!("Failed to send command response: {}", e);
                                        }
                                    } else {
                                        // Client not authenticated
                                        let response = WsMessage::Error {
                                            code: "NOT_AUTHENTICATED".to_string(),
                                            message: "Not authenticated".to_string()
                                        };
                                        if let Err(e) = to_ws_tx.send(Message::Text(serde_json::to_string(&response)?)).await {
                                            error!("Failed to send auth error: {}", e);
                                        }
                                    }
                                }
                                _ => {
                                    warn!("Unsupported message type from browser");
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse WebSocket message: {}", e);
                            let response = WsMessage::Error {
                                code: "INVALID_MESSAGE".to_string(),
                                message: format!("Invalid message format: {}", e)
                            };
                            if let Err(e) = to_ws_tx.send(Message::Text(serde_json::to_string(&response)?)).await {
                                error!("Failed to send parse error: {}", e);
                            }
                        }
                    }
                }
                Message::Binary(_) => {
                    // Handle binary messages if needed
                    warn!("Binary WebSocket messages not supported");
                }
                Message::Ping(data) => {
                    // Automatically respond to ping with pong
                    if let Err(e) = to_ws_tx.send(Message::Pong(data)).await {
                        error!("Failed to send pong: {}", e);
                    }
                }
                Message::Close(_) => {
                    info!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        }
    };
    
    // Forward messages to WebSocket
    let ws_sender_task = async move {
        while let Some(msg) = to_ws_rx.recv().await {
            if let Err(e) = ws_sender.send(msg).await {
                error!("Failed to send WebSocket message: {}", e);
                break;
            }
        }
    };
    
    // Run both tasks concurrently
    tokio::select! {
        _ = ws_handler => {},
        _ = ws_sender_task => {},
    }
    
    Ok(())
}

/// Create and start a WebSocket bridge with default configuration
pub async fn start_bridge() -> Result<()> {
    let config = BridgeConfig::default();
    let bridge = WsBridge::new(config);
    bridge.start().await
}