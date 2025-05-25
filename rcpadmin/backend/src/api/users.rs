use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    error::Result,
    models::{CreateUser, UpdateUser, UserInfo, UserRole},
    AppState,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

async fn list_users(_state: State<AppState>) -> Result<Json<Vec<UserInfo>>> {
    // Mock implementation - in real app, use state.db to query users
    let users = vec![UserInfo {
        id: Uuid::new_v4(),
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        role: UserRole::Admin,
        is_active: true,
    }];
    info!("Retrieved {} users", users.len());
    Ok(Json(users))
}

async fn get_user(Path(_user_id): Path<Uuid>) -> Result<Json<UserInfo>> {
    // Mock implementation
    let user = UserInfo {
        id: Uuid::new_v4(),
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        role: UserRole::Admin,
        is_active: true,
    };
    Ok(Json(user))
}

async fn create_user(Json(_create_user): Json<CreateUser>) -> Result<Json<UserInfo>> {
    // Mock implementation
    let user = UserInfo {
        id: Uuid::new_v4(),
        username: "newuser".to_string(),
        email: "newuser@example.com".to_string(),
        role: UserRole::Viewer,
        is_active: true,
    };
    info!("Created new user: {}", user.username);
    Ok(Json(user))
}

async fn update_user(
    Path(_user_id): Path<Uuid>,
    Json(_update): Json<UpdateUser>,
) -> Result<Json<UserInfo>> {
    // Mock implementation
    let user = UserInfo {
        id: Uuid::new_v4(),
        username: "updateduser".to_string(),
        email: "updated@example.com".to_string(),
        role: UserRole::Operator,
        is_active: true,
    };
    info!("Updated user: {}", user.username);
    Ok(Json(user))
}

async fn delete_user(Path(user_id): Path<Uuid>) -> Result<Json<serde_json::Value>> {
    info!("Deleted user: {}", user_id);
    Ok(Json(serde_json::json!({
        "message": format!("User {} deleted successfully", user_id),
        "success": true
    })))
}
