use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

// Database models that match SQLite schema exactly
#[derive(Debug, Clone, FromRow)]
pub struct UserDb {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<UserDb> for User {
    fn from(db_user: UserDb) -> Self {
        Self {
            id: Uuid::parse_str(&db_user.id).unwrap_or_default(),
            username: db_user.username,
            email: db_user.email,
            password_hash: db_user.password_hash,
            role: UserRole::from_str(&db_user.role).unwrap_or(UserRole::Viewer),
            is_active: db_user.is_active,
            created_at: DateTime::parse_from_rfc3339(&db_user.created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&db_user.updated_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct UserInfoDb {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
}

impl From<UserInfoDb> for UserInfo {
    fn from(db_user: UserInfoDb) -> Self {
        Self {
            id: Uuid::parse_str(&db_user.id).unwrap_or_default(),
            username: db_user.username,
            email: db_user.email,
            role: UserRole::from_str(&db_user.role).unwrap_or(UserRole::Viewer),
            is_active: db_user.is_active,
        }
    }
}

// API models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Operator,
    Viewer,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Operator => write!(f, "Operator"),
            UserRole::Viewer => write!(f, "Viewer"),
        }
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(UserRole::Admin),
            "Operator" => Ok(UserRole::Operator),
            "Viewer" => Ok(UserRole::Viewer),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

// RCP Daemon Models

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub version: String,
    pub uptime: u64,
    pub active_sessions: u32,
    pub system_metrics: SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub total_memory: u64,
    pub disk_usage: u64,
    pub total_disk: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub application_id: String,
    pub user_id: Option<String>,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub status: SessionStatus,
    pub metrics: SessionMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Idle,
    Disconnected,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub network_rx: u64,
    pub network_tx: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub path: String,
    pub launch_command: String,
    pub arguments: Vec<String>,
    pub environment: std::collections::HashMap<String, String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApplication {
    pub name: String,
    pub version: String,
    pub description: String,
    pub path: String,
    pub launch_command: String,
    pub arguments: Vec<String>,
    pub environment: std::collections::HashMap<String, String>,
}
