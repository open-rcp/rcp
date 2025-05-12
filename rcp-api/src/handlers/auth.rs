use axum::{
    async_trait,
    extract::{Json, State},
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{db, ApiError, AppState};

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

/// Authentication response with token
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
    user: UserInfo,
    expires_at: String,
}

/// User information for response
#[derive(Debug, Serialize)]
pub struct UserInfo {
    id: String,
    username: String,
    role: String,
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,  // user_id
    exp: usize,   // expiration time
    iat: usize,   // issued at
    role: String, // user role
}

/// User representation in authentication middleware
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub role: String,
}

/// Login handler
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Prepare authentication request to the service
    let auth_request = serde_json::json!({
        "username": payload.username,
        "password": payload.password
    });

    // Send authentication request to service
    let command = "authenticate-user";
    let args = serde_json::to_vec(&auth_request)?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::AuthError(format!("Authentication failed: {}", e)))?;

    // Parse the authentication response
    let auth_result: serde_json::Value = serde_json::from_slice(&response)?;

    // Check if authentication was successful
    if let Some(error) = auth_result.get("error").and_then(|e| e.as_str()) {
        return Err(ApiError::AuthError(error.to_string()));
    }

    // Extract user information
    let user_id = auth_result
        .get("id")
        .and_then(|id| id.as_str())
        .ok_or_else(|| {
            ApiError::ServiceError("Invalid response from service: missing user ID".to_string())
        })?
        .to_string();

    let username = auth_result
        .get("username")
        .and_then(|u| u.as_str())
        .ok_or_else(|| {
            ApiError::ServiceError("Invalid response from service: missing username".to_string())
        })?
        .to_string();

    let role = auth_result
        .get("role")
        .and_then(|r| r.as_str())
        .ok_or_else(|| {
            ApiError::ServiceError("Invalid response from service: missing role".to_string())
        })?
        .to_string();

    // Create JWT token for API authentication
    let config = Arc::clone(&state.config);
    let expiration = Utc::now() + Duration::minutes(config.jwt_expiration_minutes as i64);
    let exp = expiration.timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: user_id.clone(),
        exp,
        iat,
        role: role.clone(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::ServerError(format!("Failed to create token: {}", e)))?;

    // Store token in API database for local validation
    let expires_at = expiration.to_rfc3339();
    db::create_token(&state.db_pool, 60).await?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&user_id),
        "user_login",
        Some("user"),
        Some(&user_id),
        None,
    )
    .await?;

    // Return auth response
    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user_id,
            username,
            role,
        },
        expires_at,
    };

    Ok(Json(response))
}

/// Auth middleware extractor
#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Get app state
        let app_state = parts
            .extensions
            .get::<AppState>()
            .ok_or_else(|| ApiError::ServerError("Application state not found".to_string()))?;

        // Get authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .ok_or_else(|| ApiError::AuthError("Missing Authorization header".to_string()))?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| ApiError::AuthError("Invalid Authorization header".to_string()))?;

        // Extract token from Bearer format
        if !auth_str.starts_with("Bearer ") {
            return Err(ApiError::AuthError(
                "Invalid Authorization format".to_string(),
            ));
        }

        let token = auth_str[7..].trim();
        if token.is_empty() {
            return Err(ApiError::AuthError("Empty JWT token".to_string()));
        }

        // Decode and validate token
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(app_state.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| ApiError::AuthError(format!("Invalid token: {}", e)))?;

        let claims = token_data.claims;

        // Verify token is valid with the service
        let service_client = app_state.service_client.lock().await;

        // Create token verification command
        let verify_request = serde_json::json!({
            "user_id": claims.sub.clone(),
            "token": token
        });

        // Send verification request to service
        let command = "verify-token";
        let args = serde_json::to_vec(&verify_request).map_err(|e| {
            ApiError::ServerError(format!("Failed to serialize token verification: {}", e))
        })?;

        let verification_result = service_client.send_command(command, &args).await;

        // Check if verification was successful - if not, proceed with token contents
        // This allows the API to work even if the service is temporarily unavailable
        if let Ok(response) = verification_result {
            let result: serde_json::Value = serde_json::from_slice(&response)
                .map_err(|_| ApiError::AuthError("Invalid response from service".to_string()))?;

            if let Some(valid) = result.get("valid").and_then(|v| v.as_bool()) {
                if !valid {
                    return Err(ApiError::AuthError("Token rejected by service".to_string()));
                }
            }
        }

        // Create auth user
        Ok(AuthUser {
            id: claims.sub.clone(),
            role: claims.role.clone(),
        })
    }
}

/// Admin role middleware
pub async fn require_admin<B>(
    auth_user: AuthUser,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    if auth_user.role != "admin" {
        return Err(ApiError::ForbiddenError(
            "Admin access required".to_string(),
        ));
    }

    Ok(next.run(request).await)
}

/// Operator role or higher middleware
pub async fn require_operator<B>(
    auth_user: AuthUser,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    if auth_user.role != "admin" && auth_user.role != "operator" {
        return Err(ApiError::ForbiddenError(
            "Operator access required".to_string(),
        ));
    }

    Ok(next.run(request).await)
}

