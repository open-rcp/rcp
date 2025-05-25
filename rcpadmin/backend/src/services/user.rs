use crate::{
    db::Database,
    error::{AppError, Result},
    models::{CreateUser, UpdateUser, User, UserInfo, UserRole},
};
use chrono::Utc;
use sqlx::{query, query_as};
use uuid::Uuid;

pub struct UserService {
    db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User> {
        // Check if username already exists
        let existing = query!("SELECT id FROM users WHERE username = $1", user.username)
            .fetch_optional(self.db.pool())
            .await?;
            
        if existing.is_some() {
            return Err(AppError::Validation(format!(
                "User with username '{}' already exists",
                user.username
            )));
        }
        
        // Hash password
        let password_hash = bcrypt::hash(user.password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash password: {}", e)))?;
        
        // Create new user
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        let created_user = query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, role, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, username, email, password_hash, role as "role: _", is_active, created_at, updated_at
            "#,
            id,
            user.username,
            user.email,
            password_hash,
            user.role as _,
            true,
            now,
            now
        )
        .fetch_one(self.db.pool())
        .await?;
        
        Ok(created_user)
    }

    pub async fn get_users(&self) -> Result<Vec<UserInfo>> {
        let users = query!(
            r#"
            SELECT id, username, email, role as "role: _", is_active
            FROM users
            ORDER BY username ASC
            "#
        )
        .fetch_all(self.db.pool())
        .await?;
        
        Ok(users
            .into_iter()
            .map(|u| UserInfo {
                id: u.id,
                username: u.username,
                email: u.email,
                role: u.role,
            })
            .collect())
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User> {
        let user = query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role as "role: _", is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.db.pool())
        .await?;
        
        user.ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", id)))
    }

    pub async fn update_user(&self, id: Uuid, update: UpdateUser) -> Result<User> {
        let existing = self.get_user(id).await?;
        
        let email = update.email.unwrap_or(existing.email);
        let role = update.role.unwrap_or(existing.role);
        let is_active = update.is_active.unwrap_or(existing.is_active);
        let now = Utc::now();
        
        let updated_user = query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1, role = $2, is_active = $3, updated_at = $4
            WHERE id = $5
            RETURNING id, username, email, password_hash, role as "role: _", is_active, created_at, updated_at
            "#,
            email,
            role as _,
            is_active,
            now,
            id
        )
        .fetch_one(self.db.pool())
        .await?;
        
        Ok(updated_user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        // Make sure user exists
        let _ = self.get_user(id).await?;
        
        // Delete user
        query!("DELETE FROM users WHERE id = $1", id)
            .execute(self.db.pool())
            .await?;
            
        Ok(())
    }
}