use crate::error::{Error, Result};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use futures_util::stream::StreamExt;

/// Authentication methods supported by the RCP protocol
#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    /// No authentication (not recommended for production)
    None,
    
    /// Pre-shared key authentication
    PreSharedKey,
    
    /// Certificate-based authentication
    Certificate,
    
    /// Username/password authentication
    UsernamePassword,
}

impl Default for AuthMethod {
    fn default() -> Self {
        AuthMethod::PreSharedKey
    }
}

/// Client connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ClientState {
    /// Client is disconnected
    Disconnected,
    
    /// Client is attempting to connect
    Connecting,
    
    /// Client is connected but not authenticated
    Connected,
    
    /// Client is authenticated and ready
    Authenticated,
    
    /// Client is disconnecting
    Disconnecting,
}

/// Client events that can be received by the application
#[derive(Debug, Clone)]
pub enum ClientEvent {
    /// Client state has changed
    StateChanged(ClientState),
    
    /// A frame has been received from the server
    FrameReceived(Frame),
    
    /// Client has been disconnected
    Disconnected(String),
    
    /// An error has occurred
    Error(String),
    
    /// Authentication succeeded
    AuthenticationSucceeded(Session),
    
    /// Authentication failed
    AuthenticationFailed(String),
}

/// Client configuration parameters
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Server hostname or IP address
    pub host: String,
    
    /// Server port
    pub port: u16,
    
    /// Authentication method to use
    pub auth_method: AuthMethod,
    
    /// Pre-shared key (when using PSK authentication)
    pub psk: Option<String>,
    
    /// Username (when using username/password authentication)
    pub username: Option<String>,
    
    /// Password (when using username/password authentication)
    pub password: Option<String>,
    
    /// Certificate path (when using certificate authentication)
    pub cert_path: Option<String>,
    
    /// Private key path (when using certificate authentication)
    pub key_path: Option<String>,
    
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    
    /// Reconnect automatically on connection loss
    pub auto_reconnect: bool,
    
    /// Maximum reconnection attempts (0 = unlimited)
    pub max_reconnect_attempts: u32,
    
    /// Client identifier
    pub client_id: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8716,
            auth_method: AuthMethod::default(),
            psk: None,
            username: None,
            password: None,
            cert_path: None,
            key_path: None,
            connection_timeout: 30,
            auto_reconnect: true,
            max_reconnect_attempts: 5,
            client_id: format!("rcp-client-{}", uuid::Uuid::new_v4()),
        }
    }
}

/// Session information after successful authentication
#[derive(Debug, Clone)]
pub struct Session {
    /// Unique session identifier
    pub session_id: String,
    
    /// Session creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Server capabilities
    pub server_capabilities: Vec<String>,
}

/// Frame structure for communication between client and server
#[derive(Debug, Clone)]
pub struct Frame {
    /// Command identifier for this frame
    command_id: u8,
    
    /// Frame payload data
    payload: Vec<u8>,
}

impl Frame {
    /// Create a new frame
    pub fn new(command_id: u8, payload: Vec<u8>) -> Self {
        Self { command_id, payload }
    }
    
    /// Get the command ID of this frame
    pub fn command_id(&self) -> u8 {
        self.command_id
    }
    
    /// Get the payload of this frame
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    
    /// Parse a frame from a byte buffer
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 5 {
            return Err(Error::frame_parsing("Frame too short"));
        }
        
        let command_id = data[0];
        let payload_len = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
        
        if data.len() < 5 + payload_len {
            return Err(Error::frame_parsing("Frame payload incomplete"));
        }
        
        let payload = data[5..5 + payload_len].to_vec();
        Ok(Self { command_id, payload })
    }
    
    /// Serialize the frame into a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let payload_len = self.payload.len() as u32;
        let mut buffer = Vec::with_capacity(5 + payload_len as usize);
        
        buffer.push(self.command_id);
        buffer.extend_from_slice(&payload_len.to_be_bytes());
        buffer.extend_from_slice(&self.payload);
        
        buffer
    }
}

