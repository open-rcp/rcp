use crate::{
    config::ServerConfig,
    error::{Error, Result},
    session::Session,
};
use log::{debug, error, info};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// The main RCP server that accepts connections and manages sessions
pub struct Server {
    /// Server configuration
    config: ServerConfig,
    
    /// Active sessions
    sessions: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Session>>>>>,
}

impl Server {
    /// Create a new server with the given configuration
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Run the server and start accepting connections
    pub async fn run(self) -> Result<()> {
        let addr = self.config.server_addr();
        info!("Starting RCP server on {}", addr);
        
        let listener = TcpListener::bind(&addr).await?;
        
        loop {
            match listener.accept().await {
                Ok((socket, peer_addr)) => {
                    info!("New connection from {}", peer_addr);
                    
                    // Check session limit
                    let session_count = self.sessions.lock().await.len();
                    if session_count >= self.config.session.max_sessions {
                        error!("Session limit reached ({}), rejecting connection", self.config.session.max_sessions);
                        continue;
                    }
                    
                    // Clone what we need for the connection task
                    let config = self.config.clone();
                    let sessions = Arc::clone(&self.sessions);
                    
                    // Spawn a new task to handle the connection
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(socket, peer_addr.to_string(), config, sessions).await {
                            error!("Error handling connection from {}: {}", peer_addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
    
    /// Handle a new connection
    async fn handle_connection(
        socket: tokio::net::TcpStream,
        peer_addr: String,
        config: ServerConfig,
        sessions: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Session>>>>>,
    ) -> Result<()> {
        // Set up the protocol handler
        let tcp_stream = socket;
        
        // Create a new session
        let session_id = Uuid::new_v4();
        let session = Session::new(session_id, tcp_stream, config.clone(), peer_addr)?;
        let session = Arc::new(Mutex::new(session));
        
        // Store the session
        sessions.lock().await.insert(session_id, Arc::clone(&session));
        
        // Start the session
        let result = session.lock().await.start().await;
        
        // Clean up the session when it's done
        sessions.lock().await.remove(&session_id);
        debug!("Session {} removed", session_id);
        
        result
    }
}