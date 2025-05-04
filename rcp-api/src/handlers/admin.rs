use axum::{
    extract::{State, Query, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{AppState, ApiError, db};
use crate::handlers::auth::AuthUser;

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
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Serialize)]
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
    // Get configuration from database
    let config = sqlx::query!(
        r#"
        SELECT data FROM configurations 
        WHERE name = 'system' AND config_type = 'system'
        LIMIT 1
        "#
    )
    .fetch_optional(&state.db_pool)
    .await?;
    
    let system_config = if let Some(config) = config {
        serde_json::from_str::<SystemConfig>(&config.data)
            .map_err(|e| ApiError::ServerError(format!("Failed to parse system config: {}", e)))?
    } else {
        // Return default config if none exists
        let default_config = SystemConfig {
            service_address: "127.0.0.1".to_string(),
            service_port: 9000,
            tls_enabled: false,
            certificate_path: None,
            key_path: None,
            max_servers: 10,
            max_connections_per_server: 100,
            log_level: "info".to_string(),
            log_retention_days: 30,
        };
        
        // Save default config to database
        let config_data = serde_json::to_string(&default_config)
            .map_err(|e| ApiError::ServerError(format!("Failed to serialize config: {}", e)))?;
        
        let now = chrono::Utc::now().to_rfc3339();
        let id = uuid::Uuid::new_v4().to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO configurations (id, name, config_type, data, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            id, "system", "system", config_data, now, now
        )
        .execute(&state.db_pool)
        .await?;
        
        default_config
    };
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_system_config",
        Some("system"),
        None,
        None
    ).await?;
    
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
        return Err(ApiError::ValidationError("Service port must be greater than 0".to_string()));
    }
    
    if system_config.tls_enabled && (system_config.certificate_path.is_none() || system_config.key_path.is_none()) {
        return Err(ApiError::ValidationError("Certificate and key paths are required when TLS is enabled".to_string()));
    }
    
    if !["debug", "info", "warn", "error"].contains(&system_config.log_level.as_str()) {
        return Err(ApiError::ValidationError("Log level must be one of: debug, info, warn, error".to_string()));
    }
    
    // Serialize configuration
    let config_data = serde_json::to_string(&system_config)
        .map_err(|e| ApiError::ServerError(format!("Failed to serialize config: {}", e)))?;
    
    // Update configuration in database
    let now = chrono::Utc::now().to_rfc3339();
    
    let result = sqlx::query!(
        r#"
        UPDATE configurations
        SET data = ?, updated_at = ?
        WHERE name = 'system' AND config_type = 'system'
        "#,
        config_data, now
    )
    .execute(&state.db_pool)
    .await?;
    
    if result.rows_affected() == 0 {
        // Configuration doesn't exist, insert it
        let id = uuid::Uuid::new_v4().to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO configurations (id, name, config_type, data, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            id, "system", "system", config_data, now, now
        )
        .execute(&state.db_pool)
        .await?;
    }
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "update_system_config",
        Some("system"),
        None,
        Some(&format!("log_level={}, tls_enabled={}", 
            system_config.log_level, system_config.tls_enabled))
    ).await?;
    
    // In a real implementation, we might notify the service of the config change
    // or restart services as needed
    
    Ok(StatusCode::NO_CONTENT)
}

/// Get audit logs
pub async fn get_audit_logs(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLogEntry>>, ApiError> {
    // Build query conditions
    let mut conditions = Vec::new();
    let mut values = Vec::new();
    
    if let Some(user_id) = &params.user_id {
        conditions.push("user_id = ?");
        values.push(user_id.to_string());
    }
    
    if let Some(action) = &params.action {
        conditions.push("action = ?");
        values.push(action.to_string());
    }
    
    if let Some(entity_type) = &params.entity_type {
        conditions.push("entity_type = ?");
        values.push(entity_type.to_string());
    }
    
    if let Some(entity_id) = &params.entity_id {
        conditions.push("entity_id = ?");
        values.push(entity_id.to_string());
    }
    
    if let Some(from) = &params.from {
        conditions.push("timestamp >= ?");
        values.push(from.to_string());
    }
    
    if let Some(to) = &params.to {
        conditions.push("timestamp <= ?");
        values.push(to.to_string());
    }
    
    // Build the query
    let mut query = "SELECT id, user_id, action, entity_type, entity_id, details, timestamp FROM audit_log".to_string();
    
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    
    query.push_str(" ORDER BY timestamp DESC");
    
    // Apply limit and offset
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    
    // Execute the query (manually for dynamic conditions)
    let mut db_query = sqlx::query(&query);
    for value in values {
        db_query = db_query.bind(value);
    }
    
    let rows = db_query.fetch_all(&state.db_pool).await?;
    
    // Create a mapping of user IDs to usernames
    let mut user_map = std::collections::HashMap::new();
    
    // Extract user IDs from logs
    let mut user_ids = Vec::new();
    for row in &rows {
        if let Ok(Some(user_id)) = row.try_get::<Option<String>, _>("user_id") {
            if !user_ids.contains(&user_id) {
                user_ids.push(user_id);
            }
        }
    }
    
    // Get usernames for user IDs
    for user_id in user_ids {
        let user = sqlx::query_as!(
            db::User,
            r#"SELECT id, username, password_hash, role, active, created_at, last_login FROM users WHERE id = ?"#,
            user_id
        )
        .fetch_optional(&state.db_pool)
        .await?;
        
        if let Some(user) = user {
            user_map.insert(user.id, user.username);
        }
    }
    
    // Map rows to audit log entries
    let mut log_entries = Vec::new();
    
    for row in rows {
        let id: String = row.try_get("id")?;
        let user_id: Option<String> = row.try_get("user_id")?;
        let action: String = row.try_get("action")?;
        let entity_type: Option<String> = row.try_get("entity_type")?;
        let entity_id: Option<String> = row.try_get("entity_id")?;
        let details: Option<String> = row.try_get("details")?;
        let timestamp: String = row.try_get("timestamp")?;
        
        // Get username if user_id exists
        let username = user_id.as_ref().and_then(|id| user_map.get(id).cloned());
        
        log_entries.push(AuditLogEntry {
            id,
            timestamp,
            user_id,
            username,
            action,
            entity_type,
            entity_id,
            details,
        });
    }
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_audit_logs",
        None,
        None,
        Some(&format!("limit={}, offset={}", limit, offset))
    ).await?;
    
    Ok(Json(log_entries))
}