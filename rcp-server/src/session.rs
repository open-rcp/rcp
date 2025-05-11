use crate::{
    config::ServerConfig,
    error::{Error, Result},
    service::{services::ServiceFactory, Service},
};
use log::{debug, error, info, warn};
use rcp_core::{
    Auth, AuthMethod, AuthPayload, AuthResponse, CommandId, ConnectionState, Frame, Protocol,
};
use std::collections::HashMap;
use tokio::net::TcpStream;
use uuid::Uuid;

/// A client session on the server
pub struct Session {
    /// Session ID
    pub id: Uuid,

    /// Protocol handler
    protocol: Protocol<TcpStream>,

    /// Server configuration
    config: ServerConfig,

    /// Peer address
    #[allow(dead_code)]
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
    services: HashMap<String, Box<dyn Service + Send>>,
}

impl Session {
    /// Create a new session
    pub fn new(
        id: Uuid,
        tcp_stream: TcpStream,
        config: ServerConfig,
        peer_addr: String,
    ) -> Result<Self> {
        let protocol = Protocol::new(tcp_stream);
        Ok(Self {
            id,
            protocol,
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
                        break;
                    }
                }
                None => {
                    // Connection closed
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
        self.process_frame(&frame).await
    }

    /// Process a frame
    async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
        let cmd_id = frame.command_id();
        log::debug!(
            "Processing frame with command ID {:02x} for session {}",
            cmd_id,
            self.id
        );

        match cmd_id {
            cmd if cmd == CommandId::Heartbeat as u8 => {
                // Reply with heartbeat
                let response = Frame::new(CommandId::Heartbeat as u8, Vec::new());
                self.protocol.write_frame(&response).await?;
            }
            cmd if cmd == CommandId::LaunchApp as u8 => {
                // Check permissions
                if !self.has_permission("app:launch") {
                    return Err(Error::PermissionDenied(
                        "No permission to launch applications".to_string(),
                    ));
                }

                // Make sure we have the app service
                if !self.services.contains_key("app") {
                    // Try to subscribe to app service if not already subscribed
                    self.subscribe_service("app").await?;
                }

                // Forward the LaunchApp command to the app service
                if let Some(service) = self.services.get_mut("app") {
                    info!("Forwarding LaunchApp command to app service");
                    service.process_frame(frame).await?;
                } else {
                    return Err(Error::Service("App service not available".to_string()));
                }
            }
            cmd if cmd == CommandId::ServiceSubscribe as u8 => {
                let service_name = String::from_utf8_lossy(frame.payload()).to_string();
                info!(
                    "Received service subscription request for {} from session {}",
                    service_name, self.id
                );

                // Check if we're allowed to access this service
                if !self.has_permission(&service_name) && !self.has_permission("*") {
                    return Err(Error::PermissionDenied(format!(
                        "No permission to access service: {}",
                        service_name
                    )));
                }

                self.subscribe_service(&service_name).await?;

                // Send acknowledgment to the client
                let ack_frame = Frame::new(CommandId::Ack as u8, service_name.into_bytes());
                self.protocol.write_frame(&ack_frame).await?;
            }
            cmd if cmd == CommandId::Ping as u8 => {
                info!("Client requested disconnect for session {}", self.id);
                let response = Frame::new(CommandId::Ping as u8, Vec::new());
                self.protocol.write_frame(&response).await?;
            }
            // Commands that may be handled by services
            cmd if cmd == CommandId::VideoQuality as u8
                || cmd == CommandId::SendInput as u8
                || cmd == CommandId::ClipboardData as u8 =>
            {
                let mut handled = false;
                // Try to find a service that can handle this command
                for (name, service) in &mut self.services {
                    match service.process_frame(frame).await {
                        Ok(_) => {
                            log::info!(
                                "Frame with command {:02x} processed by service {} for session {}",
                                cmd_id,
                                name,
                                self.id
                            );
                            handled = true;
                            break;
                        }
                        Err(e) => {
                            // Log but continue trying other services
                            log::debug!("Service {} could not process frame: {}", name, e);
                        }
                    }
                }

                if !handled {
                    // No service could handle this frame
                    log::warn!(
                        "No service could handle command {:02x} for session {}",
                        cmd_id,
                        self.id
                    );
                    return Err(Error::Protocol(format!(
                        "Unhandled command: {:02x}",
                        cmd_id
                    )));
                }
            }
            _ => {
                // Unrecognized command
                log::warn!(
                    "Unrecognized command {:02x} for session {}",
                    cmd_id,
                    self.id
                );
                return Err(Error::Protocol(format!(
                    "Unrecognized command: {:02x}",
                    cmd_id
                )));
            }
        }

        // Check for service frames to send
        self.process_service_frames().await?;

        Ok(())
    }