/// Main RCP client implementation
pub struct Client {
    /// Client configuration
    config: ClientConfig,
    
    /// Current client state
    state: ClientState,
    
    /// Active TCP connection to the server
    connection: Option<TcpStream>,
    
    /// Current session after authentication
    session: Option<Session>,
    
    /// Channel for sending events to the application
    event_sender: Sender<ClientEvent>,
    
    /// Channel for receiving events from the client
    event_receiver: Receiver<ClientEvent>,
    
    /// Service subscriptions
    subscriptions: HashMap<String, bool>,
    
    /// Last connection error
    last_error: Option<String>,
}

impl Client {
    /// Create a new RCP client with the given configuration
    pub fn new(config: ClientConfig) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        
        Self {
            config,
            state: ClientState::Disconnected,
            connection: None,
            session: None,
            event_sender: sender,
            event_receiver: receiver,
            subscriptions: HashMap::new(),
            last_error: None,
        }
    }
    
    /// Connect to the server
    pub async fn connect(&mut self) -> Result<()> {
        if self.state != ClientState::Disconnected {
            return Err(Error::invalid_state("Client is already connected or connecting"));
        }
        
        self.set_state(ClientState::Connecting).await;
        
        // Create address string
        let addr = format!("{}:{}", self.config.host, self.config.port);
        
        // Resolve address
        let address = match addr.to_socket_addrs() {
            Ok(mut addrs) => match addrs.next() {
                Some(addr) => addr,
                None => return Err(Error::connection("Failed to resolve server address")),
            },
            Err(e) => return Err(Error::connection(format!("Failed to resolve address: {}", e))),
        };
        
        // Attempt to connect with timeout
        let timeout_duration = Duration::from_secs(self.config.connection_timeout);
        let stream = match timeout(timeout_duration, TcpStream::connect(address)).await {
            Ok(Ok(stream)) => stream,
            Ok(Err(e)) => {
                self.set_state(ClientState::Disconnected).await;
                return Err(Error::connection(format!("Failed to connect: {}", e)));
            },
            Err(_) => {
                self.set_state(ClientState::Disconnected).await;
                return Err(Error::timeout("Connection attempt timed out"));
            },
        };
        
        // Store connection
        self.connection = Some(stream);
        self.set_state(ClientState::Connected).await;
        
        // Start background tasks for handling incoming frames
        self.spawn_receiver_task();
        
        Ok(())
    }
    
    /// Authenticate with the server
    pub async fn authenticate(&mut self) -> Result<Session> {
        if self.state != ClientState::Connected {
            return Err(Error::invalid_state("Client must be connected before authentication"));
        }
        
        let auth_method = &self.config.auth_method;
        let client_id = &self.config.client_id;
        
        // Prepare authentication data based on the method
        let auth_data = match auth_method {
            AuthMethod::None => {
                vec![]
            },
            AuthMethod::PreSharedKey => {
                if let Some(psk) = &self.config.psk {
                    let mut data = Vec::new();
                    data.extend_from_slice(psk.as_bytes());
                    data
                } else {
                    return Err(Error::authentication("PSK authentication method selected but no PSK provided"));
                }
            },
            AuthMethod::UsernamePassword => {
                if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
                    let mut data = Vec::new();
                    data.extend_from_slice(username.as_bytes());
                    data.push(0); // null byte separator
                    data.extend_from_slice(password.as_bytes());
                    data
                } else {
                    return Err(Error::authentication("Username/password authentication method selected but credentials not provided"));
                }
            },
            AuthMethod::Certificate => {
                return Err(Error::authentication("Certificate authentication is not yet implemented"));
            },
        };
        
        // Create authentication frame
        let mut payload = Vec::new();
        payload.push(match auth_method {
            AuthMethod::None => 0,
            AuthMethod::PreSharedKey => 1,
            AuthMethod::UsernamePassword => 2,
            AuthMethod::Certificate => 3,
        });
        payload.extend_from_slice(client_id.as_bytes());
        payload.push(0); // null byte separator
        payload.extend_from_slice(&auth_data);
        
        let auth_frame = Frame::new(0x01, payload); // 0x01 = AUTH command
        
        // Send authentication frame
        let mut conn = match &mut self.connection {
            Some(conn) => conn,
            None => return Err(Error::connection("Connection lost")),
        };
        
        let data = auth_frame.serialize();
        conn.write_all(&data).await?;
        
        // Wait for authentication response
        let response = match timeout(Duration::from_secs(10), self.wait_for_auth_response()).await {
            Ok(response) => response,
            Err(_) => return Err(Error::timeout("Authentication timed out")),
        };
        
        match response {
            Ok(session) => {
                self.session = Some(session.clone());
                self.set_state(ClientState::Authenticated).await;
                Ok(session)
            },
            Err(e) => {
                self.last_error = Some(e.to_string());
                Err(e)
            }
        }
    }
    
    /// Disconnect from the server
    pub async fn disconnect(&mut self) -> Result<()> {
        if self.state == ClientState::Disconnected || self.state == ClientState::Disconnecting {
            return Ok(());
        }
        
        self.set_state(ClientState::Disconnecting).await;
        
        // Send disconnect frame if connected
        if let Some(mut conn) = self.connection.take() {
            let disconnect_frame = Frame::new(0x02, Vec::new()); // 0x02 = DISCONNECT command
            let _ = conn.write_all(&disconnect_frame.serialize()).await;
            let _ = conn.flush().await;
            let _ = conn.shutdown().await;
        }
        
        // Reset client state
        self.session = None;
        self.subscriptions.clear();
        self.set_state(ClientState::Disconnected).await;
        
        Ok(())
    }
    
    /// Send a frame to the server
    pub async fn send_frame(&mut self, frame: Frame) -> Result<()> {
        if self.state != ClientState::Authenticated && self.state != ClientState::Connected {
            return Err(Error::invalid_state("Client must be connected and authenticated to send frames"));
        }
        
        let mut conn = match &mut self.connection {
            Some(conn) => conn,
            None => return Err(Error::connection("Connection lost")),
        };
        
        let data = frame.serialize();
        conn.write_all(&data).await?;
        conn.flush().await?;
        
        Ok(())
    }
    
    /// Subscribe to a service
    pub async fn subscribe(&mut self, service_name: &str) -> Result<()> {
        if self.state != ClientState::Authenticated {
            return Err(Error::invalid_state("Client must be authenticated to subscribe to services"));
        }
        
        // Check if already subscribed
        if self.subscriptions.get(service_name).cloned().unwrap_or(false) {
            return Ok(());
        }
        
        // Create subscription frame
        let mut payload = Vec::new();
        payload.extend_from_slice(service_name.as_bytes());
        
        let frame = Frame::new(0x03, payload); // 0x03 = SUBSCRIBE command
        self.send_frame(frame).await?;
        
        // Update subscription status
        self.subscriptions.insert(service_name.to_string(), true);
        
        Ok(())
    }
    
    /// Unsubscribe from a service
    pub async fn unsubscribe(&mut self, service_name: &str) -> Result<()> {
        if self.state != ClientState::Authenticated {
            return Err(Error::invalid_state("Client must be authenticated to unsubscribe from services"));
        }
        
        // Check if not subscribed
        if !self.subscriptions.get(service_name).cloned().unwrap_or(false) {
            return Ok(());
        }
        
        // Create unsubscription frame
        let mut payload = Vec::new();
        payload.extend_from_slice(service_name.as_bytes());
        
        let frame = Frame::new(0x04, payload); // 0x04 = UNSUBSCRIBE command
        self.send_frame(frame).await?;
        
        // Update subscription status
        self.subscriptions.insert(service_name.to_string(), false);
        
        Ok(())
    }
    
    /// Get the current client state
    pub fn state(&self) -> ClientState {
        self.state.clone()
    }
    
    /// Get the current session, if authenticated
    pub fn session(&self) -> Option<Session> {
        self.session.clone()
    }
    
    /// Get the event receiver to listen for client events
    pub fn event_receiver(&mut self) -> mpsc::Receiver<ClientEvent> {
        self.event_receiver.clone()
    }
    
    /// Get the last error that occurred
    pub fn last_error(&self) -> Option<String> {
        self.last_error.clone()
    }
    
    /// Update the client state and send a state change event
    async fn set_state(&mut self, state: ClientState) {
        self.state = state.clone();
        let _ = self.event_sender.send(ClientEvent::StateChanged(state)).await;
    }
    
    /// Wait for authentication response
    async fn wait_for_auth_response(&mut self) -> Result<Session> {
        let mut receiver = self.event_receiver();
        
        while let Some(event) = receiver.recv().await {
            match event {
                ClientEvent::AuthenticationSucceeded(session) => {
                    return Ok(session);
                }
                ClientEvent::AuthenticationFailed(reason) => {
                    return Err(Error::authentication(reason));
                }
                ClientEvent::Error(e) => {
                    return Err(Error::other(e));
                }
                ClientEvent::Disconnected(reason) => {
                    return Err(Error::connection(reason));
                }
                _ => {
                    // Continue waiting for authentication response
                }
            }
        }
        
        Err(Error::connection("Connection closed while waiting for authentication response"))
    }
    
    /// Spawn a background task for handling incoming frames
    fn spawn_receiver_task(&mut self) {
        if let Some(conn) = self.connection.as_ref() {
            let mut reader = conn.try_clone().unwrap();
            let sender = self.event_sender.clone();
            
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                let mut data_buffer = Vec::new();
                
                loop {
                    match reader.read(&mut buffer).await {
                        Ok(0) => {
                            // Connection closed
                            let _ = sender.send(ClientEvent::Disconnected("Connection closed by server".to_string())).await;
                            break;
                        }
                        Ok(n) => {
                            // Append data to buffer
                            data_buffer.extend_from_slice(&buffer[..n]);
                            
                            // Process complete frames
                            while data_buffer.len() >= 5 {
                                // Check if we have a complete frame
                                let payload_len = u32::from_be_bytes([
                                    data_buffer[1], data_buffer[2], data_buffer[3], data_buffer[4]
                                ]) as usize;
                                
                                if data_buffer.len() < 5 + payload_len {
                                    // Incomplete frame, wait for more data
                                    break;
                                }
                                
                                // Extract and process frame
                                let frame_data = data_buffer[..5 + payload_len].to_vec();
                                data_buffer = data_buffer[5 + payload_len..].to_vec();
                                
                                match Frame::parse(&frame_data) {
                                    Ok(frame) => {
                                        // Handle special frames
                                        match frame.command_id() {
                                            0x11 => { // AUTH_OK
                                                if let Ok(session_id) = String::from_utf8(frame.payload().to_vec()) {
                                                    let session = Session {
                                                        session_id,
                                                        created_at: chrono::Utc::now(),
                                                        server_capabilities: Vec::new(),
                                                    };
                                                    let _ = sender.send(ClientEvent::AuthenticationSucceeded(session)).await;
                                                } else {
                                                    let _ = sender.send(ClientEvent::AuthenticationFailed(
                                                        "Invalid session ID in authentication response".to_string()
                                                    )).await;
                                                }
                                            }
                                            0x12 => { // AUTH_FAILED
                                                let reason = String::from_utf8_lossy(frame.payload()).to_string();
                                                let _ = sender.send(ClientEvent::AuthenticationFailed(reason)).await;
                                            }
                                            0x13 => { // ERROR
                                                let error = String::from_utf8_lossy(frame.payload()).to_string();
                                                let _ = sender.send(ClientEvent::Error(error)).await;
                                            }
                                            _ => {
                                                // Forward other frames to the application
                                                let _ = sender.send(ClientEvent::FrameReceived(frame)).await;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let _ = sender.send(ClientEvent::Error(format!("Frame parsing error: {}", e))).await;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = sender.send(ClientEvent::Error(format!("Read error: {}", e))).await;
                            let _ = sender.send(ClientEvent::Disconnected("Connection error".to_string())).await;
                            break;
                        }
                    }
                }
            });
        }
    }
}