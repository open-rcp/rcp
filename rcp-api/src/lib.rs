use std::sync::Arc;
use tokio::sync::Mutex;

pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod routes;
pub mod service;

/// Re-exports for commonly used types
pub use error::ApiError;
pub use config::ApiConfig;

/// Application state shared across API handlers
#[derive(Clone)]
pub struct AppState {
    /// Connection to the RCP service
    pub service_client: Arc<Mutex<service::ServiceClient>>,
    
    /// Database pool for persistence
    pub db_pool: db::DbPool,
    
    /// API configuration
    pub config: Arc<ApiConfig>,
}

/// Initialize the API server with configuration
pub async fn init(config: ApiConfig) -> Result<(), ApiError> {
    tracing::info!("Initializing RCP API server");
    
    // Initialize database
    let db_pool = db::init_db(&config.database_url).await?;
    
    // Connect to RCP service
    let service_client = service::ServiceClient::connect(&config.service_connection_string, None)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to connect to RCP service: {}", e)))?;
    
    // Store config values before moving into Arc
    let bind_address = config.bind_address.clone();
    let port = config.port;
    
    // Create application state
    let app_state = AppState {
        service_client: Arc::new(Mutex::new(service_client)),
        db_pool,
        config: Arc::new(config),
    };
    
    // Initialize API server
    let router = routes::create_router(app_state);
    
    // Start the server
    let addr = format!("{}:{}", bind_address, port);
    tracing::info!("Starting API server at {}", addr);
    
    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .map_err(|e| ApiError::ServerError(format!("Server error: {}", e)))?;
    
    Ok(())
}

/// Health check function to determine if the API is healthy
pub async fn is_healthy(state: &AppState) -> bool {
    // Check database connection
    if let Err(e) = db::ping(&state.db_pool).await {
        tracing::error!("Database health check failed: {}", e);
        return false;
    }
    
    // Check service connection
    let service_client = state.service_client.lock().await;
    if let Err(e) = service_client.ping().await {
        tracing::error!("Service health check failed: {}", e);
        return false;
    }
    
    true
}