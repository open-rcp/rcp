#[cfg(feature = "api")]
use crate::{
    api::config::ApiConfig,
    config::ServiceConfig,
    error::ServiceError,
    manager::ServiceManager,
    // handlers module is not used directly anymore
    server::Server,
};
use axum::Json;
use serde_json;

use axum::{
    http::{HeaderValue, Method},
    routing::{get, post}, // Only using get and post routes
    Router,
};
use log::{error, info}; // debug is unused
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// API server component
pub struct ApiServer {
    /// API configuration
    pub config: ApiConfig,

    /// Service manager reference for direct access
    service_manager: Arc<Mutex<ServiceManager>>,

    /// Whether the API server is running
    running: Arc<Mutex<bool>>,
}

/// API application state shared across handlers
/// Currently not actively used in routes, but kept for future use
#[derive(Clone)]
#[allow(dead_code)]
pub struct ApiState {
    /// API configuration
    pub config: Arc<ApiConfig>,

    /// Service configuration
    pub service_config: Arc<ServiceConfig>,

    /// Service manager reference
    pub service_manager: Arc<Mutex<ServiceManager>>,

    /// Server reference (when available)
    pub server: Option<Arc<Mutex<Server>>>,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(config: ApiConfig, service_manager: Arc<Mutex<ServiceManager>>) -> Self {
        Self {
            config,
            service_manager,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start the API server
    pub async fn start(&self) -> Result<(), ServiceError> {
        let addr = format!("{}:{}", self.config.address, self.config.port);
        info!("Starting API server on {}", addr);

        // Set running state
        {
            let mut running = self.running.lock().await;
            *running = true;
        }

        // Get service manager reference and config
        let service_manager_lock = self.service_manager.lock().await;
        let service_config = service_manager_lock.get_config().clone();
        let service_manager = self.service_manager.clone();
        drop(service_manager_lock);

        // Get server reference (if available)
        let server = {
            let service_manager = service_manager.lock().await;
            service_manager.get_server().clone()
        };

        // Create API state
        let api_state = ApiState {
            config: Arc::new(self.config.clone()),
            service_config: Arc::new(service_config),
            service_manager: self.service_manager.clone(),
            server,
        };

        // Configure CORS
        let cors = self.configure_cors();

        // Build the router with simple placeholder routes for now
        let app = Router::new()
            // Basic endpoints
            .route("/", get(|| async { "RCP API Server" }))
            .route(
                "/health",
                get(|| async {
                    Json(serde_json::json!({
                        "status": "ok",
                        "version": env!("CARGO_PKG_VERSION")
                    }))
                }),
            )
            // Service endpoints
            .route(
                "/v1/status",
                get(|| async {
                    Json(serde_json::json!({
                        "service": "running",
                        "server": {
                            "running": false,
                            "sessions": null
                        }
                    }))
                }),
            )
            .route(
                "/v1/config",
                get(|| async {
                    Json(serde_json::json!({
                        "service_address": "0.0.0.0",
                        "service_port": 55555,
                        "server_enabled": true,
                        "api_enabled": true
                    }))
                }),
            )
            // Server management endpoints
            .route(
                "/v1/server/start",
                post(|| async {
                    Json(serde_json::json!({
                        "action": "start",
                        "result": "not_available"
                    }))
                }),
            )
            .route(
                "/v1/server/stop",
                post(|| async {
                    Json(serde_json::json!({
                        "action": "stop",
                        "result": "not_available"
                    }))
                }),
            )
            .route(
                "/v1/server/sessions",
                get(|| async {
                    Json(serde_json::json!({
                        "count": 0,
                        "sessions": []
                    }))
                }),
            )
            // Add tracing and CORS
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            .with_state(api_state);

        // Parse the address
        let addr: SocketAddr = addr
            .parse()
            .map_err(|e| ServiceError::Api(format!("Invalid API address: {}", e)))?;

        // Start the server in a separate task
        let running = self.running.clone();
        tokio::spawn(async move {
            info!("API server listening on {}", addr);
            if let Err(e) = axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
            {
                error!("API server error: {}", e);
                // Update running state
                let mut running_guard = running.lock().await;
                *running_guard = false;
            }
        });

        Ok(())
    }

    /// Stop the API server
    pub async fn stop(&self) -> Result<(), ServiceError> {
        info!("Stopping API server");

        // Update running state
        let mut running = self.running.lock().await;
        *running = false;

        // Note: Axum doesn't provide a clean way to stop the server
        // In a production environment, we would need a more robust solution
        // For now, we just update the state and let the server continue running
        // until the process terminates

        Ok(())
    }

    /// Configure CORS for the API server
    fn configure_cors(&self) -> CorsLayer {
        let mut cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(Any);

        // Add allowed origins from configuration
        if !self.config.cors_allowed_origins.is_empty() {
            let origins = self
                .config
                .cors_allowed_origins
                .iter()
                .filter_map(|origin| origin.parse::<HeaderValue>().ok())
                .collect::<Vec<_>>();

            cors = cors.allow_origin(origins);
        } else {
            // Default to allowing any origin if none configured
            cors = cors.allow_origin(Any);
        }

        cors
    }

    /// Check if the API server is running
    pub async fn is_running(&self) -> bool {
        let running = self.running.lock().await;
        *running
    }
}

// All API handler functionality is now in the handlers module
