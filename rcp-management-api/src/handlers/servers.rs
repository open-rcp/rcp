use crate::app::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::{CreateServerRequest, Server, UpdateServerRequest};
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use surrealdb::opt::RecordId;

/// List all servers
pub async fn list_servers(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Server>>> {
    // Updated for SurrealDB 2.3.0
    let servers: Vec<Server> = state.db.select("server").await
        .map_err(|e| ApiError::Database(format!("Failed to fetch servers: {}", e)))?;

    Ok(Json(servers))
}

/// Get a specific server by ID
pub async fn get_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<Server>> {
    let server_id = format!("server:{}", server_id);
    
    // Updated for SurrealDB 2.3.0
    let server: Option<Server> = state.db.select(server_id).await
        .map_err(|e| ApiError::Database(format!("Failed to fetch server: {}", e)))?;

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

    // Insert the server into the database (updated for SurrealDB 2.3.0)
    let created: Option<Server> = state.db.create("server")
        .content(&server)
        .await
        .map_err(|e| ApiError::Database(format!("Failed to create server: {}", e)))?;

    // Return the created server
    match created {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal("Failed to retrieve created server".to_string())),
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
    let existing: Option<Server> = state.db.select(&server_id).await
        .map_err(|e| ApiError::Database(format!("Failed to fetch server: {}", e)))?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Build update content based on provided fields
    let mut update_content = serde_json::json!({});
    
    if let Some(name) = update_req.name {
        update_content["name"] = serde_json::json!(name);
    }
    
    if let Some(host) = update_req.host {
        update_content["host"] = serde_json::json!(host);
    }
    
    if let Some(port) = update_req.port {
        update_content["port"] = serde_json::json!(port);
    }
    
    // Always update the updated_at timestamp
    update_content["updated_at"] = serde_json::json!(time::OffsetDateTime::now_utc());

    // Updated for SurrealDB 2.3.0
    let updated: Option<Server> = state.db.update(&server_id)
        .merge(update_content)
        .await
        .map_err(|e| ApiError::Database(format!("Failed to update server: {}", e)))?;

    match updated {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal("Failed to retrieve updated server".to_string())),
    }
}

/// Delete a server
pub async fn delete_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let server_id = format!("server:{}", server_id);

    // Check if the server exists
    let existing: Option<Server> = state.db.select(&server_id).await
        .map_err(|e| ApiError::Database(format!("Failed to fetch server: {}", e)))?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Delete the server (updated for SurrealDB 2.3.0)
    state.db.delete(&server_id).await
        .map_err(|e| ApiError::Database(format!("Failed to delete server: {}", e)))?;

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
    let existing: Option<Server> = state.db.select(&server_id).await
        .map_err(|e| ApiError::Database(format!("Failed to fetch server: {}", e)))?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Here we would actually start the RCP server process
    // For now, just update the status in the database (updated for SurrealDB 2.3.0)
    let updated: Option<Server> = state.db.update(&server_id)
        .merge(serde_json::json!({ 
            "status": "online",
            "updated_at": time::OffsetDateTime::now_utc()
        }))
        .await
        .map_err(|e| ApiError::Database(format!("Failed to update server status: {}", e)))?;

    match updated {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal("Failed to retrieve updated server".to_string())),
    }
}

/// Stop a server
pub async fn stop_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<Server>> {
    let server_id = format!("server:{}", server_id);

    // Check if the server exists
    let existing: Option<Server> = state.db.select(&server_id).await
        .map_err(|e| ApiError::Database(format!("Failed to fetch server: {}", e)))?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Here we would actually stop the RCP server process
    // For now, just update the status in the database (updated for SurrealDB 2.3.0)
    let updated: Option<Server> = state.db.update(&server_id)
        .merge(serde_json::json!({ 
            "status": "offline",
            "updated_at": time::OffsetDateTime::now_utc()
        }))
        .await
        .map_err(|e| ApiError::Database(format!("Failed to update server status: {}", e)))?;

    match updated {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal("Failed to retrieve updated server".to_string())),
    }
}

/// Restart a server
pub async fn restart_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> ApiResult<Json<Server>> {
    let server_id = format!("server:{}", server_id);

    // Check if the server exists
    let existing: Option<Server> = state.db.select(&server_id).await
        .map_err(|e| ApiError::Database(format!("Failed to fetch server: {}", e)))?;

    if existing.is_none() {
        return Err(ApiError::NotFound("Server not found".to_string()));
    }

    // Here we would actually restart the RCP server process
    // For now, just update the status in the database to simulate a restart (updated for SurrealDB 2.3.0)
    let updated: Option<Server> = state.db.update(&server_id)
        .merge(serde_json::json!({ 
            "status": "restarting",
            "updated_at": time::OffsetDateTime::now_utc()
        }))
        .await
        .map_err(|e| ApiError::Database(format!("Failed to update server status: {}", e)))?;

    // Simulate restart delay and then set to online
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let updated: Option<Server> = state.db.update(&server_id)
        .merge(serde_json::json!({ 
            "status": "online",
            "updated_at": time::OffsetDateTime::now_utc()
        }))
        .await
        .map_err(|e| ApiError::Database(format!("Failed to update server status: {}", e)))?;

    match updated {
        Some(server) => Ok(Json(server)),
        None => Err(ApiError::Internal("Failed to retrieve updated server".to_string())),
    }
}