use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub role: String,
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
}

pub async fn protect(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Response {
    let token = auth.token();
    
    match validate_token(token, &state.config.jwt_secret) {
        Ok(claims) => {
            // Verify the token is valid
            match Uuid::parse_str(&claims.sub) {
                Ok(_user_id) => {
                    // Check if user exists and is active
                    // For high-security applications, you might want to verify the user from the database here
                    
                    // Continue with the request
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
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Response {
    let token = auth.token();
    
    match validate_token(token, &state.config.jwt_secret) {
        Ok(claims) => {
            // Check if user has admin role
            if claims.role != UserRole::Admin.to_string() {
                AppError::Unauthorized("Admin role required".to_string()).into_response()
            } else {
                // Continue with the request
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