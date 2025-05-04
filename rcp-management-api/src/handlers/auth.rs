use crate::{ApiResult, AppState};
use actix_web::{web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    role: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

/// Login endpoint to authenticate users
pub async fn login(
    app_state: web::Data<AppState>,
    login: web::Json<LoginRequest>,
) -> ApiResult<HttpResponse> {
    // In a real implementation, we would validate against a database
    // Here we're doing a simple check for demonstration
    if login.username == "admin" && login.password == "password" {
        // Create a JWT token
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 24 * 3600; // 24 hours

        let claims = Claims {
            sub: "1".to_string(),
            exp: expiration as usize,
            role: "admin".to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(app_state.jwt_secret.as_ref()),
        )
        .unwrap_or_else(|e| {
            error!("Error encoding JWT token: {}", e);
            String::new()
        });

        let user = User {
            id: "1".to_string(),
            username: login.username.clone(),
            role: "admin".to_string(),
        };

        Ok(HttpResponse::Ok().json(LoginResponse { token, user }))
    } else {
        Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "status": "error",
            "message": "Invalid username or password"
        })))
    }
}

/// Logout endpoint (client-side only in this implementation)
pub async fn logout() -> ApiResult<HttpResponse> {
    // In this implementation, logout is handled client-side by removing the token
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Logged out successfully"
    })))
}

/// Get current authenticated user information
pub async fn get_current_user(
    app_state: web::Data<AppState>,
    // In a real implementation, we would extract the user from JWT token
    // For now, we'll just return a mock user
) -> ApiResult<HttpResponse> {
    let user = User {
        id: "1".to_string(),
        username: "admin".to_string(),
        role: "admin".to_string(),
    };

    Ok(HttpResponse::Ok().json(user))
}
