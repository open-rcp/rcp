use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::Value;

use crate::{error::Result, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_sessions))
        .route("/:id", get(get_session))
        // .route("/:id", delete(close_session))
}

async fn get_sessions(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "sessions": [],
        "message": "RCP daemon not connected - mock response"
    })))
}

async fn get_session(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "session": null,
        "message": "RCP daemon not connected - mock response"
    })))
}
