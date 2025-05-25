use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::Value;

use crate::{error::Result, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(get_status))
        .route("/restart", get(restart_server))
        .route("/config", get(get_config))
}

async fn get_status(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "status": "running",
        "uptime": "0h 0m 0s",
        "version": "unknown",
        "message": "RCP daemon not connected - mock response"
    })))
}

async fn restart_server(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "success": false,
        "message": "RCP daemon not connected - mock response"
    })))
}

async fn get_config(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "config": {},
        "message": "RCP daemon not connected - mock response"
    })))
}
