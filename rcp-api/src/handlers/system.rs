use axum::{
    extract::{State, Query},
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{AppState, ApiError, db};
use crate::handlers::auth::AuthUser;

/// System status response
#[derive(Debug, Serialize)]
pub struct SystemStatusResponse {
    version: String,
    uptime: u64,
    memory_usage: MemoryUsage,
    active_servers: u32,
    active_sessions: u32,
    api_status: ApiStatus,
}

/// Memory usage information
#[derive(Debug, Serialize)]
pub struct MemoryUsage {
    total_mb: u64,
    used_mb: u64,
    percentage: f32,
}

/// API status information
#[derive(Debug, Serialize)]
pub struct ApiStatus {
    database_connection: bool,
    service_connection: bool,
    active_users: u32,
    api_uptime: u64,
}

/// Log query parameters
#[derive(Debug, Deserialize)]
pub struct LogQuery {
    service: Option<String>,
    level: Option<String>,
    from: Option<String>,
    to: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

/// Log entry response
#[derive(Debug, Serialize)]
pub struct LogEntry {
    timestamp: String,
    level: String,
    service: String,
    message: String,
    details: Option<String>,
}

/// Get system status information
pub async fn system_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<SystemStatusResponse>, ApiError> {
    // Get service client to call the RCP service
    let mut service_client = state.service_client.lock().await;
    
    // Get service status
    let service_status = service_client.get_status().await
        .map_err(|e| ApiError::ServiceError(format!("Failed to get service status: {}", e)))?;
    
    // Get active user count
    let active_user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE active = 1")
        .fetch_one(&state.db_pool)
        .await?;
    
    // Prepare response
    let response = SystemStatusResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: service_status.uptime,
        memory_usage: MemoryUsage {
            // This is placeholder data - in a real implementation, get actual memory usage
            total_mb: 1024,
            used_mb: 256,
            percentage: 25.0,
        },
        active_servers: service_status.active_servers.len() as u32,
        active_sessions: service_status.active_connections,
        api_status: ApiStatus {
            database_connection: true,
            service_connection: true,
            active_users: active_user_count as u32,
            // This is a placeholder - in a real implementation, track actual API uptime
            api_uptime: 3600,
        },
    };
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_system_status",
        None,
        None,
        None
    ).await?;
    
    Ok(Json(response))
}

/// Get system logs
pub async fn get_logs(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<LogQuery>,
) -> Result<Json<Vec<LogEntry>>, ApiError> {
    // Get service client to call the RCP service
    let mut service_client = state.service_client.lock().await;
    
    // Prepare log query command
    #[derive(Serialize)]
    struct LogQueryCommand {
        service: Option<String>,
        level: Option<String>,
        from: Option<String>,
        to: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    }
    
    let query_command = LogQueryCommand {
        service: params.service,
        level: params.level,
        from: params.from,
        to: params.to,
        limit: params.limit.or(Some(100)),
        offset: params.offset.or(Some(0)),
    };
    
    // Send log query to service
    let command = "get-logs";
    let args = serde_json::to_vec(&query_command)?;
    let response = service_client.send_command(command, &args).await
        .map_err(|e| ApiError::ServiceError(format!("Failed to get logs: {}", e)))?;
    
    // Parse response
    let log_entries: Vec<LogEntry> = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse log entries: {}", e)))?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_logs",
        None,
        None,
        Some(&format!("service={:?}, level={:?}, limit={:?}",
            params.service, params.level, params.limit))
    ).await?;
    
    Ok(Json(log_entries))
}