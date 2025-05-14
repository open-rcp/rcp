use crate::server::{config::ServerConfig, error::Result, session::Session};
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
        let addr = format!("{}:{}", self.config.address, self.config.port);
        info!("Starting RCP server on {}", addr);

        let listener = TcpListener::bind(&addr).await?;

        // Mark server as running and set start time
        {
            let mut running_guard = self.running.lock().await;
            *running_guard = true;

            let mut start_time_guard = self.start_time.lock().await;
            *start_time_guard = Some(Instant::now());
        }

        // Accept connections
        while let Ok((socket, peer_addr)) = listener.accept().await {
            let peer_addr_str = peer_addr.to_string();
            info!("Accepted connection from: {}", peer_addr_str);

            // Create a new session
            let session_id = Uuid::new_v4();
            let session = Session::new(session_id, socket, self.config.clone(), peer_addr_str);

            // Store the session
            {
                let mut sessions = self.sessions.lock().await;
                sessions.insert(session_id, Arc::new(Mutex::new(session)));
            }

            // Spawn a task to handle the session
            let server_clone = self.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.handle_session(session_id).await {
                    error!("Session error: {}", e);
                }

                // Always clean up the session
                let _ = server_clone.remove_session(session_id).await;
            });
        }

        Ok(())
    }

    /// Handle a client session
    async fn handle_session(&self, session_id: Uuid) -> Result<()> {
        let session_arc = {
            let sessions = self.sessions.lock().await;
            sessions.get(&session_id).cloned()
        };

        if let Some(session_mutex) = session_arc {
            let mut session = session_mutex.lock().await;
            session.process().await
        } else {
            error!("Session not found: {}", session_id);
            Err(crate::server::error::Error::NotFound(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Remove a session
    async fn remove_session(&self, session_id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;

        if let Some(session_arc) = sessions.get(&session_id) {
            // Try to disconnect the session properly
            let mut session = session_arc.lock().await;
            let _ = session.disconnect().await;
        }

        sessions.remove(&session_id);
        debug!("Session removed: {}", session_id);
        Ok(())
    }

    /// Get all active sessions
    pub async fn get_sessions(&self) -> Vec<Uuid> {
        let sessions = self.sessions.lock().await;
        sessions.keys().cloned().collect()
    }

    /// Get the server uptime
    pub async fn uptime(&self) -> Option<Duration> {
        let start_time = self.start_time.lock().await;
        start_time.map(|t| t.elapsed())
    }

    /// Check if the server is running
    pub async fn is_running(&self) -> bool {
        let running = self.running.lock().await;
        *running
    }

    /// Stop the server
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping RCP server");

        // Set running to false
        {
            let mut running = self.running.lock().await;
            *running = false;
        }

        // Disconnect all sessions
        let sessions = self.sessions.lock().await;
        for (session_id, session) in sessions.iter() {
            debug!("Disconnecting session: {}", session_id);
            let mut session = session.lock().await;
            if let Err(e) = session.disconnect().await {
                error!("Error disconnecting session {}: {}", session_id, e);
            }
        }

        // Reset start time
        let mut start_time = self.start_time.lock().await;
        *start_time = None;

        Ok(())
    }
}
