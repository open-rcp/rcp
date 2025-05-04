use crate::{
    error::{Error, Result},
    service::{ServiceClient, ServiceFactory, ServiceMessage, ServiceType},
    DEFAULT_CONNECTION_TIMEOUT_SECS, DEFAULT_KEEP_ALIVE_SECS, DEFAULT_RECONNECT_DELAY_MS,
};
use log::{debug, error, info, trace, warn};
use rcp_core::{
    Auth, AuthChallenge, AuthMethod, AuthPayload, AuthResponse, CommandId, ConnectionState, Frame,
    Protocol, SessionInfo, DEFAULT_PORT,
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex, RwLock},
    time,
};
use uuid::Uuid;

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Server hostname or IP address
    pub host: String,

    /// Server port
    pub port: u16,

    /// Client name/description
    pub client_name: String,

    /// Client ID (auto-generated if None)
    pub client_id: Option<Uuid>,

    /// Authentication method to use
    pub auth_method: AuthMethod,

    /// Pre-shared key for authentication
    pub auth_psk: Option<String>,

    /// Reconnect automatically on disconnection
    pub auto_reconnect: bool,

    /// Delay before reconnection attempt (ms)
    pub reconnect_delay_ms: u64,

    /// Keep-alive interval in seconds
    pub keep_alive_secs: u64,

    /// Connection timeout in seconds
    pub connection_timeout_secs: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: DEFAULT_PORT,
            client_name: "RCP Client".to_string(),
            client_id: Some(Uuid::new_v4()),
            auth_method: AuthMethod::PreSharedKey,
            auth_psk: None,
            auto_reconnect: true,
            reconnect_delay_ms: DEFAULT_RECONNECT_DELAY_MS,
            keep_alive_secs: DEFAULT_KEEP_ALIVE_SECS,
            connection_timeout_secs: DEFAULT_CONNECTION_TIMEOUT_SECS,
        }
    }
}

/// Builder for creating an RCP client
#[derive(Debug, Default)]
pub struct ClientBuilder {
    /// Client configuration
    config: ClientConfig,
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
        }
    }

    /// Set the server host
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.config.host = host.into();
        self
    }

    /// Set the server port
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set the client name
    pub fn client_name(mut self, name: impl Into<String>) -> Self {
        self.config.client_name = name.into();
        self
    }

    /// Set the client ID
    pub fn client_id(mut self, id: Uuid) -> Self {
        self.config.client_id = Some(id);
        self
    }

    /// Set the authentication method
    pub fn auth_method(mut self, method: AuthMethod) -> Self {
        self.config.auth_method = method;
        self
    }

    /// Set the pre-shared key for authentication
    pub fn auth_psk(mut self, psk: impl Into<String>) -> Self {
        self.config.auth_psk = Some(psk.into());
        self
    }

    /// Enable or disable automatic reconnection
    pub fn auto_reconnect(mut self, enable: bool) -> Self {
        self.config.auto_reconnect = enable;
        self
    }

    /// Set the reconnection delay
    pub fn reconnect_delay(mut self, delay_ms: u64) -> Self {
        self.config.reconnect_delay_ms = delay_ms;
        self
    }

    /// Set the keep-alive interval
    pub fn keep_alive_interval(mut self, seconds: u64) -> Self {
        self.config.keep_alive_secs = seconds;
        self
    }

    /// Set the connection timeout
    pub fn connection_timeout(mut self, seconds: u64) -> Self {
        self.config.connection_timeout_secs = seconds;
        self
    }

    /// Build the client
    pub fn build(self) -> Client {
        Client::new(self.config)
    }
}

/// Client state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientState {
    /// Disconnected
    Disconnected,

    /// Connecting
    Connecting,

    /// Connected but not authenticated
    Connected,

    /// Authenticating
    Authenticating,

    /// Authenticated and ready
    Ready,

    /// Closing
    Closing,
}

impl From<ConnectionState> for ClientState {
    fn from(state: ConnectionState) -> Self {
        match state {
            ConnectionState::Connected => Self::Connected,
            ConnectionState::Authenticating => Self::Authenticating,
            ConnectionState::Authenticated => Self::Ready,
            ConnectionState::Closing => Self::Closing,
            ConnectionState::Closed => Self::Disconnected,
        }
    }
}

/// Main RCP client
#[derive(Debug)]
pub struct Client {
    /// Client configuration
    config: ClientConfig,

    /// Client state
    state: Arc<RwLock<ClientState>>,

