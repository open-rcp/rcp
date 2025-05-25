use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get},
    Router,
};
use tracing::info;
use uuid::Uuid;

use crate::{error::Result, models::Session, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_sessions))
        .route("/:id", get(get_session).delete(close_session))
}

async fn list_sessions(State(state): State<AppState>) -> Result<Json<Vec<Session>>> {
    let sessions = state.rcpdaemon_client.get_sessions().await?;
    info!("Retrieved {} active sessions", sessions.len());
    Ok(Json(sessions))
}

async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<Session>> {
    let session = state.rcpdaemon_client.get_session(session_id).await?;
    info!("Retrieved session {}", session_id);
    Ok(Json(session))
}

async fn close_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    info!("Closing session {}", session_id);
    state.rcpdaemon_client.close_session(session_id).await?;

    Ok(Json(serde_json::json!({
        "message": format!("Session {} closed successfully", session_id),
        "success": true
    })))
}
