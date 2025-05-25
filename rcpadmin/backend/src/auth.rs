use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{User, UserRole},
    AppState,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub role: String,
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
}

pub async fn protect(State(state): State<AppState>, mut request: Request, next: Next) -> Response {
    // Extract the Authorization header manually
    let auth_header = match request.headers().get("authorization") {
        Some(header) => header,
        None => return AppError::Auth("Missing authorization header".to_string()).into_response(),
    };

    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            return AppError::Auth("Invalid authorization header".to_string()).into_response()
        }
    };

    let token = if auth_str.starts_with("Bearer ") {
        &auth_str[7..]
    } else {
        return AppError::Auth("Invalid authorization format".to_string()).into_response();
    };

    match validate_token(token, &state.config.jwt_secret) {
        Ok(claims) => {
            // Verify the token is valid
            match Uuid::parse_str(&claims.sub) {
                Ok(_user_id) => {
                    // Insert claims into request extensions for use in handlers
                    request.extensions_mut().insert(claims);
                    next.run(request).await
                }
                Err(_) => AppError::Auth("Invalid token".to_string()).into_response(),
            }
        }
        Err(err) => err.into_response(),
    }
}

pub async fn require_admin(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract the Authorization header manually
    let auth_header = match request.headers().get("authorization") {
        Some(header) => header,
        None => return AppError::Auth("Missing authorization header".to_string()).into_response(),
    };

    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            return AppError::Auth("Invalid authorization header".to_string()).into_response()
        }
    };

    let token = if auth_str.starts_with("Bearer ") {
        &auth_str[7..]
    } else {
        return AppError::Auth("Invalid authorization format".to_string()).into_response();
    };

    match validate_token(token, &state.config.jwt_secret) {
        Ok(claims) => {
            // Check if user has admin role
            if claims.role != UserRole::Admin.to_string() {
                AppError::Unauthorized("Admin role required".to_string()).into_response()
            } else {
                // Insert claims into request extensions for use in handlers
                request.extensions_mut().insert(claims);
                next.run(request).await
            }
        }
        Err(err) => err.into_response(),
    }
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Auth(format!("Invalid token: {}", e)))?;

    Ok(decoded.claims)
}

pub fn generate_token(user: &User, secret: &str, expires_in_days: i64) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(expires_in_days))
        .expect("Valid timestamp")
        .timestamp();

    let issued_at = Utc::now().timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        role: user.role.to_string(),
        exp: expiration,
        iat: issued_at,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to generate token: {}", e)))?;

    Ok(token)
}
