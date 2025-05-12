use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::handlers::auth::AuthUser;
use crate::{db, ApiError, AppState};

/// User information response
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    id: String,
    username: String,
    role: String,
    active: bool,
    created_at: String,
    last_login: Option<String>,
}

/// User creation request
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
    role: String,
}

/// User update request
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    role: Option<String>,
    active: Option<bool>,
}

/// Password change request
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    current_password: Option<String>,
    new_password: String,
}

/// List all users
pub async fn list_users(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to list users
    let command = "list-users";
    let args = serde_json::to_vec(&serde_json::json!({}))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list users: {}", e)))?;

    // Parse users from response
    let users: Vec<UserResponse> = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse user list: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "list_users",
        None,
        None,
        None,
    )
    .await?;

    Ok(Json(users))
}

/// Get a user by ID
pub async fn get_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to get the user
    let command = "get-user";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to get user: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse user response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        return Err(ApiError::NotFoundError(error.to_string()));
    }

    // Parse user from response
    let user: UserResponse = serde_json::from_value(response_value)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse user data: {}", e)))?;

    // Log the action (using rcp-server audit system)
    let audit_command = "add-audit-log";
    let audit_args = serde_json::to_vec(&serde_json::json!({
        "user_id": auth_user.id,
        "action": "get_user",
        "entity_type": "user",
        "entity_id": id,
        "details": null
    }))?;

    let _ = service_client
        .send_command(audit_command, &audit_args)
        .await;

    Ok(Json(user))
}

/// Get the current user's profile
pub async fn get_profile(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, ApiError> {
    // Reuse get_user to fetch the current user's profile
    get_user(State(state), auth_user.clone(), Path(auth_user.id)).await
}

/// Create a new user
pub async fn create_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to create the user
    let command = "create-user";
    let args = serde_json::to_vec(&serde_json::json!({
        "username": payload.username,
        "password": payload.password,
        "role": payload.role
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to create user: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("already taken") {
            return Err(ApiError::ConflictError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Parse user from response
    let user: UserResponse = serde_json::from_value(response_value)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse user data: {}", e)))?;

    // Log the action (using rcp-server audit system)
    let audit_command = "add-audit-log";
    let audit_args = serde_json::to_vec(&serde_json::json!({
        "user_id": auth_user.id,
        "action": "create_user",
        "entity_type": "user",
        "entity_id": user.id,
        "details": format!("username={}, role={}", payload.username, payload.role)
    }))?;

    let _ = service_client
        .send_command(audit_command, &audit_args)
        .await;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Update a user
pub async fn update_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to update the user
    let command = "update-user";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id,
        "role": payload.role,
        "active": payload.active,
        "requesting_user_id": auth_user.id
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to update user: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Parse user from response
    let user: UserResponse = serde_json::from_value(response_value)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse user data: {}", e)))?;

    // Log the action (using rcp-server audit system)
    let audit_command = "add-audit-log";
    let audit_args = serde_json::to_vec(&serde_json::json!({
        "user_id": auth_user.id,
        "action": "update_user",
        "entity_type": "user",
        "entity_id": id,
        "details": format!("role={:?}, active={:?}", payload.role, payload.active)
    }))?;

    let _ = service_client
        .send_command(audit_command, &audit_args)
        .await;

    Ok(Json(user))
}

/// Delete a user
pub async fn delete_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Prevent users from deleting themselves
    if auth_user.id == id {
        return Err(ApiError::ValidationError(
            "Cannot delete your own account".to_string(),
        ));
    }

    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to delete the user
    let command = "delete-user";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id,
        "requesting_user_id": auth_user.id
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to delete user: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Log the action (using rcp-server audit system)
    let audit_command = "add-audit-log";
    let audit_args = serde_json::to_vec(&serde_json::json!({
        "user_id": auth_user.id,
        "action": "delete_user",
        "entity_type": "user",
        "entity_id": id,
        "details": null
    }))?;

    let _ = service_client
        .send_command(audit_command, &audit_args)
        .await;

    Ok(StatusCode::NO_CONTENT)
}

/// Change user password
pub async fn change_password(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Call the RCP service to change the password
    let command = "change-user-password";
    let args = serde_json::to_vec(&serde_json::json!({
        "id": id,
        "current_password": payload.current_password,
        "new_password": payload.new_password,
        "requesting_user_id": auth_user.id,
        "requesting_user_role": auth_user.role
    }))?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to change password: {}", e)))?;

    // Check for error response
    let response_value: serde_json::Value = serde_json::from_slice(&response)
        .map_err(|e| ApiError::ServiceError(format!("Failed to parse response: {}", e)))?;

    if let Some(error) = response_value.get("error").and_then(|e| e.as_str()) {
        if error.contains("not found") {
            return Err(ApiError::NotFoundError(error.to_string()));
        } else if error.contains("incorrect") {
            return Err(ApiError::AuthError(error.to_string()));
        }
        return Err(ApiError::ValidationError(error.to_string()));
    }

    // Log the action (using rcp-server audit system)
    let audit_command = "add-audit-log";
    let audit_args = serde_json::to_vec(&serde_json::json!({
        "user_id": auth_user.id,
        "action": "change_password",
        "entity_type": "user",
        "entity_id": id,
        "details": null
    }))?;

    let _ = service_client
        .send_command(audit_command, &audit_args)
        .await;

    Ok(StatusCode::NO_CONTENT)
}
