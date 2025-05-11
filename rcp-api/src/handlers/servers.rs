use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, ApiError, db};
use crate::handlers::auth::AuthUser;

/// Server information response
#[derive(Debug, Serialize)]
pub struct ServerResponse {
    id: String,
    name: String,
    status: String,
    port: u16,
    connections: u32,
    started_at: Option<String>,
}

/// Server creation request
#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    name: String,
    port: u16,
    max_connections: usize,
    tls_enabled: Option<bool>,
    tls_cert_path: Option<String>,
    tls_key_path: Option<String>,
}

/// List all servers
pub async fn list_servers(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<ServerResponse>>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;
    
    // Get server list from service
    let servers = service_client.list_servers().await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list servers: {}", e)))?;
    
    // Convert to response format
    let responses = servers.into_iter()
        .map(|server| ServerResponse {
            id: server.id,
            name: server.name,
            status: server.status,
            port: server.port.unwrap_or(0),
            connections: 0, // Default value since the field doesn't exist
            started_at: Some(server.created_at.clone()), // Using created_at as a substitute
        })
        .collect();
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "list_servers",
        None,
        None,
        None
    ).await?;
    
    Ok(Json(responses))
}

/// Get a specific server by name
pub async fn get_server(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(name): Path<String>,
) -> Result<Json<ServerResponse>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;
    
    // Get server list from service
    let servers = service_client.list_servers().await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list servers: {}", e)))?;
    
    // Find the specific server
    let server = servers.into_iter()
        .find(|s| s.name == name)
        .ok_or_else(|| ApiError::NotFoundError(format!("Server '{}' not found", name)))?;
    
    // Convert to response format
    let response = ServerResponse {
        id: server.id,
        name: server.name,
        status: server.status,
        port: server.port.unwrap_or(0),
        connections: 0, // Default value since the field doesn't exist
        started_at: Some(server.created_at.clone()), // Using created_at as a substitute
    };
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_server",
        Some("server"),
        Some(&response.id),
        None
    ).await?;
    
    Ok(Json(response))
}

/// Create a new server
pub async fn create_server(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateServerRequest>,
) -> Result<(StatusCode, Json<ServerResponse>), ApiError> {
    // First check if a server with this name already exists in the database
    let now = chrono::Utc::now().to_rfc3339();
    let server_id = Uuid::new_v4().to_string();
    
    // Create server configuration in the database
    sqlx::query(
        r#"
        INSERT INTO server_configs (id, name, port, max_connections, tls_enabled, cert_path, key_path, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&server_id)
    .bind(&payload.name)
    .bind(payload.port as i32)
    .bind(payload.max_connections as i32)
    .bind(payload.tls_enabled.unwrap_or(false) as i32)
    .bind(payload.tls_cert_path)
    .bind(payload.tls_key_path)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("UNIQUE constraint") {
            ApiError::ConflictError(format!("Server '{}' already exists", payload.name))
        } else {
            ApiError::DatabaseError(format!("Failed to create server: {}", e))
        }
    })?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "create_server",
        Some("server"),
        Some(&server_id),
        Some(&format!("name={}, port={}", payload.name, payload.port))
    ).await?;
    
    // Return the created server info
    let response = ServerResponse {
        id: server_id,
        name: payload.name,
        status: "stopped".to_string(),
        port: payload.port,
        connections: 0,
        started_at: None,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

/// Start a server
pub async fn start_server(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(name): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;
    
    // Start the server
    service_client.start_server(&name).await
        .map_err(|e| ApiError::ServiceError(format!("Failed to start server: {}", e)))?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "start_server",
        Some("server"),
        None,
        Some(&format!("name={}", name))
    ).await?;
    
    Ok(StatusCode::OK)
}

/// Stop a server
pub async fn stop_server(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(name): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;
    
    // Stop the server
    service_client.stop_server(&name).await
        .map_err(|e| ApiError::ServiceError(format!("Failed to stop server: {}", e)))?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "stop_server",
        Some("server"),
        None,
        Some(&format!("name={}", name))
    ).await?;
    
    Ok(StatusCode::OK)
}

/// Restart a server
pub async fn restart_server(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(name): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;
    
    // Restart the server
    service_client.stop_server(&name).await
        .map_err(|e| ApiError::ServiceError(format!("Failed to stop server: {}", e)))?;
        
    service_client.start_server(&name).await
        .map_err(|e| ApiError::ServiceError(format!("Failed to start server: {}", e)))?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "restart_server",
        Some("server"),
        None,
        Some(&format!("name={}", name))
    ).await?;
    
    Ok(StatusCode::OK)
}

/// Delete a server configuration
pub async fn delete_server(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(name): Path<String>,
) -> Result<StatusCode, ApiError> {
    // First try to stop the server if it's running
    let service_client = state.service_client.lock().await;
    
    // Get server list to check if the server exists and is running
    let servers = service_client.list_servers().await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list servers: {}", e)))?;
    
    let server = servers.iter().find(|s| s.name == name);
    
    if let Some(server) = server {
        if server.status.to_lowercase() == "running" {
            // Stop the server first
            service_client.stop_server(&name).await
                .map_err(|e| ApiError::ServiceError(format!("Failed to stop server: {}", e)))?;
        }
    }
    
    // Delete the server configuration from the database
    let result = sqlx::query("DELETE FROM server_configs WHERE name = ?")
        .bind(&name)
        .execute(&state.db_pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFoundError(format!("Server '{}' not found", name)));
    }
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "delete_server",
        Some("server"),
        None,
        Some(&format!("name={}", name))
    ).await?;
    
    Ok(StatusCode::NO_CONTENT)
}