// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcp-api/src/handlers/apps.rs
//
// Application Management
//
// This module provides an interface layer for application management in the RCP system.
// The architecture follows these principles:
//  1. The actual application installation/uninstallation is done by server administrators outside of RCP
//  2. RCP Server configures and manages these pre-installed applications
//  3. App management is purely CRUD operations for app configurations, not actual installation/uninstallation
//
// Key operations:
// - List/Get/Create/Update/Delete - Standard CRUD operations for application configurations
// - Enable/Disable - Toggle the availability of applications within RCP without installing/uninstalling
// - Launch/Terminate - Control application instances for users
//
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::handlers::auth::AuthUser;
use crate::{db, ApiError, AppState};

/// Application information response
#[derive(Debug, Serialize, Deserialize)]
pub struct AppResponse {
    id: String,
    name: String,
    path: String,
    args: Option<Vec<String>>,
    enabled: bool,
    icon_path: Option<String>,
    description: Option<String>,
    categories: Option<Vec<String>>,
    allowed_users: Option<Vec<String>>,
    allowed_roles: Option<Vec<String>>,
    resource_limits: Option<ResourceLimits>,
    created_at: String,
    updated_at: String,
}

/// Resource limits for applications
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLimits {
    max_cpu_percent: Option<u8>,
    max_memory_mb: Option<u32>,
    max_disk_mb: Option<u64>,
    max_bandwidth_mbps: Option<u32>,
}

/// Application creation request
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAppRequest {
    name: String,
    path: String,
    args: Option<Vec<String>>,
    enabled: Option<bool>,
    icon_path: Option<String>,
    description: Option<String>,
    categories: Option<Vec<String>>,
    allowed_users: Option<Vec<String>>,
    allowed_roles: Option<Vec<String>>,
    resource_limits: Option<ResourceLimits>,
}

/// Application update request
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAppRequest {
    name: Option<String>,
    path: Option<String>,
    args: Option<Vec<String>>,
    enabled: Option<bool>,
    icon_path: Option<String>,
    description: Option<String>,
    categories: Option<Vec<String>>,
    allowed_users: Option<Vec<String>>,
    allowed_roles: Option<Vec<String>>,
    resource_limits: Option<ResourceLimits>,
}

/// List all applications
pub async fn list_apps(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<AppResponse>>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to list apps
    let command = "list-apps";
    let args = serde_json::to_vec(&serde_json::json!({}))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list applications: {}", e)))?;

    // Parse apps from response
    let apps: Vec<AppResponse> = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse application list: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "list_apps",
        None,
        None,
        None,
    )
    .await?;

    Ok(Json(apps))
}

/// Get an application by ID
pub async fn get_app(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<AppResponse>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to get the app
    let command = "get-app";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to get application: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response).map_err(|e| {
        ApiError::ServiceError(format!("Failed to parse application response: {}", e))
    })?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        return Err(ApiError::NotFoundError(error.to_string()));
    }

    // Parse app from response
    let app: AppResponse = serde_json::from_value(response_value)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse application data: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_app",
        Some("application"),
        Some(&id),
        None,
    )
    .await?;

    Ok(Json(app))
}

/// Create a new application
pub async fn create_app(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateAppRequest>,
) -> Result<(StatusCode, Json<AppResponse>), ApiError> {
    // Validate input
    if payload.path.is_empty() {
        return Err(ApiError::ValidationError(
            "Application path is required".to_string(),
        ));
    }

    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to create the app
    let command = "create-app";
    let args = serde_json::to_vec(&payload)?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to create application: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("already exists") {
            return Err(ApiError::ConflictError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Parse app from response
    let app: AppResponse = serde_json::from_value(response_value)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse application data: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "create_app",
        Some("application"),
        Some(&app.id),
        Some(&format!("name={}, path={}", payload.name, payload.path)),
    )
    .await?;

    Ok((StatusCode::CREATED, Json(app)))
}

/// Update an application
pub async fn update_app(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAppRequest>,
) -> Result<Json<AppResponse>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to update the app
    let command = "update-app";
    let mut update_payload = serde_json::to_value(payload)?;

    // Add the ID to the payload
    if let serde_json::Value::Object(ref mut map) = update_payload {
        map.insert("id".to_string(), serde_json::Value::String(id.clone()));
    }

    let args = serde_json::to_vec(&update_payload)?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to update application: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Parse app from response
    let app: AppResponse = serde_json::from_value(response_value)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse application data: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "update_app",
        Some("application"),
        Some(&id),
        Some("Application updated"),
    )
    .await?;

    Ok(Json(app))
}

/// Delete an application
pub async fn delete_app(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to delete the app
    let command = "delete-app";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to delete application: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "delete_app",
        Some("application"),
        Some(&id),
        None,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Enable an application configuration in RCP
///
/// This doesn't install the application (which should be pre-installed by administrators),
/// but rather enables its configuration within the RCP system, making it available to users.
pub async fn enable_app(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to enable the app configuration
    let command = "update-app";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id,
        "enabled": true
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to enable application: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "enable_app",
        Some("application"),
        Some(&id),
        None,
    )
    .await?;

    Ok(StatusCode::OK)
}

/// Disable an application
pub async fn disable_app(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to disable the app configuration
    let command = "update-app";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id,
        "enabled": false
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to disable application: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "disable_app",
        Some("application"),
        Some(&id),
        None,
    )
    .await?;

    Ok(StatusCode::OK)
}

/// Launch an application for a specific user
pub async fn launch_app(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((id, user_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to launch the app
    let command = "launch-app";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id,
        "user_id": user_id,
        "launching_user_id": auth_user.id
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to launch application: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "launch_app",
        Some("application"),
        Some(&id),
        Some(&format!("for_user={}", user_id)),
    )
    .await?;

    Ok(Json(response_value))
}

/// Terminate a running application instance
pub async fn terminate_app_instance(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(instance_id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to terminate the app instance
    let command = "terminate-app-instance";
    let args = serde_json::to_vec(&serde_json::json!({
        "instance_id": instance_id
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| {
            ApiError::ServiceError(format!("Failed to terminate application instance: {}", e))
        })?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "terminate_app_instance",
        Some("app_instance"),
        Some(&instance_id),
        None,
    )
    .await?;

    Ok(StatusCode::OK)
}

/// List running application instances
pub async fn list_app_instances(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to list app instances
    let command = "list-app-instances";
    let args = serde_json::to_vec(&serde_json::json!({}))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| {
            ApiError::ServiceError(format!("Failed to list application instances: {}", e))
        })?;

    // Parse response
    let instances: serde_json::Value = serde_json::from_slice(&response).map_err(|e| {
        ApiError::ServiceError(format!("Failed to parse application instances: {}", e))
    })?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "list_app_instances",
        None,
        None,
        None,
    )
    .await?;

    Ok(Json(instances))
}
