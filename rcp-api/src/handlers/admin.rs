use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::handlers::auth::AuthUser;
use crate::{db, ApiError, AppState};

/// System configuration response and update request
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    service_address: String,
    service_port: u16,
    tls_enabled: bool,
    certificate_path: Option<String>,
    key_path: Option<String>,
    max_servers: u32,
    max_connections_per_server: u32,
    log_level: String,
    log_retention_days: u32,
}

/// Audit log query parameters
#[derive(Debug, Deserialize, Serialize)]
pub struct AuditLogQuery {
    user_id: Option<String>,
    action: Option<String>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    from: Option<String>,
    to: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

/// Audit log entry
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogEntry {
    id: String,
    timestamp: String,
    user_id: Option<String>,
    username: Option<String>,
    action: String,
    entity_type: Option<String>,
    entity_id: Option<String>,
    details: Option<String>,
}

/// Get system configuration
pub async fn get_system_config(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<SystemConfig>, ApiError> {
    // In the refactored architecture, system configuration is delegated to the RCP service
    let service_client = state.service_client.lock().await;

    // Get system config from service (not using response directly, using defaults)
    service_client.get_status().await?;

    // Extract config or provide default
    let system_config = SystemConfig {
        service_address: state.config.bind_address.clone(),
        service_port: state.config.port,
        tls_enabled: false, // Default value
        certificate_path: None,
        key_path: None,
        max_servers: 100,               // Default value
        max_connections_per_server: 50, // Default value
        log_level: "info".to_string(),  // Default value
        log_retention_days: 30,         // Default value
    };

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_system_config",
        Some("system"),
        None,
        None,
    )
    .await?;

    Ok(Json(system_config))
}

/// Update system configuration
pub async fn update_system_config(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(system_config): Json<SystemConfig>,
) -> Result<StatusCode, ApiError> {
    // Validate configuration
    if system_config.service_port == 0 {
        return Err(ApiError::ValidationError(
            "Service port must be greater than 0".to_string(),
        ));
    }

    if system_config.tls_enabled
        && (system_config.certificate_path.is_none() || system_config.key_path.is_none())
    {
        return Err(ApiError::ValidationError(
            "Certificate and key paths are required when TLS is enabled".to_string(),
        ));
    }

    if !["debug", "info", "warn", "error"].contains(&system_config.log_level.as_str()) {
        return Err(ApiError::ValidationError(
            "Log level must be one of: debug, info, warn, error".to_string(),
        ));
    }

    // In refactored architecture, we delegate system config to RCP service
    let service_client = state.service_client.lock().await;

    // Create command to update configuration
    let config_json = serde_json::to_value(&system_config)
        .map_err(|e| ApiError::ServerError(format!("Failed to serialize config: {}", e)))?;

    // Send update configuration command to service
    let command = "update-config";
    let args = serde_json::to_vec(&config_json)
        .map_err(|e| ApiError::ServerError(format!("Failed to serialize command args: {}", e)))?;

    service_client.send_command(command, &args).await?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "update_system_config",
        Some("system"),
        None,
        Some(&format!(
            "log_level={}, tls_enabled={}",
            system_config.log_level, system_config.tls_enabled
        )),
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get audit logs
pub async fn get_audit_logs(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLogEntry>>, ApiError> {
    // In refactored architecture, audit logs are sent to the RCP service
    let service_client = state.service_client.lock().await;

    // Create command to fetch audit logs
    let query_json = serde_json::to_value(&params)
        .map_err(|e| ApiError::ServerError(format!("Failed to serialize query: {}", e)))?;

    // Send audit log query command to service
    let command = "get-audit-logs";
    let args = serde_json::to_vec(&query_json)
        .map_err(|e| ApiError::ServerError(format!("Failed to serialize command args: {}", e)))?;

    let response = service_client.send_command(command, &args).await?;

    // Parse response to audit logs
    let audit_logs: Vec<AuditLogEntry> = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServerError(format!("Failed to parse audit logs: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_audit_logs",
        None,
        None,
        Some(&format!(
            "limit={}, offset={}",
            params.limit.unwrap_or(100),
            params.offset.unwrap_or(0)
        )),
    )
    .await?;

    Ok(Json(audit_logs))
}
