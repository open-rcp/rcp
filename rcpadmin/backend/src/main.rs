use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};

mod api;
mod auth;
mod config;
mod db;
mod error;
mod models;
mod services;
mod websocket;

use crate::{
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
    Router::new()
        // API routes
        .nest("/api/v1", api::create_routes())
        // WebSocket routes
        .nest("/ws", websocket::create_routes())
        // Health check
        .route("/health", get(health_check))
        // Static files (for serving frontend in production)
        .fallback_service(tower_http::services::ServeDir::new("../web/dist"))
        // Middleware
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