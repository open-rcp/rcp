use crate::db::init_database;
use crate::routes;
use axum::Router;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// Creates and configures the Axum application with all routes and middleware
pub async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    // Initialize the SurrealDB connection
    let db = init_database().await?;
    
    // Create a shared application state
    let app_state = Arc::new(AppState {
        db,
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
    pub db: Surreal<Db>,
}