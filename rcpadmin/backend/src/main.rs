use axum::{middleware, response::Json, routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

mod api;
mod auth;
mod config;
mod db;
mod error;
mod models;
mod services;
mod websocket;

use crate::{
    auth::{protect, require_admin},
    config::Config,
    db::Database,
    services::rcpdaemon::RcpDaemonClient,
};

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: Database,
    pub config: Config,
    pub rcpdaemon_client: RcpDaemonClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("rcpadmin_backend=debug,tower_http=debug")
        .init();

    info!("Starting RCP Admin Backend v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded");

    // Initialize database
    let db = Database::new(&config.database_url).await?;
    db.migrate().await?;
    info!("Database initialized");

    // Initialize RCP daemon client
    let rcpdaemon_client = RcpDaemonClient::new(&config.rcpdaemon_url).await?;
    info!("Connected to RCP daemon at {}", config.rcpdaemon_url);

    // Create application state
    let state = Arc::new(AppStateInner {
        db,
        config: config.clone(),
        rcpdaemon_client,
    });

    // Build application router
    let app = create_router(state);

    // Start server
    let listener = TcpListener::bind(&config.bind_address).await?;
    info!("Server listening on {}", config.bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    // Create protected API routes
    let protected_api = Router::new()
        .nest("/server", api::server::create_routes())
        .nest("/applications", api::applications::create_routes())
        .nest("/sessions", api::sessions::create_routes())
        .nest("/system", api::system::create_routes())
        .layer(middleware::from_fn_with_state(state.clone(), protect));

    // Create admin-only API routes
    let admin_api = Router::new()
        .nest("/users", api::users::create_routes())
        .layer(middleware::from_fn_with_state(state.clone(), require_admin));

    // Build the main router
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Auth routes (no middleware)
        .nest("/api/v1/auth", api::auth::create_routes())
        // Protected API routes
        .nest("/api/v1", protected_api)
        .nest("/api/v1", admin_api)
        // WebSocket routes
        .nest("/ws", websocket::create_routes())
        // Static files (for serving frontend in production)
        .fallback_service(tower_http::services::ServeDir::new("../web/dist"))
        // Global middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    }))
}
