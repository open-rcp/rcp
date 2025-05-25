use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::Value;

use crate::{error::Result, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/metrics", get(get_metrics))
        .route("/health", get(health_check))
        .route("/logs", get(get_logs))
}

async fn get_metrics(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "cpu_usage": 0.0,
        "memory_usage": 0.0,
        "disk_usage": 0.0,
        "network_io": {"rx": 0, "tx": 0},
        "message": "RCP daemon not connected - mock response"
    })))
}

async fn health_check(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "message": "RCP daemon not connected - mock response"
    })))
}

async fn get_logs(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "logs": [],
        "message": "RCP daemon not connected - mock response"
    })))
}
