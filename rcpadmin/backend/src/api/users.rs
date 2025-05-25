use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use bcrypt::{hash, DEFAULT_COST};
use tracing::info;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{CreateUser, UpdateUser, User, UserInfo, UserRole},
    AppState,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<UserInfo>>> {
    // In a real app, fetch users from database
    // For this example, return a mock list
    let users = vec![
        UserInfo {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: UserRole::Admin,
            is_active: true,
        },
        UserInfo {
            id: Uuid::new_v4(),
            username: "operator".to_string(),
            email: "operator@example.com".to_string(),
            role: UserRole::Operator,
            is_active: true,
        },
        UserInfo {
            id: Uuid::new_v4(),
            username: "viewer".to_string(),
            email: "viewer@example.com".to_string(),
            role: UserRole::Viewer,
            is_active: false,
        },
    ];
    
    info!("Retrieved {} users", users.len());
    Ok(Json(users))
}

async fn get_user(
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserInfo>> {
    // In a real app, fetch user from database
    // For this example, return a mock user
    
    let user = UserInfo {
        id: user_id,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        role: UserRole::Admin,
        is_active: true,
    };
    
    info!("Retrieved user {}", user_id);
    Ok(Json(user))
}

async fn create_user(
    Json(create_user): Json<CreateUser>,
) -> Result<Json<UserInfo>> {
    // In a real app, save to database
    // For this example, mock the creation
    
    // Hash the password
    let password_hash = hash(&create_user.password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash password: {}", e)))?;
    
    let user = UserInfo {
        id: Uuid::new_v4(),
        username: create_user.username,
        email: create_user.email,
        role: create_user.role,
        is_active: true,
    };
    
    info!("Created user {}", user.id);
    Ok(Json(user))
}

async fn update_user(
    Path(user_id): Path<Uuid>,
    Json(update): Json<UpdateUser>,
) -> Result<Json<UserInfo>> {
    // In a real app, update in database
    // For this example, mock the update
    
    let user = UserInfo {
        id: user_id,
        username: "admin".to_string(),
        email: update.email.unwrap_or_else(|| "admin@example.com".to_string()),
        role: update.role.unwrap_or(UserRole::Admin),
        is_active: update.is_active.unwrap_or(true),
    };
    
    info!("Updated user {}", user_id);
    Ok(Json(user))
}

async fn delete_user(
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    // In a real app, delete from database or mark as inactive
    // For this example, just return success
    
    info!("Deleted user {}", user_id);
    Ok(Json(serde_json::json!({
        "message": format!("User {} deleted successfully", user_id),
        "success": true
    })))
}