use crate::{
    config::ServerConfig,
    error::{Error, Result},
    service::Service,
};
use log::{debug, error, info, warn};
use rcp_core::{
    Auth, AuthChallenge, AuthMethod, AuthPayload, AuthResponse, CommandId, ConnectionState,
    Frame, Protocol, SessionInfo,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use uuid::Uuid;

/// A client session
pub struct Session {
    /// Session ID
    pub id: Uuid,
    
    /// Protocol handler
    protocol: Protocol<TcpStream>,
    
    /// Server configuration
    config: ServerConfig,
    
    /// Peer address
    peer_addr: String,
    
    /// Session state
    state: ConnectionState,
    
    /// Client ID
    client_id: Option<Uuid>,
    
    /// Client name
    client_name: Option<String>,
    
    /// Session permissions
    permissions: Vec<String>,
    
    /// Active services
    services: HashMap<String, Arc<Mutex<dyn Service + Send>>>,
}

impl Session {
    /// Create a new session
    pub fn new(
        id: Uuid,
        stream: TcpStream,
        config: ServerConfig,
        peer_addr: String,
    ) -> Result<Self> {
        Ok(Self {
            id,
            protocol: Protocol::new(stream),
            config,
            peer_addr,
            state: ConnectionState::Connected,
            client_id: None,
            client_name: None,
            permissions: Vec::new(),
            services: HashMap::new(),
        })
    }
    
    /// Start the session
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting session {}", self.id);
        
        // Authenticate the client
        self.authenticate().await?;
        
        // Main session loop
        loop {
            match self.protocol.read_frame().await? {
                Some(frame) => {
                    if let Err(e) = self.handle_frame(frame).await {
                        error!("Error handling frame: {}", e);
                        // Send error to client
                        let error_frame = Frame::new(
                            CommandId::Error as u8,
                            format!("Error: {}", e).into_bytes(),
                        );
                        if let Err(e) = self.protocol.write_frame(&error_frame).await {
                            error!("Failed to send error frame: {}", e);
                            break;
                        }
                    }
                }
                None => {
                    info!("Client disconnected, ending session {}", self.id);
                    break;
                }
            }
        }
        
        // Clean up
        self.close().await?;
        
        Ok(())
    }
    
    /// Handle an incoming frame
    async fn handle_frame(&mut self, frame: Frame) -> Result<()> {
        match frame.command_id() {
            cmd if cmd == CommandId::Heartbeat as u8 => {
                // Reply with heartbeat
                let response = Frame::new(CommandId::Heartbeat as u8, Vec::new());
                self.protocol.write_frame(&response).await?;
            }
            cmd if cmd == CommandId::LaunchApp as u8 => {
                // Check permissions
                if !self.has_permission("app:launch") {
                    return Err(Error::PermissionDenied("No permission to launch applications".to_string()));
                }
                
                // TODO: Implement app launching
                warn!("Application launch not implemented yet");
            }
            cmd if cmd == CommandId::ServiceSubscribe as u8 => {
                // Handle service subscription
                // TODO: Parse service name from frame
                let service_name = String::from_utf8_lossy(&frame.payload());
                info!("Client requested subscription to service: {}", service_name);
                
                // Check permissions
                if !self.has_permission(&format!("service:{}", service_name)) {
                    return Err(Error::PermissionDenied(format!("No permission to use service {}", service_name)));
                }
                
                // TODO: Register service
                warn!("Service subscription not implemented yet");
            }
            cmd if cmd == CommandId::Ping as u8 => {
                // Reply with pong (same command ID)
                let response = Frame::new(CommandId::Ping as u8, frame.payload().to_vec());
                self.protocol.write_frame(&response).await?;
            }
            cmd => {
                warn!("Unhandled command: {:02x}", cmd);
            }
        }
        
        Ok(())
    }
    
    /// Authenticate the client
    async fn authenticate(&mut self) -> Result<()> {
        debug!("Starting authentication for session {}", self.id);
        self.protocol.set_state(ConnectionState::Authenticating);
        
        // Wait for authentication request
        let auth_frame = match self.protocol.read_frame().await? {
            Some(frame) if frame.command_id() == CommandId::Auth as u8 => frame,
            Some(_) => return Err(Error::Authentication("Expected AUTH command".to_string())),
            None => return Err(Error::Authentication("Connection closed during authentication".to_string())),
        };
        
        // Parse authentication payload
        let auth_payload: AuthPayload = rcp_core::utils::from_bytes(&auth_frame.payload())?;
        
        self.client_id = Some(auth_payload.client_id);
        self.client_name = Some(auth_payload.client_name.clone());
        
        info!("Client {} ({}) requesting authentication", auth_payload.client_name, auth_payload.client_id);
        
        // Check if authentication is required
        if !self.config.auth.required {
            debug!("Authentication not required, granting access");
            return self.complete_authentication(Vec::from(["*".to_string()])).await;
        }
        
        // Check allowed clients if configured
        if !self.config.auth.allowed_clients.is_empty() {
            let client_id_str = auth_payload.client_id.to_string();
            if !self.config.auth.allowed_clients.contains(&client_id_str) {
                return Err(Error::Authentication("Client not in allowed list".to_string()));
            }
        }
        
        // Handle by auth method
        match auth_payload.auth_method {
            AuthMethod::PreSharedKey => self.authenticate_psk(auth_payload).await,
            AuthMethod::PublicKey => {
                warn!("PublicKey authentication not implemented yet");
                Err(Error::Authentication("PublicKey authentication not supported".to_string()))
            }
            AuthMethod::TwoFactor => {
                warn!("TwoFactor authentication not implemented yet");
                Err(Error::Authentication("TwoFactor authentication not supported".to_string()))
            }
        }
    }
    
    /// Authenticate using pre-shared key method
    async fn authenticate_psk(&mut self, auth_payload: AuthPayload) -> Result<()> {
        // Check if PSK is configured
        let psk = match &self.config.auth.psk {
            Some(key) => key,
            None => return Err(Error::Authentication("PSK authentication not configured".to_string())),
        };
        
        // Generate challenge
        let challenge = Auth::generate_challenge();
        
        // Send challenge to client
        let challenge_data = rcp_core::utils::to_bytes(&challenge)?;
        let challenge_frame = Frame::new(CommandId::Auth as u8, challenge_data);
        self.protocol.write_frame(&challenge_frame).await?;
        
        // Wait for response
        let response_frame = match self.protocol.read_frame().await? {
            Some(frame) if frame.command_id() == CommandId::Auth as u8 => frame,
            Some(_) => return Err(Error::Authentication("Expected AUTH response".to_string())),
            None => return Err(Error::Authentication("Connection closed during authentication".to_string())),
        };
        
        // Parse response
        let auth_response: AuthResponse = rcp_core::utils::from_bytes(&response_frame.payload())?;
        
        // Verify response
        if Auth::verify_psk(psk, &challenge.challenge, &challenge.salt, &auth_response.response) {
            // Authentication successful
            info!("Authentication successful for client {}", auth_payload.client_name);
            
            // Grant permissions
            let permissions = vec![
                "display".to_string(),
                "input".to_string(),
                "app:launch".to_string(),
            ];
            
            self.complete_authentication(permissions).await
        } else {
            Err(Error::Authentication("Invalid authentication response".to_string()))
        }
    }
    
    /// Complete the authentication process
    async fn complete_authentication(&mut self, permissions: Vec<String>) -> Result<()> {
        self.permissions = permissions;
        
        // Create session info
        let session_info = Auth::create_session(
            self.permissions.clone(),
            self.config.session.timeout_secs,
        );
        
        // Send session info to client
        let session_data = rcp_core::utils::to_bytes(&session_info)?;
        let session_frame = Frame::new(CommandId::Auth as u8, session_data);
        self.protocol.write_frame(&session_frame).await?;
        
        // Update session state
        self.protocol.set_state(ConnectionState::Authenticated);
        self.state = ConnectionState::Authenticated;
        
        info!("Session {} authenticated", self.id);
        Ok(())
    }
    
    /// Check if the session has a specific permission
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string()) || 
        self.permissions.contains(&"*".to_string())
    }
    
    /// Close the session
    pub async fn close(&mut self) -> Result<()> {
        info!("Closing session {}", self.id);
        
        // Clean up services
        for (name, service) in self.services.drain() {
            debug!("Stopping service: {}", name);
            // TODO: Add proper service shutdown
        }
        
        // Close the protocol
        self.protocol.close().await?;
        
        Ok(())
    }
}