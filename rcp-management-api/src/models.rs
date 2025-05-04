use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use surrealdb::opt::RecordId;
use crate::auth::WithId;

// User model for database and API
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<RecordId>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: String,
    pub role: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

// Login credentials DTO
#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

// Auth token response
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

// Server model
#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: Option<RecordId>,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub status: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

// Create server request DTO
#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub host: String,
    pub port: i32,
}

// Update server request DTO
#[derive(Debug, Deserialize)]
pub struct UpdateServerRequest {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
}

// Session model
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Option<RecordId>,
    pub server_id: RecordId,
    pub client_id: String,
    pub client_ip: String,
    pub started_at: Option<OffsetDateTime>,
    pub last_activity: Option<OffsetDateTime>,
    pub status: String,
}

// Statistics overview response
#[derive(Debug, Serialize)]
pub struct StatsOverview {
    pub active_servers: i64,
    pub active_sessions: i64,
    pub total_sessions_today: i64,
    pub total_users: i64,
    pub uptime: i64,
}

// Helper trait implementation for models to get their ID as a string
impl WithId for User {
    fn get_id(&self) -> Option<String> {
        self.id.as_ref().map(|record_id| record_id.to_string())
    }
}

impl WithId for Server {
    fn get_id(&self) -> Option<String> {
        self.id.as_ref().map(|record_id| record_id.to_string())
    }
}

impl WithId for Session {
    fn get_id(&self) -> Option<String> {
        self.id.as_ref().map(|record_id| record_id.to_string())
    }
}