use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    db::Database,
    error::{AppError, Result},
    models::{CreateUser, UpdateUser, User, UserDb, UserInfo, UserInfoDb},
};

pub struct UserService {
    db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User> {
        // Check if username already exists
        let existing = query("SELECT id FROM users WHERE username = ?")
            .bind(&user.username)
            .fetch_optional(self.db.pool())
            .await?;

        if existing.is_some() {
            return Err(AppError::Validation(format!(
                "User with username '{}' already exists",
                user.username
            )));
        }

        // Hash password
        let password_hash = hash(&user.password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash password: {}", e)))?;

        // Create new user
        let now = Utc::now();
        let id = Uuid::new_v4();

        let created_user = query_as::<_, UserDb>(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, is_active, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
            "#
        )
        .bind(id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&password_hash)
        .bind(user.role.to_string())
        .bind(user.is_active)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .fetch_one(self.db.pool())
        .await?;

        Ok(created_user.into())
    }

    pub async fn get_users(&self) -> Result<Vec<UserInfo>> {
        let users = query_as::<_, UserInfoDb>(
            "SELECT id, username, email, role, is_active FROM users ORDER BY username ASC",
        )
        .fetch_all(self.db.pool())
        .await?;

        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User> {
        let user = query_as::<_, UserDb>(
            "SELECT id, username, email, password_hash, role, is_active, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_one(self.db.pool())
        .await?;

        Ok(user.into())
    }

    pub async fn update_user(&self, id: Uuid, update: UpdateUser) -> Result<User> {
        let now = Utc::now();

        let updated_user = query_as::<_, UserDb>(
            r#"
            UPDATE users
            SET username = COALESCE(?1, username),
                email = COALESCE(?2, email),
                role = COALESCE(?3, role),
                is_active = COALESCE(?4, is_active),
                updated_at = ?5
            WHERE id = ?6
            RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
            "#,
        )
        .bind(update.username)
        .bind(update.email)
        .bind(update.role.map(|r| r.to_string()))
        .bind(update.is_active)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .fetch_one(self.db.pool())
        .await?;

        Ok(updated_user.into())
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        let result = query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }
}
