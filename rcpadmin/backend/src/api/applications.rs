use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::Value;

use crate::{error::Result, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_applications))
        .route("/:id", get(get_application))
        // .route("/", post(create_application))
        // .route("/:id", put(update_application))
        // .route("/:id", delete(delete_application))
}

async fn get_applications(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "applications": [],
        "message": "RCP daemon not connected - mock response"
    })))
}

async fn get_application(State(_state): State<AppState>) -> Result<Json<Value>> {
    // Mock response for now - replace with actual rcpdaemon call later
    Ok(Json(serde_json::json!({
        "application": null,
        "message": "RCP daemon not connected - mock response"
    })))
}
