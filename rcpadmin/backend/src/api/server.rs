use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use tracing::info;

use crate::{error::Result, models::ServerStatus, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(get_status))
        .route("/restart", post(restart_server))
        .route("/stop", post(stop_server))
}

async fn get_status(State(state): State<AppState>) -> Result<Json<ServerStatus>> {
    let status = state.rcpdaemon_client.get_status().await?;
    info!("Retrieved server status: {:?}", status);
    Ok(Json(status))
}

async fn restart_server(State(_state): State<AppState>) -> Result<Json<serde_json::Value>> {
    // This would normally call a method on the RCP daemon client
    // For now we'll just return a mock response
    info!("Restart server request received");

    Ok(Json(serde_json::json!({
        "message": "Server restart initiated",
        "success": true
    })))
}

async fn stop_server(State(_state): State<AppState>) -> Result<Json<serde_json::Value>> {
    // This would normally call a method on the RCP daemon client
    // For now we'll just return a mock response
    info!("Stop server request received");

    Ok(Json(serde_json::json!({
        "message": "Server stop initiated",
        "success": true
    })))
}
