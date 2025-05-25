use crate::{
    db::Database,
    error::{AppError, Result},
    models::{LoginRequest, User},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

pub struct AuthService {
    db: Database,
}

impl AuthService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn login(&self, creds: LoginRequest) -> Result<TokenResponse> {
        // Find user by username
        let user = query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role as "role: _", is_active, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            creds.username
        )
        .fetch_optional(self.db.pool())
        .await?;

        let user = match user {
            Some(user) => user,
            None => return Err(AppError::Auth("Invalid credentials".to_string())),
        };

        // Verify password
        let password_matches = bcrypt::verify(&creds.password, &user.password_hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Password verification failed: {}", e)))?;
        if !password_matches {
            return Err(AppError::Auth("Invalid credentials".to_string()));
        }

        // Check if user is active
        if !user.is_active {
            return Err(AppError::Auth("Account is inactive".to_string()));
        }

        // Generate JWT token
        let token = self.create_token(&user)?;

        Ok(TokenResponse { token })
    }

    fn create_token(&self, user: &User) -> Result<String> {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
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
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Auth(format!("Token verification failed: {}", e)))?;

        Ok(token_data.claims)
    }

    pub async fn get_user_from_token(&self, token: &str) -> Result<User> {
        let claims = self.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::Auth(format!("Invalid user ID in token: {}", e)))?;
        
        let user = query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role as "role: _", is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(self.db.pool())
        .await?;

        user.ok_or_else(|| AppError::Auth("Invalid user token".to_string()))
    }
}