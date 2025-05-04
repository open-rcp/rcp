use axum::{
    extract::{State, Json},
    http::{StatusCode, Request, header},
    response::{Response, IntoResponse},
    async_trait,
    middleware::Next,
    RequestExt,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use std::sync::Arc;

use crate::{AppState, ApiError, db, config::Config};

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
    sub: String, // user_id
    exp: usize,  // expiration time
    iat: usize,  // issued at
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
    // Get user from database
    let user = db::get_user_by_username(&state.db_pool, &payload.username).await?
        .ok_or_else(|| ApiError::AuthError("Invalid username or password".to_string()))?;
    
    // Check if user is active
    if !user.active {
        return Err(ApiError::AuthError("Account is disabled".to_string()));
    }
    
    // Verify password (in a real app, use bcrypt or argon2)
    // This is a placeholder for demonstration - in production use a proper verification
    if !user.password_hash.ends_with(&payload.password) {
        return Err(ApiError::AuthError("Invalid username or password".to_string()));
    }
    
    // Update last login time
    db::update_last_login(&state.db_pool, &user.id).await?;
    
    // Create token
    let config = Arc::clone(&state.config);
    let expiration = Utc::now() + Duration::minutes(config.jwt_expiration_minutes as i64);
    let exp = expiration.timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;
    
    let claims = Claims {
        sub: user.id.clone(),
        exp,
        iat,
        role: user.role.clone(),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::ServerError(format!("Failed to create token: {}", e)))?;
    
    // Store token in database
    let expires_at = expiration.to_rfc3339();
    db::create_token(&state.db_pool, &user.id, &token, &expires_at).await?;
    
    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&user.id),
        "user_login",
        None,
        None,
        None,
    ).await?;
    
    // Return auth response
    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user.id,
            username: user.username,
            role: user.role,
        },
        expires_at,
    };
    
    Ok(Json(response))
}

/// Auth middleware extractor
#[async_trait]
impl<S, B> axum::extract::FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Get app state
        let app_state = parts.extensions.get::<AppState>()
            .ok_or_else(|| ApiError::ServerError("Application state not found".to_string()))?;
        
        // Get authorization header
        let auth_header = parts.headers
            .get(header::AUTHORIZATION)
            .ok_or_else(|| ApiError::AuthError("Missing Authorization header".to_string()))?;
        
        let auth_str = auth_header.to_str()
            .map_err(|_| ApiError::AuthError("Invalid Authorization header".to_string()))?;
        
        // Extract token from Bearer format
        if !auth_str.starts_with("Bearer ") {
            return Err(ApiError::AuthError("Invalid Authorization format".to_string()));
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
        
        // Verify user still exists and is active
        let user = sqlx::query_as!(
            db::User,
            r#"SELECT id, username, password_hash, role, active, created_at, last_login FROM users WHERE id = ?"#,
            claims.sub
        )
        .fetch_optional(&app_state.db_pool)
        .await?;
        
        let user = user.ok_or_else(|| ApiError::AuthError("User not found".to_string()))?;
        
        if !user.active {
            return Err(ApiError::AuthError("User account is disabled".to_string()));
        }
        
        // Create auth user
        Ok(AuthUser {
            id: user.id,
            role: user.role,
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
        return Err(ApiError::ForbiddenError("Admin access required".to_string()));
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
        return Err(ApiError::ForbiddenError("Operator access required".to_string()));
    }
    
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
    ).await?;
    
    Ok(StatusCode::OK)
}