/// Token refresh handler
pub async fn refresh_token(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<AuthResponse>, ApiError> {
    let service_client = state.service_client.lock().await;

    // Create token refresh command
    let refresh_request = serde_json::json!({
        "user_id": auth_user.id
    });

    // Send refresh request to service
    let command = "refresh-token";
    let args = serde_json::to_vec(&refresh_request)?;

    let response = service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::AuthError(format!("Token refresh failed: {}", e)))?;

    // Parse the refresh response
    let refresh_result: serde_json::Value = serde_json::from_slice(&response)?;

    // Check if refresh was successful
    if let Some(error) = refresh_result.get("error").and_then(|e| e.as_str()) {
        return Err(ApiError::AuthError(error.to_string()));
    }

    // Extract user information from the refresh
    let username = refresh_result
        .get("username")
        .and_then(|u| u.as_str())
        .ok_or_else(|| {
            ApiError::ServiceError("Invalid response from service: missing username".to_string())
        })?
        .to_string();

    // Create new JWT token for API authentication
    let config = Arc::clone(&state.config);
    let expiration = Utc::now() + Duration::minutes(config.jwt_expiration_minutes as i64);
    let exp = expiration.timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: auth_user.id.clone(),
        exp,
        iat,
        role: auth_user.role.clone(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::ServerError(format!("Failed to create token: {}", e)))?;

    // Store token in API database for local validation
    let expires_at = expiration.to_rfc3339();
    db::create_token(&state.db_pool, 60).await?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "token_refresh",
        Some("user"),
        Some(&auth_user.id),
        None,
    )
    .await?;

    // Return auth response
    let response = AuthResponse {
        token,
        user: UserInfo {
            id: auth_user.id.clone(),
            username,
            role: auth_user.role.clone(),
        },
        expires_at,
    };

    Ok(Json(response))
}

/// Authorization middleware
pub async fn auth_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    // Extract app state from request extensions
    let state = request
        .extensions()
        .get::<AppState>()
        .cloned()
        .ok_or_else(|| {
            ApiError::ServerError("Application state missing from request".to_string())
        })?;
    // Extract auth header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| ApiError::AuthError("Missing Authorization header".to_string()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| ApiError::AuthError("Invalid Authorization header".to_string()))?;

    // Extract token from Bearer format
    if !auth_str.starts_with("Bearer ") {
        return Err(ApiError::AuthError(
            "Invalid Authorization format".to_string(),
        ));
    }

    let token = auth_str[7..].trim();
    if token.is_empty() {
        return Err(ApiError::AuthError("Empty JWT token".to_string()));
    }

    // Decode and validate token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| ApiError::AuthError(format!("Invalid token: {}", e)))?;

    let claims = token_data.claims;

    // Create auth user and add to request extensions
    let auth_user = AuthUser {
        id: claims.sub,
        role: claims.role,
    };

    request.extensions_mut().insert(auth_user);

    // Continue to the next middleware or handler
    Ok(next.run(request).await)
}

/// Admin middleware
pub async fn admin_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    // Extract app state from request extensions
    let state = request
        .extensions()
        .get::<AppState>()
        .cloned()
        .ok_or_else(|| {
            ApiError::ServerError("Application state missing from request".to_string())
        })?;

    // Extract auth header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| ApiError::AuthError("Missing Authorization header".to_string()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| ApiError::AuthError("Invalid Authorization header".to_string()))?;

    // Extract token from Bearer format
    if !auth_str.starts_with("Bearer ") {
        return Err(ApiError::AuthError(
            "Invalid Authorization format".to_string(),
        ));
    }

    let token = auth_str[7..].trim();
    if token.is_empty() {
        return Err(ApiError::AuthError("Empty JWT token".to_string()));
    }

    // Decode and validate token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| ApiError::AuthError(format!("Invalid token: {}", e)))?;

    let claims = token_data.claims;

    // Check if the user is an admin
    if claims.role != "admin" {
        return Err(ApiError::ForbiddenError(
            "Admin access required".to_string(),
        ));
    }

    // Create auth user and add to request extensions
    let auth_user = AuthUser {
        id: claims.sub,
        role: claims.role,
    };

    request.extensions_mut().insert(auth_user);

    // Continue to the next middleware or handler
    Ok(next.run(request).await)
}

/// Health check endpoint (public)
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

/// Get current auth status
pub async fn auth_status(auth_user: AuthUser) -> Result<Json<UserInfo>, ApiError> {
    // This endpoint just verifies that auth is working
    Ok(Json(UserInfo {
        id: auth_user.id.clone(),
        username: "authenticated".to_string(), // We don't have the username here, just the ID
        role: auth_user.role.clone(),
    }))
}

/// Logout handler
pub async fn logout(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<StatusCode, ApiError> {
    // In a real implementation, we would blacklist the token
    // For now, we just log the logout
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "user_logout",
        None,
        None,
        None,
    )
    .await?;

    Ok(StatusCode::OK)
}