    /// Session info
    session_info: Arc<RwLock<Option<SessionInfo>>>,

    /// Protocol handler
    protocol: Arc<Mutex<Option<Protocol<TcpStream>>>>,

    /// Services
    services: Arc<RwLock<HashMap<ServiceType, ServiceClient>>>,
}

impl Client {
    /// Create a new client
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(ClientState::Disconnected)),
            session_info: Arc::new(RwLock::new(None)),
            protocol: Arc::new(Mutex::new(None)),
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Get the current client state
    pub async fn state(&self) -> ClientState {
        *self.state.read().await
    }

    /// Connect to the server
    pub async fn connect(&self) -> Result<()> {
        // Check if already connected
        {
            let state = *self.state.read().await;
            if state != ClientState::Disconnected {
                return Err(Error::Connection(
                    "Already connected or connecting".to_string(),
                ));
            }

            // Update state
            *self.state.write().await = ClientState::Connecting;
        }

        // Connect to server with timeout
        let server_addr = format!("{}:{}", self.config.host, self.config.port);
        debug!("Connecting to {}", server_addr);

        let stream = match time::timeout(
            Duration::from_secs(self.config.connection_timeout_secs),
            TcpStream::connect(&server_addr),
        )
        .await
        {
            Ok(Ok(stream)) => stream,
            Ok(Err(e)) => {
                *self.state.write().await = ClientState::Disconnected;
                return Err(Error::Connection(format!("Failed to connect: {}", e)));
            }
            Err(_) => {
                *self.state.write().await = ClientState::Disconnected;
                return Err(Error::Timeout(format!(
                    "Connection timeout after {} seconds",
                    self.config.connection_timeout_secs
                )));
            }
        };

        debug!("Connected to {}", server_addr);

        // Create protocol handler
        let protocol = Protocol::new(stream);
        *self.protocol.lock().await = Some(protocol);

        // Update state
        *self.state.write().await = ClientState::Connected;

        Ok(())
    }

    /// Authenticate with the server
    pub async fn authenticate(&self) -> Result<()> {
        // Check state
        {
            let state = *self.state.read().await;
            if state != ClientState::Connected {
                return Err(Error::Authentication(format!(
                    "Cannot authenticate in state {:?}",
                    state
                )));
            }

            // Update state
            *self.state.write().await = ClientState::Authenticating;
        }

        let mut protocol = self.protocol.lock().await;
        let protocol = match protocol.as_mut() {
            Some(p) => p,
            None => {
                *self.state.write().await = ClientState::Disconnected;
                return Err(Error::Connection("Not connected".to_string()));
            }
        };

        protocol.set_state(ConnectionState::Authenticating);

        // Create authentication payload
        let auth_payload = AuthPayload {
            client_id: self.config.client_id.unwrap_or_else(Uuid::new_v4),
            client_name: self.config.client_name.clone(),
            auth_method: self.config.auth_method,
            auth_data: Vec::new(),
        };

        // Serialize and send
        let auth_data = rcp_core::utils::to_bytes(&auth_payload)?;
        let auth_frame = Frame::new(CommandId::Auth as u8, auth_data);
        protocol.write_frame(&auth_frame).await?;

        // Wait for challenge
        let challenge_frame = match protocol.read_frame().await? {
            Some(frame) if frame.command_id() == CommandId::Auth as u8 => frame,
            Some(_) => {
                *self.state.write().await = ClientState::Connected;
                return Err(Error::Authentication("Expected AUTH challenge".to_string()));
            }
            None => {
                *self.state.write().await = ClientState::Disconnected;
                return Err(Error::Connection(
                    "Connection closed during authentication".to_string(),
                ));
            }
        };

        // Parse challenge
        let challenge: AuthChallenge = rcp_core::utils::from_bytes(challenge_frame.payload())?;

        // Handle challenge based on auth method
        match self.config.auth_method {
            AuthMethod::PreSharedKey => {
                let psk = match &self.config.auth_psk {
                    Some(key) => key,
                    None => {
                        *self.state.write().await = ClientState::Connected;
                        return Err(Error::Authentication("PSK not configured".to_string()));
                    }
                };

                // Generate response
                let response_data =
                    Auth::compute_psk_response(psk, &challenge.challenge, &challenge.salt);
                let auth_response = AuthResponse {
                    client_id: self.config.client_id.unwrap_or_else(Uuid::new_v4),
                    response: response_data,
                };

                // Send response
                let response_data = rcp_core::utils::to_bytes(&auth_response)?;
                let response_frame = Frame::new(CommandId::Auth as u8, response_data);
                protocol.write_frame(&response_frame).await?;
            }
            _ => {
                *self.state.write().await = ClientState::Connected;
                return Err(Error::Authentication(format!(
                    "Authentication method {:?} not implemented",
                    self.config.auth_method
                )));
            }
        }

        // Wait for result (session info)
        let session_frame = match protocol.read_frame().await? {
            Some(frame) if frame.command_id() == CommandId::Auth as u8 => frame,
            Some(_) => {
                *self.state.write().await = ClientState::Connected;
                return Err(Error::Authentication("Expected session info".to_string()));
            }
            None => {
                *self.state.write().await = ClientState::Disconnected;
                return Err(Error::Connection(
                    "Connection closed during authentication".to_string(),
                ));
            }
        };

        // Parse session info
        let session_info: SessionInfo = rcp_core::utils::from_bytes(session_frame.payload())?;

        // Store session info
        *self.session_info.write().await = Some(session_info);

        // Update state
        protocol.set_state(ConnectionState::Authenticated);
        *self.state.write().await = ClientState::Ready;

        info!("Authentication successful");
        Ok(())
    }

    /// Connect and authenticate in one step
    pub async fn connect_and_authenticate(&self) -> Result<()> {
        self.connect().await?;
        self.authenticate().await?;
        Ok(())
    }

    /// Start the client message processing loop
    pub async fn start(&self) -> Result<()> {
        // Check state
        {
            let state = *self.state.read().await;
            if state != ClientState::Ready {
                return Err(Error::Session(format!("Cannot start in state {:?}", state)));
            }
        }

        // Set up background tasks for message handling
        let state = Arc::clone(&self.state);
        let protocol_lock = Arc::clone(&self.protocol);
        let services = Arc::clone(&self.services);

        // Message processor task
        tokio::spawn(async move {
            debug!("Starting client message processor");

            loop {
                // Check state
                if *state.read().await != ClientState::Ready {
                    break;
                }

                // Process incoming messages
                let frame_result = {
                    let mut protocol_guard = protocol_lock.lock().await;
                    if let Some(protocol) = protocol_guard.as_mut() {
                        protocol.read_frame().await
                    } else {
                        break;
                    }
                };

                match frame_result {
                    Ok(Some(frame)) => {
                        // Process frame
                        if let Err(e) = process_frame(frame, &services).await {
                            error!("Error processing frame: {}", e);
                        }
                    }
                    Ok(None) => {
                        // Connection closed
                        warn!("Connection closed by server");
                        *state.write().await = ClientState::Disconnected;
                        break;
                    }
                    Err(e) => {
                        // Connection error
                        error!("Connection error: {}", e);
                        *state.write().await = ClientState::Disconnected;
                        break;
                    }
                }
            }

            debug!("Client message processor stopped");
        });

        Ok(())
    }

    /// Subscribe to a service
    pub async fn subscribe_service(&self, service_type: ServiceType) -> Result<ServiceClient> {
        // Check if already subscribed
        {
            let services = self.services.read().await;
            if services.contains_key(&service_type) {
                return Ok(services[&service_type].clone());
            }
        }

        // Check state
        {
            let state = *self.state.read().await;
            if state != ClientState::Ready {
                return Err(Error::Session(format!(
                    "Cannot subscribe to service in state {:?}",
                    state
                )));
            }
        }

        debug!("Subscribing to service: {:?}", service_type);

        // Create service instance
        let service = ServiceFactory::create(service_type)
            .ok_or_else(|| Error::Service(format!("Service {:?} not implemented", service_type)))?;

        // Send subscription request
        let service_name = service_type.as_str().as_bytes().to_vec();
        let frame = Frame::new(service_type.subscription_command(), service_name);

        // Send the frame
        {
            let mut protocol_guard = self.protocol.lock().await;
            if let Some(protocol) = protocol_guard.as_mut() {
                protocol.write_frame(&frame).await?;
            } else {
                return Err(Error::Connection("Not connected".to_string()));
            }
        }

        // Create service channels
        let (tx, mut rx) = mpsc::channel::<ServiceMessage>(100);

        // Create service client
        let service_client =
            ServiceClient::new(service_type, service_type.as_str().to_string(), tx.clone());

        // Store service client
        {
            let mut services = self.services.write().await;
            services.insert(service_type, service_client.clone());
        }

        // Start service handling in background
        let protocol_lock = Arc::clone(&self.protocol);
        let state = Arc::clone(&self.state);
        let mut service = service;

        tokio::spawn(async move {
            debug!("Starting service handler for {:?}", service_type);

            // Start the service
            if let Err(e) = service.start().await {
                error!("Failed to start service {:?}: {}", service_type, e);
                return;
            }

            // Process service messages
            while let Some(msg) = rx.recv().await {
                // Check if client is still connected
                if *state.read().await != ClientState::Ready {
                    break;
                }

                trace!("Received service message: {:?}", msg.id);

                // Process message
                if let Err(e) = service.handle_message(msg.clone()).await {
                    error!("Error handling service message: {}", e);
                    continue;
                }

                // Send message to server if needed
                if let Some(protocol) = protocol_lock.lock().await.as_mut() {
                    if let Err(e) = protocol.write_frame(&msg.frame).await {
                        error!("Failed to send service frame to server: {}", e);
                    }
                }
            }

            debug!("Service handler for {:?} stopped", service_type);

            // Stop the service
            if let Err(e) = service.stop().await {
                error!("Error stopping service {:?}: {}", service_type, e);
            }
        });

        Ok(service_client)
    }

    /// Get a service client if already subscribed
    pub async fn get_service(&self, service_type: ServiceType) -> Option<ServiceClient> {
        let services = self.services.read().await;
        services.get(&service_type).cloned()
    }

    /// Get or create a service client
    pub async fn get_or_subscribe_service(
        &self,
        service_type: ServiceType,
    ) -> Result<ServiceClient> {
        // Check if already subscribed
        if let Some(service) = self.get_service(service_type).await {
            return Ok(service);
        }

        // Subscribe to the service
        self.subscribe_service(service_type).await
    }

    /// Get the session info
    pub async fn session_info(&self) -> Option<SessionInfo> {
        self.session_info.read().await.clone()
    }

    /// Disconnect from the server
    pub async fn disconnect(&self) -> Result<()> {
        // Check state
        {
            let state = *self.state.read().await;
            if state == ClientState::Disconnected {
                return Ok(());
            }

            // Update state
            *self.state.write().await = ClientState::Closing;
        }

        // Close services
        {
            let services = self.services.read().await;
            debug!("Shutting down {} services", services.len());
        }

        // Close connection
        {
            let mut protocol_guard = self.protocol.lock().await;
            if let Some(protocol) = protocol_guard.as_mut() {
                if let Err(e) = protocol.close().await {
                    warn!("Error closing connection: {}", e);
                }
            }
            *protocol_guard = None;
        }

        // Clear session info
        *self.session_info.write().await = None;

        // Update state
        *self.state.write().await = ClientState::Disconnected;

        debug!("Disconnected from server");
        Ok(())
    }

    /// Check if the client is connected
    pub async fn is_connected(&self) -> bool {
        matches!(
            *self.state.read().await,
            ClientState::Connected | ClientState::Authenticating | ClientState::Ready
        )
    }

    /// Check if the client is authenticated
    pub async fn is_authenticated(&self) -> bool {
        *self.state.read().await == ClientState::Ready
    }
}