    /// Process outgoing frames from services
    async fn process_service_frames(&mut self) -> Result<()> {
        // Poll each service for frames to send
        for (name, service) in &mut self.services {
            if let Ok(Some(frame)) = service.get_frame().await {
                log::debug!(
                    "Sending frame from service {} to client for session {}",
                    name,
                    self.id
                );
                if let Err(e) = self.protocol.write_frame(&frame).await {
                    log::error!("Failed to send frame to client: {}", e);
                    return Err(Error::Core(e));
                }
            }
        }

        Ok(())
    }

    /// Subscribe to a service
    async fn subscribe_service(&mut self, service_name: &str) -> Result<()> {
        if self.services.contains_key(service_name) {
            log::info!(
                "Service {} already subscribed for session {}",
                service_name,
                self.id
            );
            return Ok(());
        }

        log::info!(
            "Subscribing to service {} for session {}",
            service_name,
            self.id
        );
        let mut service = ServiceFactory::create(service_name)
            .ok_or_else(|| Error::Service(format!("Service not available: {}", service_name)))?;

        // Start the service
        service.start().await?;

        // Add to services map
        self.services.insert(service_name.to_string(), service);

        log::info!(
            "Service {} successfully subscribed for session {}",
            service_name,
            self.id
        );

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
            None => {
                return Err(Error::Authentication(
                    "Connection closed during authentication".to_string(),
                ))
            }
        };

        // Parse authentication payload
        let auth_payload: AuthPayload = rcp_core::utils::from_bytes(auth_frame.payload())?;

        self.client_id = Some(auth_payload.client_id);
        self.client_name = Some(auth_payload.client_name.clone());

        info!(
            "Client {} ({}) requesting authentication",
            auth_payload.client_name, auth_payload.client_id
        );

        // Check if authentication is required
        if !self.config.auth.required {
            debug!("Authentication not required, granting access");
            return self
                .complete_authentication(Vec::from(["*".to_string()]))
                .await;
        }

        // Check allowed clients if configured
        if !self.config.auth.allowed_clients.is_empty() {
            let client_id_str = auth_payload.client_id.to_string();
            if !self.config.auth.allowed_clients.contains(&client_id_str) {
                return Err(Error::Authentication(
                    "Client not in allowed list".to_string(),
                ));
            }
        }

        // Handle by auth method
        match auth_payload.auth_method {
            AuthMethod::PreSharedKey => self.authenticate_psk(auth_payload).await,
            AuthMethod::PublicKey => {
                warn!("PublicKey authentication not implemented yet");
                Err(Error::Authentication(
                    "PublicKey authentication not supported".to_string(),
                ))
            }
            AuthMethod::TwoFactor => {
                warn!("TwoFactor authentication not implemented yet");
                Err(Error::Authentication(
                    "TwoFactor authentication not supported".to_string(),
                ))
            }
            AuthMethod::Password(_, _) => {
                // For now, handle Password authentication the same way as PSK
                warn!("Using PSK for Password authentication");
                self.authenticate_psk(auth_payload).await
            }
        }
    }

    /// Authenticate using pre-shared key method
    async fn authenticate_psk(&mut self, auth_payload: AuthPayload) -> Result<()> {
        // Check if PSK is configured
        let psk = match &self.config.auth.psk {
            Some(key) => key,
            None => {
                return Err(Error::Authentication(
                    "PSK authentication not configured".to_string(),
                ))
            }
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
            None => {
                return Err(Error::Authentication(
                    "Connection closed during authentication".to_string(),
                ))
            }
        };

        // Parse response
        let auth_response: AuthResponse = rcp_core::utils::from_bytes(response_frame.payload())?;

        // Verify response
        if Auth::verify_psk(
            psk,
            &challenge.challenge,
            &challenge.salt,
            &auth_response.response,
        ) {
            // Authentication successful
            info!(
                "Authentication successful for client {}",
                auth_payload.client_name
            );

            // Grant permissions
            let permissions = vec![
                "display".to_string(),
                "input".to_string(),
                "app".to_string(), // Added app service permission
                "app:launch".to_string(),
            ];

            self.complete_authentication(permissions).await
        } else {
            Err(Error::Authentication(
                "Invalid authentication response".to_string(),
            ))
        }
    }

    /// Complete the authentication process
    async fn complete_authentication(&mut self, permissions: Vec<String>) -> Result<()> {
        self.permissions = permissions;

        // Create session info
        let session_info =
            Auth::create_session(self.permissions.clone(), self.config.session.timeout_secs);

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
        self.permissions.contains(&permission.to_string())
            || self.permissions.contains(&"*".to_string())
    }

    /// Close the session
    pub async fn close(&mut self) -> Result<()> {
        info!("Closing session {}", self.id);

        // Clean up services
        for (name, mut service) in self.services.drain() {
            debug!("Stopping service: {}", name);
            if let Err(e) = service.stop().await {
                warn!("Error stopping service {}: {}", name, e);
            }
        }

        // Close the protocol
        self.protocol.close().await?;

        Ok(())
    }
}
