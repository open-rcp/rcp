use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use tracing::{info, warn};

use crate::{
    error::Result,
    models::{Application, CreateApplication},
    AppState,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_applications).post(create_application))
        .route("/:id", get(get_application).put(update_application).delete(delete_application))
}

async fn list_applications(State(state): State<AppState>) -> Result<Json<Vec<Application>>> {
    let apps = state.rcpdaemon_client.get_applications().await?;
    info!("Retrieved {} applications", apps.len());
    Ok(Json(apps))
}

async fn get_application(
    State(state): State<AppState>,
    Path(app_id): Path<String>,
) -> Result<Json<Application>> {
    let app = state.rcpdaemon_client.get_application(&app_id).await?;
    info!("Retrieved application {}", app.id);
    Ok(Json(app))
}

async fn create_application(
    State(state): State<AppState>,
    Json(app): Json<CreateApplication>,
) -> Result<Json<Application>> {
    info!("Creating new application: {}", app.name);
    let created_app = state.rcpdaemon_client.create_application(app).await?;
    Ok(Json(created_app))
}

async fn update_application(
    State(state): State<AppState>,
    Path(app_id): Path<String>,
    Json(app): Json<CreateApplication>,
) -> Result<Json<Application>> {
    info!("Updating application {}", app_id);
    let updated_app = state.rcpdaemon_client.update_application(&app_id, app).await?;
    Ok(Json(updated_app))
}

async fn delete_application(
    State(state): State<AppState>,
    Path(app_id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    info!("Deleting application {}", app_id);
    state.rcpdaemon_client.delete_application(&app_id).await?;
    
    Ok(Json(serde_json::json!({
        "message": format!("Application {} deleted successfully", app_id),
        "success": true
    })))
}