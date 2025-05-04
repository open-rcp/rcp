use crate::{config::ServerConfig, error::Result, session::Session};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use uuid::Uuid;

/// The main RCP server that accepts connections and manages sessions
#[derive(Clone)]
pub struct Server {
    /// Server configuration
    config: ServerConfig,

    /// Active sessions
    sessions: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Session>>>>>,

    /// Server state
    running: Arc<Mutex<bool>>,

    /// Server start time
    start_time: Arc<Mutex<Option<Instant>>>,
}

impl Server {
    /// Create a new server with the given configuration
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(false)),
            start_time: Arc::new(Mutex::new(None)),
        }
    }

    /// Run the server and start accepting connections
    pub async fn run(self) -> Result<()> {
        let addr = self.config.server_addr();
        info!("Starting RCP server on {}", addr);

        let listener = TcpListener::bind(&addr).await?;

        // Mark server as running and set start time
        {
            let mut running_guard = self.running.lock().await;
            *running_guard = true;

            let mut start_time_guard = self.start_time.lock().await;
            *start_time_guard = Some(Instant::now());
        }

        loop {
            // Check if we should stop
            if !*self.running.lock().await {
                info!("Server shutdown requested");
                break;
            }

            // Use tokio::select to handle both accepting connections and shutdown requests
            let accept_result = tokio::select! {
                result = listener.accept() => result,
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    // Just a timeout to check for shutdown periodically
                    continue;
                }
            };

            match accept_result {
                Ok((socket, peer_addr)) => {
                    info!("New connection from {}", peer_addr);

                    // Check session limit
                    let session_count = self.sessions.lock().await.len();
                    if session_count >= self.config.session.max_sessions {
                        error!(
                            "Session limit reached ({}), rejecting connection",
                            self.config.session.max_sessions
                        );
                        continue;
                    }

                    // Clone what we need for the connection task
                    let config = self.config.clone();
                    let sessions = Arc::clone(&self.sessions);

                    // Spawn a new task to handle the connection
                    tokio::spawn(async move {
                        if let Err(e) =
                            Self::handle_connection(socket, peer_addr.to_string(), config, sessions)
                                .await
                        {
                            error!("Error handling connection from {}: {}", peer_addr, e);
                        }
                    });
                }
                Err(e) => error!("Error accepting connection: {}", e),
            }
        }

        info!("Server shutdown complete");
        Ok(())
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
        sessions
            .lock()
            .await
            .insert(session_id, Arc::clone(&session));

        // Start the session
        let result = session.lock().await.start().await;

        // Clean up the session when it's done
        sessions.lock().await.remove(&session_id);
        debug!("Session {} removed", session_id);

        result
    }

    // -- Management API methods --

    /// Check if the server is currently running
    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }

    /// Get the number of active sessions
    pub async fn active_session_count(&self) -> usize {
        self.sessions.lock().await.len()
    }

    /// Get the number of connected clients
    pub async fn connected_client_count(&self) -> usize {
        // In this implementation, each session corresponds to one client
        self.active_session_count().await
    }

    /// Get the uptime of the server in seconds
    pub async fn uptime(&self) -> u64 {
        let start_time_guard = self.start_time.lock().await;
        match *start_time_guard {
            Some(start_time) => start_time.elapsed().as_secs(),
            None => 0,
        }
    }

    /// Get the uptime of the server as a formatted string
    pub async fn uptime_formatted(&self) -> String {
        let secs = self.uptime().await;
        let days = secs / 86400;
        let hours = (secs % 86400) / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    /// Stop the server
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping server...");
        let mut running_guard = self.running.lock().await;
        *running_guard = false;
        Ok(())
    }

    /// Start the server (if not already running)
    pub async fn start(&self) -> Result<()> {
        let already_running = *self.running.lock().await;

        if already_running {
            info!("Server is already running");
            return Ok(());
        }

        // Clone self to pass to new task
        let server_clone = self.clone();

        // Spawn a task to run the server
        tokio::spawn(async move {
            if let Err(e) = server_clone.run().await {
                error!("Error running server: {}", e);
            }
        });

        Ok(())
    }

    /// Restart the server
    pub async fn restart(&self) -> Result<()> {
        info!("Restarting server...");

        // Stop server
        self.stop().await?;

        // Wait a moment for connections to close
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Start server again
        self.start().await
    }
}
