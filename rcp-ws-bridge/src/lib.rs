mod error;

use error::{Error, Result};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use rcp_client::{Client, ClientBuilder};
use rcp_core::{AuthMethod, Frame};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use uuid::Uuid;

/// Messages exchanged with the WebSocket client
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum WsMessage {
    /// Authentication request from browser
    Auth { token: String },

    /// RCP command from browser
    Command {
        service: String,
        data: serde_json::Value,
    },

    /// Data update for browser
    Update {
        service: String,
        data: serde_json::Value,
    },

    /// Error message for browser
    Error { code: String, message: String },
}

/// Configuration for the WebSocket bridge
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    /// Address to bind WebSocket server to
    pub ws_host: String,

    /// Port to use for WebSocket server
    pub ws_port: u16,

    /// RCP server hostname
    pub rcp_host: String,

    /// RCP server port
    pub rcp_port: u16,
}

/// WebSocket bridge to proxy RCP connections over WebSockets
pub struct WsBridge {
    config: BridgeConfig,
}

impl WsBridge {
    /// Create a new WebSocket bridge
    pub fn new(config: BridgeConfig) -> Self {
        Self { config }
    }

    /// Start the WebSocket server
    pub async fn start(&self) -> Result<()> {
        // Create WebSocket address
        let addr = format!("{}:{}", self.config.ws_host, self.config.ws_port);
        let socket_addr: SocketAddr = addr.parse().expect("Invalid WebSocket address");

        // Create listener
        let listener = TcpListener::bind(&socket_addr).await?;
        info!("WebSocket server listening on: {}", socket_addr);

        // Accept connections
        while let Ok((stream, addr)) = listener.accept().await {
            info!("New WebSocket connection from {}", addr);
            let rcp_host = self.config.rcp_host.clone();
            let rcp_port = self.config.rcp_port;

            // Spawn a new task for each connection
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, rcp_host, rcp_port).await {
                    error!("Error in connection handler: {}", e);
                }
            });
        }

        Ok(())
    }
}

