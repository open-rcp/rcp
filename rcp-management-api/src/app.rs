use crate::db::{init_database, init_surrealdb};
use crate::routes;
use axum::Router;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use sqlx::PgPool;
use tokio::sync::Mutex;

/// Creates and configures the Axum application with all routes and middleware
pub async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    // Initialize the SurrealDB connection
    let db = init_surrealdb().await?;
    
    // Initialize database with default URL for Postgres
    let db_pool = init_database("sqlite://rcp_management.db").await?;
    
    // Create a shared application state
    let app_state = Arc::new(AppState {
        db,
        db_pool: Some(db_pool),
        server_handle: None,
        jwt_secret: "default_secret_change_me".to_string(),
    });

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router with all routes
    let app = Router::new()
        .merge(routes::api_routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(app_state);

    Ok(app)
}

/// Shared application state that can be accessed by all routes
pub struct AppState {
    // We're supporting two different database options depending on configuration
    pub db: Surreal<Db>,
    pub db_pool: Option<PgPool>,
    
    // Reference to the RCP server for control operations
    pub server_handle: Option<Arc<Mutex<rcp_server::server::Server>>>,
    
    // Secret for JWT token generation and validation
    pub jwt_secret: String,
}