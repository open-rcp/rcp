use crate::{ApiResult, AppState};
use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub active: bool,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
    pub active: Option<bool>,
    pub password: Option<String>,
}

/// Get all users
pub async fn get_users(app_state: web::Data<AppState>) -> ApiResult<HttpResponse> {
    // In a real implementation, we would fetch users from the database
    // Here we return mock data for demonstration
    let users = vec![
        User {
            id: "1".to_string(),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
            active: true,
            password_hash: None,
        },
        User {
            id: "2".to_string(),
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            role: "user".to_string(),
            active: true,
            password_hash: None,
        },
    ];

    Ok(HttpResponse::Ok().json(users))
}

/// Get a specific user by ID
pub async fn get_user(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let user_id = path.into_inner();
    
    // In a real implementation, we would fetch the user from the database
    let user = User {
        id: user_id,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        role: "admin".to_string(),
        active: true,
        password_hash: None,
    };

    Ok(HttpResponse::Ok().json(user))
}

/// Create a new user
pub async fn create_user(
    app_state: web::Data<AppState>,
    user_req: web::Json<CreateUserRequest>,
) -> ApiResult<HttpResponse> {
    // In a real implementation, we would validate and store the user in the database
    info!("Creating new user: {}", user_req.username);
    
    // Generate a random ID for the new user
    let user_id = Uuid::new_v4().to_string();
    
    let user = User {
        id: user_id,
        username: user_req.username.clone(),
        email: user_req.email.clone(),
        role: user_req.role.clone(),
        active: true,
        password_hash: None,
    };

    Ok(HttpResponse::Created().json(user))
}

/// Update an existing user
pub async fn update_user(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    user_req: web::Json<UpdateUserRequest>,
) -> ApiResult<HttpResponse> {
    let user_id = path.into_inner();
    
    // In a real implementation, we would update the user in the database
    info!("Updating user with ID: {}", user_id);
    
    let user = User {
        id: user_id,
        username: "admin".to_string(),
        email: user_req.email.clone().unwrap_or_else(|| "admin@example.com".to_string()),
        role: user_req.role.clone().unwrap_or_else(|| "admin".to_string()),
        active: user_req.active.unwrap_or(true),
        password_hash: None,
    };

    Ok(HttpResponse::Ok().json(user))
}

/// Delete a user
pub async fn delete_user(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let user_id = path.into_inner();
    
    // In a real implementation, we would delete or deactivate the user in the database
    info!("Deleting user with ID: {}", user_id);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "User deleted successfully"
    })))
}