/// Handle a single WebSocket connection
async fn handle_connection(stream: TcpStream, rcp_host: String, rcp_port: u16) -> Result<()> {
    // Accept the WebSocket connection
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Create message channels
    let (to_ws_tx, mut to_ws_rx) = mpsc::channel::<Message>(32);

    // Client connection state
    let client = Arc::new(Mutex::new(None));
    let client_clone = client.clone();

    // Handle incoming messages from the WebSocket
    let ws_handler = async {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(msg) => {
                    if msg.is_close() {
                        debug!("WebSocket connection closed by client");
                        break;
                    }

                    // Handle different message types
                    match msg {
                        Message::Text(text) => {
                            debug!("Received text message: {}", text);
                            match serde_json::from_str::<WsMessage>(&text) {
                                Ok(ws_msg) => {
                                    match ws_msg {
                                        WsMessage::Auth { token } => {
                                            debug!("Received authentication request");
                                            // Create a new RCP client
                                            let client_builder = ClientBuilder::new()
                                                .host(&rcp_host)
                                                .port(rcp_port)
                                                .client_name("WebSocketBridge")
                                                .client_id(Uuid::new_v4())
                                                .auth_method(AuthMethod::PreSharedKey)
                                                .auth_psk(token.clone());

                                            let new_client = client_builder.build();

                                            // Attempt to connect
                                            match new_client.connect().await {
                                                Ok(_) => {
                                                    info!("Connected to RCP server, authenticating...");
                                                    // Authenticate
                                                    match new_client.authenticate().await {
                                                        Ok(_) => {
                                                            info!(
                                                                "Client authenticated successfully"
                                                            );

                                                            // Start client message processor
                                                            if let Err(e) = new_client.start().await
                                                            {
                                                                error!(
                                                                    "Failed to start client: {}",
                                                                    e
                                                                );
                                                                let response = WsMessage::Error {
                                                                    code: "START_FAILED".to_string(),
                                                                    message: format!(
                                                                        "Failed to start client: {}",
                                                                        e
                                                                    ),
                                                                };
                                                                let json_str =
                                                                    serde_json::to_string(
                                                                        &response,
                                                                    )
                                                                    .unwrap_or_default();
                                                                if let Err(e) = to_ws_tx
                                                                    .send(Message::Text(json_str))
                                                                    .await
                                                                {
                                                                    error!(
                                                                        "Failed to send error response: {}",
                                                                        e
                                                                    );
                                                                }
                                                                continue;
                                                            }

                                                            // Store the client
                                                            let mut client_guard =
                                                                client_clone.lock().await;
                                                            *client_guard = Some(new_client);

                                                            // Send success response
                                                            let response = WsMessage::Update {
                                                                service: "auth".to_string(),
                                                                data: serde_json::json!({"status": "success"}),
                                                            };
                                                            let json_str =
                                                                serde_json::to_string(&response)
                                                                    .unwrap_or_default();
                                                            if let Err(e) = to_ws_tx
                                                                .send(Message::Text(json_str))
                                                                .await
                                                            {
                                                                error!(
                                                                    "Failed to send auth response: {}",
                                                                    e
                                                                );
                                                            }
                                                        }
                                                        Err(e) => {
                                                            error!("Authentication failed: {}", e);
                                                            let response = WsMessage::Error {
                                                                code: "AUTH_FAILED".to_string(),
                                                                message: format!(
                                                                    "Authentication failed: {}",
                                                                    e
                                                                ),
                                                            };
                                                            let json_str =
                                                                serde_json::to_string(&response)
                                                                    .unwrap_or_default();
                                                            if let Err(e) = to_ws_tx
                                                                .send(Message::Text(json_str))
                                                                .await
                                                            {
                                                                error!(
                                                                    "Failed to send auth error: {}",
                                                                    e
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    error!(
                                                        "Failed to connect to RCP server: {}",
                                                        e
                                                    );
                                                    let response = WsMessage::Error {
                                                        code: "CONNECTION_FAILED".to_string(),
                                                        message: format!(
                                                            "Failed to connect to RCP server: {}",
                                                            e
                                                        ),
                                                    };
                                                    let json_str = serde_json::to_string(&response)
                                                        .unwrap_or_default();
                                                    if let Err(e) =
                                                        to_ws_tx.send(Message::Text(json_str)).await
                                                    {
                                                        error!(
                                                            "Failed to send connection error: {}",
                                                            e
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        WsMessage::Command { service, data } => {
                                            debug!("Received command for service: {}", service);
                                            let mut client_guard = client_clone.lock().await;
                                            if let Some(client) = client_guard.as_mut() {
                                                // TODO: Implement command forwarding to appropriate service
                                                debug!(
                                                    "Forwarding command to service {}: {:?}",
                                                    service, data
                                                );

                                                // This is a placeholder - actual implementation would depend on service types
                                                let response = WsMessage::Update {
                                                    service: service.clone(),
                                                    data: serde_json::json!({"status": "received"}),
                                                };
                                                let json_str = serde_json::to_string(&response)
                                                    .unwrap_or_default();
                                                if let Err(e) =
                                                    to_ws_tx.send(Message::Text(json_str)).await
                                                {
                                                    error!(
                                                        "Failed to send command response: {}",
                                                        e
                                                    );
                                                }
                                            } else {
                                                // Client not authenticated
                                                let response = WsMessage::Error {
                                                    code: "NOT_AUTHENTICATED".to_string(),
                                                    message: "Not authenticated".to_string(),
                                                };
                                                let json_str = serde_json::to_string(&response)
                                                    .unwrap_or_default();
                                                if let Err(e) =
                                                    to_ws_tx.send(Message::Text(json_str)).await
                                                {
                                                    error!("Failed to send auth error: {}", e);
                                                }
                                            }
                                        }
                                        _ => {
                                            debug!("Received unsupported message type");
                                            let response = WsMessage::Error {
                                                code: "INVALID_MESSAGE_TYPE".to_string(),
                                                message: "Unsupported message type".to_string(),
                                            };
                                            let json_str = serde_json::to_string(&response)
                                                .unwrap_or_default();
                                            to_ws_tx.send(Message::Text(json_str)).await.ok();
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse WebSocket message: {}", e);
                                    let response = WsMessage::Error {
                                        code: "INVALID_MESSAGE".to_string(),
                                        message: format!("Invalid message format: {}", e),
                                    };
                                    let json_str =
                                        serde_json::to_string(&response).unwrap_or_default();
                                    if let Err(e) = to_ws_tx.send(Message::Text(json_str)).await {
                                        error!("Failed to send error response: {}", e);
                                    }
                                }
                            }
                        }
                        Message::Binary(data) => {
                            debug!("Received binary data, length: {}", data.len());
                            // Process binary data (if needed for binary protocols)
                        }
                        Message::Ping(data) => {
                            debug!("Received ping");
                            // Send pong response automatically
                            if let Err(e) = to_ws_tx.send(Message::Pong(data)).await {
                                error!("Failed to send pong: {}", e);
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    };

    // Forward messages to WebSocket
    let ws_sender_task = async move {
        while let Some(msg) = to_ws_rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                error!("WebSocket sender error");
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

/// Start a WebSocket bridge with the given configuration
pub async fn start_bridge(config: BridgeConfig) -> Result<()> {
    let bridge = WsBridge::new(config);
    bridge.start().await
}
