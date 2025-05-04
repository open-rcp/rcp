use crate::app::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::{CreateServerRequest, Server, UpdateServerRequest};
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use surrealdb::sql::Thing;

/// List all servers
pub async fn list_servers(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Server>>> {
    let servers: Vec<Server> = state.db.select("server").await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch servers: {}", e))
    })?;

    Ok(Json(servers))
}

/// Get a specific server by ID
pub async fn get_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<Server>> {
    let server_id = format!("server:{}", server_id);
    let server: Option<Server> = state.db.select(server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch server: {}", e))
    })?;

    match server {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::NotFound("Server not found".to_string())),
    }
}

/// Create a new server
pub async fn create_server(
    State(state): State<Arc<AppState>>,
    Json(create_req): Json<CreateServerRequest>,
) -> ApiResult<Json<Server>> {
    // Create a new server record
    let server = Server {
        id: None,
        name: create_req.name,
        host: create_req.host,
        port: create_req.port,
        status: "offline".to_string(),
        created_at: None, // SurrealDB will set this using the DEFAULT
        updated_at: None, // SurrealDB will set this using the DEFAULT
    };

    // Insert the server into the database
    let created: Vec<Server> = state
        .db
        .create("server")
        .content(&server)
        .await
        .map_err(|e| ApiError::Database(format!("Failed to create server: {}", e)))?;

    // Return the first created server (there should be only one)
    match created.first() {
        Some(server) => Ok(Json(server.clone())),
        None => Err(ApiError::Internal(
            "Failed to retrieve created server".to_string(),
        )),
    }
}

/// Update an existing server
pub async fn update_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
    Json(update_req): Json<UpdateServerRequest>,
) -> ApiResult<Json<Server>> {
    // Check if the server exists
    let server_id = format!("server:{}", server_id);
    let existing: Option<Server> = state.db.select(&server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch server: {}", e))
    })?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Build update query dynamically based on provided fields
    let mut update = state.db.update(server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to prepare server update: {}", e))
    })?;

    if let Some(name) = update_req.name {
        update = update.merge(serde_json::json!({ "name": name })).await.map_err(|e| {
            ApiError::Database(format!("Failed to update server name: {}", e))
        })?;
    }

    if let Some(host) = update_req.host {
        update = update.merge(serde_json::json!({ "host": host })).await.map_err(|e| {
            ApiError::Database(format!("Failed to update server host: {}", e))
        })?;
    }

    if let Some(port) = update_req.port {
        update = update.merge(serde_json::json!({ "port": port })).await.map_err(|e| {
            ApiError::Database(format!("Failed to update server port: {}", e))
        })?;
    }

    // Always update the updated_at timestamp
    update = update
        .merge(serde_json::json!({ "updated_at": "time::now()" }))
        .await
        .map_err(|e| {
            ApiError::Database(format!("Failed to update server timestamp: {}", e))
        })?;

    // Get the updated server
    let updated: Option<Server> = state.db.select(server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch updated server: {}", e))
    })?;

    match updated {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal(
            "Failed to retrieve updated server".to_string(),
        )),
    }
}

/// Delete a server
pub async fn delete_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let server_id = format!("server:{}", server_id);

    // Check if the server exists
    let existing: Option<Server> = state.db.select(&server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch server: {}", e))
    })?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Delete the server
    state.db.delete(server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to delete server: {}", e))
    })?;

    Ok(Json(serde_json::json!({
        "message": "Server deleted successfully"
    })))
}

/// Start a server (control the RCP server instance)
pub async fn start_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<Server>> {
    let server_id = format!("server:{}", server_id);

    // Check if the server exists
    let existing: Option<Server> = state.db.select(&server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch server: {}", e))
    })?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Here we would actually start the RCP server process
    // For now, just update the status in the database
    let updated = state
        .db
        .update(&server_id)
        .merge(serde_json::json!({ 
            "status": "online",
            "updated_at": "time::now()"
        }))
        .await
        .map_err(|e| {
            ApiError::Database(format!("Failed to update server status: {}", e))
        })?;

    // Get the updated server
    let server: Option<Server> = state.db.select(server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch updated server: {}", e))
    })?;

    match server {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal(
            "Failed to retrieve updated server".to_string(),
        )),
    }
}

/// Stop a server
pub async fn stop_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<Server>> {
    let server_id = format!("server:{}", server_id);

    // Check if the server exists
    let existing: Option<Server> = state.db.select(&server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch server: {}", e))
    })?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Here we would actually stop the RCP server process
    // For now, just update the status in the database
    let updated = state
        .db
        .update(&server_id)
        .merge(serde_json::json!({ 
            "status": "offline",
            "updated_at": "time::now()"
        }))
        .await
        .map_err(|e| {
            ApiError::Database(format!("Failed to update server status: {}", e))
        })?;

    // Get the updated server
    let server: Option<Server> = state.db.select(server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch updated server: {}", e))
    })?;

    match server {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal(
            "Failed to retrieve updated server".to_string(),
        )),
    }
}

/// Restart a server
pub async fn restart_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<Server>> {
    let server_id = format!("server:{}", server_id);

    // Check if the server exists
    let existing: Option<Server> = state.db.select(&server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch server: {}", e))
    })?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Here we would actually restart the RCP server process
    // For now, just update the status in the database to simulate a restart
    let updated = state
        .db
        .update(&server_id)
        .merge(serde_json::json!({ 
            "status": "restarting",
            "updated_at": "time::now()"
        }))
        .await
        .map_err(|e| {
            ApiError::Database(format!("Failed to update server status: {}", e))
        })?;

    // Simulate restart delay and then set to online
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let updated = state
        .db
        .update(&server_id)
        .merge(serde_json::json!({ 
            "status": "online",
            "updated_at": "time::now()"
        }))
        .await
        .map_err(|e| {
            ApiError::Database(format!("Failed to update server status: {}", e))
        })?;

    // Get the updated server
    let server: Option<Server> = state.db.select(server_id).await.map_err(|e| {
        ApiError::Database(format!("Failed to fetch updated server: {}", e))
    })?;

    match server {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal(
            "Failed to retrieve updated server".to_string(),
        )),
    }
}