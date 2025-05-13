#[cfg(feature = "api")]
//! API server implementation

use crate::{
    api::config::ApiConfig,
    api::handlers,
    server::Server,
    config::ServiceConfig,
    error::ServiceError,
    manager::ServiceManager,
};

use axum::{
    http::{HeaderValue, Method},
    routing::{get, post, put, delete},
    Router,
};
use log::{debug, error, info};
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
#[derive(Clone)]
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
        
        // Build the router with all API endpoints
        let app = Router::new()
            // Basic endpoints
            .route("/", get(crate::api::handlers::root))
            .route("/health", get(crate::api::handlers::health))
            
            // Service endpoints
            .route("/v1/status", get(crate::api::handlers::status))
            .route("/v1/config", get(crate::api::handlers::get_config))
            
            // Server management endpoints
            .route("/v1/server/start", post(crate::api::handlers::start_server))
            .route("/v1/server/stop", post(crate::api::handlers::stop_server))
            .route("/v1/server/sessions", get(crate::api::handlers::list_sessions))
            
            // Add tracing and CORS
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            .with_state(api_state);
        
        // Parse the address
        let addr: SocketAddr = addr.parse()
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
            let origins = self.config.cors_allowed_origins.iter()
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
