use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, ApiError, db};
use crate::handlers::auth::AuthUser;

/// User information response
#[derive(Debug, Serialize)]
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
    // Query all users from the database
    let users = sqlx::query_as!(
        db::User,
        r#"SELECT id, username, password_hash, role, active, created_at, last_login FROM users"#
    )
    .fetch_all(&state.db_pool)
    .await?;
    
    // Convert to response format, excluding password hash
    let responses = users.into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.username,
            role: user.role,
            active: user.active,
            created_at: user.created_at,
            last_login: user.last_login,
        })
        .collect();
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "list_users",
        None,
        None,
        None
    ).await?;
    
    Ok(Json(responses))
}

/// Get a user by ID
pub async fn get_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, ApiError> {
    // Query user from the database
    let user = sqlx::query_as!(
        db::User,
        r#"SELECT id, username, password_hash, role, active, created_at, last_login FROM users WHERE id = ?"#,
        id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| ApiError::NotFoundError(format!("User '{}' not found", id)))?;
    
    // Convert to response format
    let response = UserResponse {
        id: user.id,
        username: user.username,
        role: user.role,
        active: user.active,
        created_at: user.created_at,
        last_login: user.last_login,
    };
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_user",
        Some("user"),
        Some(&id),
        None
    ).await?;
    
    Ok(Json(response))
}

/// Get the current user's profile
pub async fn get_profile(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, ApiError> {
    // Query user from the database
    let user = sqlx::query_as!(
        db::User,
        r#"SELECT id, username, password_hash, role, active, created_at, last_login FROM users WHERE id = ?"#,
        auth_user.id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| ApiError::AuthError("User account no longer exists".to_string()))?;
    
    // Convert to response format
    let response = UserResponse {
        id: user.id,
        username: user.username,
        role: user.role,
        active: user.active,
        created_at: user.created_at,
        last_login: user.last_login,
    };
    
    Ok(Json(response))
}

/// Create a new user
pub async fn create_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    // Check if username already exists
    let exists = db::user_exists(&state.db_pool, &payload.username).await?;
    if exists {
        return Err(ApiError::ConflictError(format!("Username '{}' is already taken", payload.username)));
    }
    
    // Validate role
    if !["admin", "operator", "viewer"].contains(&payload.role.as_str()) {
        return Err(ApiError::ValidationError("Invalid role. Must be 'admin', 'operator', or 'viewer'".to_string()));
    }
    
    // Hash password (in a real app, use bcrypt or argon2)
    // This is a placeholder - in production use a proper hash function
    let password_hash = format!("$2b$12$placeholder-hash-{}", payload.password);
    
    // Create new user
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO users (id, username, password_hash, role, active, created_at)
        VALUES (?, ?, ?, ?, 1, ?)
        "#
    )
    .bind(&id)
    .bind(&payload.username)
    .bind(&password_hash)
    .bind(&payload.role)
    .bind(&now)
    .execute(&state.db_pool)
    .await?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "create_user",
        Some("user"),
        Some(&id),
        Some(&format!("username={}, role={}", payload.username, payload.role))
    ).await?;
    
    // Return the created user info
    let response = UserResponse {
        id,
        username: payload.username,
        role: payload.role,
        active: true,
        created_at: now,
        last_login: None,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

/// Update a user
pub async fn update_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    // Get existing user
    let user = sqlx::query_as!(
        db::User,
        r#"SELECT id, username, password_hash, role, active, created_at, last_login FROM users WHERE id = ?"#,
        id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| ApiError::NotFoundError(format!("User '{}' not found", id)))?;
    
    // Prepare update fields
    let role = payload.role.unwrap_or(user.role);
    let active = payload.active.unwrap_or(user.active);
    
    // Validate role if it's being updated
    if payload.role.is_some() && !["admin", "operator", "viewer"].contains(&role.as_str()) {
        return Err(ApiError::ValidationError("Invalid role. Must be 'admin', 'operator', or 'viewer'".to_string()));
    }
    
    // Don't allow users to deactivate themselves
    if auth_user.id == id && Some(false) == payload.active {
        return Err(ApiError::ValidationError("Cannot deactivate your own account".to_string()));
    }
    
    // Update the user
    sqlx::query(
        r#"
        UPDATE users 
        SET role = ?, active = ?
        WHERE id = ?
        "#
    )
    .bind(&role)
    .bind(active)
    .bind(&id)
    .execute(&state.db_pool)
    .await?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "update_user",
        Some("user"),
        Some(&id),
        Some(&format!("role={}, active={}", role, active))
    ).await?;
    
    // Return the updated user info
    let response = UserResponse {
        id: user.id,
        username: user.username,
        role,
        active,
        created_at: user.created_at,
        last_login: user.last_login,
    };
    
    Ok(Json(response))
}

/// Delete a user
pub async fn delete_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Don't allow users to delete themselves
    if auth_user.id == id {
        return Err(ApiError::ValidationError("Cannot delete your own account".to_string()));
    }
    
    // Delete the user
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&id)
        .execute(&state.db_pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFoundError(format!("User '{}' not found", id)));
    }
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "delete_user",
        Some("user"),
        Some(&id),
        None
    ).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

/// Change user password
pub async fn change_password(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    // Get existing user
    let user = sqlx::query_as!(
        db::User,
        r#"SELECT id, username, password_hash, role, active, created_at, last_login FROM users WHERE id = ?"#,
        id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| ApiError::NotFoundError(format!("User '{}' not found", id)))?;
    
    // If changing own password, require current password
    if auth_user.id == id {
        if let Some(current_password) = &payload.current_password {
            // Verify current password (in a real app, use bcrypt or argon2)
            // This is a placeholder verification
            if !user.password_hash.ends_with(current_password) {
                return Err(ApiError::AuthError("Current password is incorrect".to_string()));
            }
        } else {
            return Err(ApiError::ValidationError("Current password is required".to_string()));
        }
    } else if auth_user.role != "admin" {
        // Only admins can change others' passwords
        return Err(ApiError::ForbiddenError("Only administrators can change other users' passwords".to_string()));
    }
    
    // Hash new password (in a real app, use bcrypt or argon2)
    // This is a placeholder - in production use a proper hash function
    let new_password_hash = format!("$2b$12$placeholder-hash-{}", payload.new_password);
    
    // Update the password
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(&new_password_hash)
        .bind(&id)
        .execute(&state.db_pool)
        .await?;
    
    // Log the action (don't include any password details in the log)
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "change_password",
        Some("user"),
        Some(&id),
        None
    ).await?;
    
    Ok(StatusCode::NO_CONTENT)
}