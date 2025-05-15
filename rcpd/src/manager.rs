use crate::{config::ServiceConfig, error::ServiceError, server::Server};
// Conditionally import API types
#[cfg(feature = "api")]
use crate::api::ApiServer;

use log::{debug, error, info};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/// Manages the RCP service including the integrated server and API
pub struct ServiceManager {
    /// Path to the working directory
    work_dir: PathBuf,

    /// Service configuration
    config: ServiceConfig,

    /// Shutdown channel sender
    shutdown_tx: mpsc::Sender<()>,

    /// Integrated server instance
    server: Option<Arc<Mutex<Server>>>,

    /// Integrated API instance (when api feature is enabled)
    #[cfg(feature = "api")]
    api: Option<ApiServer>,
}

impl ServiceManager {
    /// Create a new service manager
    pub fn new(work_dir: PathBuf, config: ServiceConfig, shutdown_tx: mpsc::Sender<()>) -> Self {
        #[cfg(feature = "api")]
        {
            Self {
                work_dir,
                config,
                shutdown_tx,
                server: None,
                api: None,
            }
        }

        #[cfg(not(feature = "api"))]
        Self {
            work_dir,
            config,
            shutdown_tx,
            server: None,
        }
    }

    /// Start the service and the integrated components
    pub async fn start(&mut self) -> Result<(), ServiceError> {
        info!("Starting RCP service");

        // Initialize and start the integrated server
        info!("Initializing integrated RCP server");
        let server = Server::new(self.config.server.clone());
        let server_arc = Arc::new(Mutex::new(server));

        // Clone for the server task
        let server_task_arc = server_arc.clone();

        // Start the server in a separate task
        tokio::spawn(async move {
            let server = server_task_arc.lock().await.clone();
            if let Err(e) = server.run().await {
                error!("Server error: {}", e);
            }
        });

        // Store the server reference
        self.server = Some(server_arc);

        // Initialize and start the API server if the feature is enabled
        #[cfg(feature = "api")]
        {
            // Check if API is enabled in the configuration
            if let Some(api_config) = &self.config.api {
                info!("Initializing integrated API server");

                // Create API server with access to service manager
                let api_server =
                    ApiServer::new(api_config.clone(), Arc::new(Mutex::new(self.clone())));

                // Start the API server
                if let Err(e) = api_server.start().await {
                    error!("Error starting API server: {}", e);
                    // Continue anyway, as the API is an optional component
                } else {
                    info!("API server started successfully");
                    self.api = Some(api_server);
                }
            } else {
                debug!("API server not enabled in configuration");
            }
        }

        info!("RCP service started successfully");
        Ok(())
    }

    /// Stop the service and all integrated components
    pub async fn stop(&mut self) -> Result<(), ServiceError> {
        info!("Stopping RCP service");

        // Stop the integrated API server if running
        #[cfg(feature = "api")]
        if let Some(api) = &self.api {
            if api.is_running().await {
                info!("Stopping integrated API server");
                if let Err(e) = api.stop().await {
                    error!("Error stopping API server: {}", e);
                }
            }
        }

        // Stop the integrated server if running
        if let Some(server_arc) = &self.server {
            let server = server_arc.lock().await;
            match server.is_running().await {
                true => {
                    info!("Stopping integrated RCP server");
                    if let Err(e) = server.stop().await {
                        error!("Error stopping server: {}", e);
                    }
                }
                false => debug!("Server not running, no need to stop"),
            }
        }

        // Send shutdown signal
        if self.shutdown_tx.send(()).await.is_err() {
            error!("Failed to send shutdown signal");
        }

        info!("RCP service stopped");
        Ok(())
    }

    /// Get server status information
    pub async fn server_status(&self) -> Option<ServerStatus> {
        if let Some(server_arc) = &self.server {
            let server = server_arc.lock().await;

            // Get server information
            let running = server.is_running().await;

            let uptime = server.uptime().await;
            let sessions = match running {
                true => {
                    let sessions = server.get_sessions().await;
                    Some(sessions.len())
                }
                false => None,
            };

            Some(ServerStatus {
                running,
                uptime,
                sessions,
            })
        } else {
            None
        }
    }

    /// Get server reference
    pub fn get_server(&self) -> &Option<Arc<Mutex<Server>>> {
        &self.server
    }

    /// Get service configuration
    pub fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    /// Get the working directory path
    pub fn get_work_dir(&self) -> &PathBuf {
        &self.work_dir
    }

    /// Get API status information
    #[cfg(feature = "api")]
    pub async fn api_status(&self) -> Option<ApiStatus> {
        if let Some(api) = &self.api {
            let running = api.is_running().await;

            Some(ApiStatus {
                running,
                address: format!("{}:{}", api.config.address, api.config.port),
            })
        } else {
            None
        }
    }

    /// Get API server reference
    #[cfg(feature = "api")]
    pub fn get_api(&self) -> &Option<ApiServer> {
        &self.api
    }
}

/// Server status information
pub struct ServerStatus {
    /// Whether the server is running
    pub running: bool,

    /// Server uptime if running
    pub uptime: Option<std::time::Duration>,

    /// Number of active sessions
    pub sessions: Option<usize>,
}

// Implement Clone for ServiceManager
impl Clone for ServiceManager {
    fn clone(&self) -> Self {
        #[cfg(feature = "api")]
        {
            Self {
                work_dir: self.work_dir.clone(),
                config: self.config.clone(),
                shutdown_tx: self.shutdown_tx.clone(),
                server: self.server.clone(),
                api: None, // API is not clonable and not needed in clones
            }
        }

        #[cfg(not(feature = "api"))]
        Self {
            work_dir: self.work_dir.clone(),
            config: self.config.clone(),
            shutdown_tx: self.shutdown_tx.clone(),
            server: self.server.clone(),
        }
    }
}

/// API status information
#[cfg(feature = "api")]
pub struct ApiStatus {
    /// Whether the API server is running
    pub running: bool,

    /// API server address and port
    pub address: String,
}
