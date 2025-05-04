use crate::app::AppState;
use crate::auth::{Claims, JwtAuth};
use crate::error::{ApiError, ApiResult};
use crate::models::{LoginCredentials, TokenResponse, User};
use crate::utils::verify_password;
use axum::{extract::State, Json};
use std::sync::Arc;

/// Login handler that authenticates a user and returns a JWT token
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<LoginCredentials>,
) -> ApiResult<Json<TokenResponse>> {
    // Query the user by username (updated for SurrealDB 2.3.0)
    let result = state.db.query("SELECT * FROM user WHERE username = $username LIMIT 1")
        .bind(("username", credentials.username.clone()))
        .await
        .map_err(|e| ApiError::Database(format!("Database error: {}", e)))?;

    // Extract the user from the query result
    let users: Vec<User> = result.take(0)
        .map_err(|_| ApiError::Authentication("Invalid username or password".to_string()))?;

    let user = users
        .first()
        .ok_or_else(|| ApiError::Authentication("Invalid username or password".to_string()))?;

    // Verify the password
    let password_matches = verify_password(&credentials.password, &user.password_hash)
        .map_err(|e| ApiError::Internal(e))?;

    if !password_matches {
        return Err(ApiError::Authentication("Invalid username or password".to_string()));
    }

    // Get token expiry from environment or use default
    let token_expiry_hours = std::env::var("TOKEN_EXPIRY_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>()
        .unwrap_or(24);

    // Create JWT claims for the user
    let claims = Claims::new(user, token_expiry_hours)?;

    // Generate JWT token
    let token = JwtAuth::global().create_token(&claims)?;

    // Return the token response
    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: token_expiry_hours * 3600, // Convert hours to seconds
    }))
}

/// Logout handler
pub async fn logout() -> ApiResult<Json<serde_json::Value>> {
    // In a stateless JWT system, the client simply discards the token
    // We could implement a token blacklist for true logout, but that's beyond the scope here

    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

/// Get the current user's profile
pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    user: crate::auth::AuthUser,
) -> ApiResult<Json<serde_json::Value>> {
    // Query the user by ID to get the full profile (updated for SurrealDB 2.3.0)
    let user_id = format!("user:{}", user.user_id);
    
    let result = state.db.query("SELECT id, username, email, role, created_at FROM user WHERE id = $id LIMIT 1")
        .bind(("id", user_id))
        .await
        .map_err(|e| ApiError::Database(format!("Database error: {}", e)))?;

    // Extract the user profile from the query result
    let profile: serde_json::Value = result.take(0)
        .map_err(|_| ApiError::NotFound("User profile not found".to_string()))?;

    Ok(Json(profile))
}