/// Process an incoming frame
async fn process_frame(
    frame: Frame,
    services: &Arc<RwLock<HashMap<ServiceType, ServiceClient>>>,
) -> Result<()> {
    match frame.command_id() {
        cmd if cmd == CommandId::Heartbeat as u8 => {
            // Heartbeat - no action needed
            trace!("Received heartbeat");
            Ok(())
        }
        cmd if cmd == CommandId::Error as u8 => {
            // Error from server
            let error_msg = String::from_utf8_lossy(frame.payload()).to_string();
            warn!("Received error from server: {}", error_msg);
            Ok(())
        }
        cmd if cmd == CommandId::StreamFrame as u8 => {
            // Forward to display service
            let services_guard = services.read().await;
            if let Some(service) = services_guard.get(&ServiceType::Display) {
                // Use fire and forget since this is streaming data
                let _ = service.send_fire_and_forget(frame).await;
            }
            Ok(())
        }
        cmd if cmd == CommandId::DisplayInfo as u8 => {
            // Forward to display service
            let services_guard = services.read().await;
            if let Some(service) = services_guard.get(&ServiceType::Display) {
                // Use fire and forget for display info
                let _ = service.send_fire_and_forget(frame).await;
            }
            Ok(())
        }
        // Handle other commands as needed
        _ => {
            debug!("Unhandled command: {:02x}", frame.command_id());
            Ok(())
        }
    }
}
