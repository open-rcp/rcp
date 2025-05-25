use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    db::Database,
    error::{AppError, Result},
    models::{LoginRequest, User, UserDb},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: String,
    pub exp: usize,
}

pub struct AuthService {
    db: Database,
}

impl AuthService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn authenticate(&self, creds: &LoginRequest) -> Result<User> {
        // Validate username format
        if !creds
            .username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(AppError::Validation("Invalid username format".to_string()));
        }

        let user_db = query_as::<_, UserDb>(
            "SELECT id, username, email, password_hash, role, is_active, created_at, updated_at FROM users WHERE username = ? AND is_active = 1",
        )
        .bind(&creds.username)
        .fetch_optional(self.db.pool())
        .await?;

        let user_db =
            user_db.ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // Verify password
        let password_valid = verify(&creds.password, &user_db.password_hash).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Password verification failed: {}", e))
        })?;

        if !password_valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        Ok(user_db.into())
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User> {
        let user_db = query_as::<_, UserDb>(
            "SELECT id, username, email, password_hash, role, is_active, created_at, updated_at FROM users WHERE id = ? AND is_active = 1",
        )
        .bind(user_id.to_string())
        .fetch_optional(self.db.pool())
        .await?;

        let user_db = user_db.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        Ok(user_db.into())
    }

    pub fn create_token(&self, user: &User) -> Result<String> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            role: user.role.to_string(),
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Token creation failed: {}", e)))?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Unauthorized(format!("Token verification failed: {}", e)))?;

        Ok(token_data.claims)
    }
}
