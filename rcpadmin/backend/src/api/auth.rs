use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use crate::{
    auth::generate_token,
    error::{AppError, Result},
    models::{LoginRequest, LoginResponse, User, UserInfo, UserRole},
    AppState,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/validate", get(validate_token))
}

async fn login(
    State(state): State<AppState>,
    Json(login): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    // For simplicity in this example, we'll use a hardcoded admin user
    // In a real application, fetch from the database

    // Check if we have a hardcoded user match
    if login.username == "admin" {
        // Check password (hardcoded for example, use proper password hashing in production)
        if login.password == "rcpadmin" {
            // Create a mock user
            let user = User {
                id: Uuid::new_v4(),
                username: "admin".to_string(),
                email: "admin@example.com".to_string(),
                password_hash: "".to_string(), // Not needed for this flow
                role: UserRole::Admin,
                is_active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // Generate JWT token
            let token = generate_token(&user, &state.config.jwt_secret, 7)?;

            // Return user info and token
            return Ok(Json(LoginResponse {
                token,
                user: UserInfo {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    role: user.role,
                    is_active: user.is_active,
                },
            }));
        }
    }

    // If we get here, authentication failed
    Err(AppError::Auth("Invalid username or password".to_string()))
}

async fn validate_token() -> Result<Json<serde_json::Value>> {
    // This endpoint is just a placeholder
    // The actual token validation happens in the middleware
    Ok(Json(serde_json::json!({
        "valid": true
    })))
}
