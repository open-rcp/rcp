use crate::error::{Error, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// User role types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
    /// Administrator with full access
    Admin,

    /// Regular user with limited access
    User,

    /// Guest user with restricted access
    Guest,
}

impl UserRole {
    /// Convert a string to a UserRole
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            "guest" => Ok(UserRole::Guest),
            _ => Err(Error::InvalidArgument(format!("Invalid user role: {}", s))),
        }
    }

    /// Convert a UserRole to a string
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        }
    }
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique ID
    pub id: Uuid,

    /// Username
    pub username: String,

    /// Password hash
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// User role
    pub role: UserRole,

    /// Account creation time
    pub created_at: DateTime<Utc>,

    /// Last login time
    pub last_login: Option<DateTime<Utc>>,
}

/// Public user information without sensitive data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Unique ID
    pub id: String,

    /// Username
    pub username: String,

    /// User role
    pub role: String,

    /// Account creation time
    pub created_at: DateTime<Utc>,

    /// Last login time
    pub last_login: Option<DateTime<Utc>>,
}

/// User manager for handling user operations
#[derive(Debug, Clone)]
pub struct UserManager {
    /// Users map (username -> user)
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl UserManager {
    /// Create a new user manager
    pub fn new() -> Self {
        // Create the default admin user
        let mut users = HashMap::new();

        // Initialize with a default admin account if environment specifies
        if let Ok(admin_pass) = std::env::var("RCP_ADMIN_PASSWORD") {
            if !admin_pass.is_empty() {
                // Generate a hash for the admin password
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();
                if let Ok(hash) = argon2.hash_password(admin_pass.as_bytes(), &salt) {
                    let admin_user = User {
                        id: Uuid::new_v4(),
                        username: "admin".to_string(),
                        password_hash: hash.to_string(),
                        role: UserRole::Admin,
                        created_at: Utc::now(),
                        last_login: None,
                    };
                    users.insert("admin".to_string(), admin_user);
                    info!("Created default admin user");
                }
            }
        }

        Self {
            users: Arc::new(RwLock::new(users)),
        }
    }

    /// Add a new user
    pub async fn add_user(
        &self,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<User> {
        // Check if username is valid (alphanumeric, at least 3 chars)
        if username.len() < 3 || !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(Error::InvalidArgument(
                "Username must be at least 3 characters and contain only alphanumeric characters and underscores"
                    .to_string(),
            ));
        }

        // Check if password is strong enough (at least 8 chars)
        if password.len() < 8 {
            return Err(Error::InvalidArgument(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        // Check if user already exists
        let mut users = self.users.write().await;
        if users.contains_key(&username) {
            return Err(Error::AlreadyExists(format!(
                "User '{}' already exists",
                username
            )));
        }

        // Hash the password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| Error::Internal(format!("Failed to hash password: {}", e)))?;

        // Create the new user
        let user = User {
            id: Uuid::new_v4(),
            username: username.clone(),
            password_hash: hash.to_string(),
            role,
            created_at: Utc::now(),
            last_login: None,
        };

        // Add to the map
        users.insert(username, user.clone());
        info!(
            "Added new user: {} with role: {:?}",
            user.username, user.role
        );

        Ok(user)
    }

    /// Get a user by username
    pub async fn get_user(&self, username: &str) -> Result<User> {
        let users = self.users.read().await;
        users
            .get(username)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("User '{}' not found", username)))
    }

    /// List all users
    pub async fn list_users(&self) -> Vec<UserInfo> {
        let users = self.users.read().await;
        users
            .values()
            .map(|u| UserInfo {
                id: u.id.to_string(),
                username: u.username.clone(),
                role: u.role.as_str().to_string(),
                created_at: u.created_at,
                last_login: u.last_login,
            })
            .collect()
    }

    /// Authenticate a user with username and password
    pub async fn authenticate(&self, username: &str, password: &str) -> Result<User> {
        // Get the user
        let mut users = self.users.write().await;
        let user = users
            .get(username)
            .cloned()
            .ok_or_else(|| Error::Authentication("Invalid username or password".to_string()))?;

        // Verify the password
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| Error::Internal(format!("Failed to parse password hash: {}", e)))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::Authentication("Invalid username or password".to_string()))?;

        // Update last login time
        if let Some(user) = users.get_mut(username) {
            user.last_login = Some(Utc::now());
        }

        debug!("User authenticated: {}", username);
        Ok(user)
    }

    /// Delete a user
    #[allow(dead_code)]
    pub async fn delete_user(&self, username: &str) -> Result<()> {
        let mut users = self.users.write().await;

        // Make sure at least one admin remains
        if username == "admin" {
            let admin_count = users.values().filter(|u| u.role == UserRole::Admin).count();
            if admin_count <= 1 {
                return Err(Error::InvalidOperation(
                    "Cannot delete the last admin user".to_string(),
                ));
            }
        }

        // Remove the user
        users
            .remove(username)
            .ok_or_else(|| Error::NotFound(format!("User '{}' not found", username)))?;

        info!("Deleted user: {}", username);
        Ok(())
    }

    /// Update a user's role
    #[allow(dead_code)]
    pub async fn update_user_role(&self, username: &str, role: UserRole) -> Result<User> {
        let mut users = self.users.write().await;

        // Check if user exists
        let user = users
            .get_mut(username)
            .ok_or_else(|| Error::NotFound(format!("User '{}' not found", username)))?;

        // Update the role
        let role_clone = role.clone();
        user.role = role;
        info!("Updated role for user {}: {:?}", username, role_clone);

        Ok(user.clone())
    }

    /// Change a user's password
    #[allow(dead_code)]
    pub async fn change_password(
        &self,
        username: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<()> {
        // First authenticate to verify current password
        self.authenticate(username, current_password).await?;

        // Check if new password is strong enough
        if new_password.len() < 8 {
            return Err(Error::InvalidArgument(
                "New password must be at least 8 characters".to_string(),
            ));
        }

        // Generate new hash
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| Error::Internal(format!("Failed to hash password: {}", e)))?;

        // Update the password hash
        let mut users = self.users.write().await;
        if let Some(user) = users.get_mut(username) {
            user.password_hash = hash.to_string();
            info!("Changed password for user: {}", username);
            Ok(())
        } else {
            Err(Error::NotFound(format!("User '{}' not found", username)))
        }
    }

    /// Reset a user's password (admin operation)
    #[allow(dead_code)]
    pub async fn reset_password(&self, username: &str, new_password: &str) -> Result<()> {
        // Check if new password is strong enough
        if new_password.len() < 8 {
            return Err(Error::InvalidArgument(
                "New password must be at least 8 characters".to_string(),
            ));
        }

        // Generate new hash
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| Error::Internal(format!("Failed to hash password: {}", e)))?;

        // Update the password hash
        let mut users = self.users.write().await;
        if let Some(user) = users.get_mut(username) {
            user.password_hash = hash.to_string();
            info!("Reset password for user: {}", username);
            Ok(())
        } else {
            Err(Error::NotFound(format!("User '{}' not found", username)))
        }
    }
}

// Implement Default for UserManager to make initialization easier
impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_creation_and_authentication() {
        let manager = UserManager::new();

        // Add a test user
        let user = manager
            .add_user(
                "testuser".to_string(),
                "password123".to_string(),
                UserRole::User,
            )
            .await
            .unwrap();

        assert_eq!(user.username, "testuser");
        assert_eq!(user.role, UserRole::User);

        // Authenticate the user
        let auth_result = manager.authenticate("testuser", "password123").await;
        assert!(auth_result.is_ok());

        // Try incorrect password
        let auth_result = manager.authenticate("testuser", "wrongpassword").await;
        assert!(auth_result.is_err());
    }

    #[tokio::test]
    async fn test_role_conversion() {
        assert_eq!(UserRole::from_str("admin").unwrap(), UserRole::Admin);
        assert_eq!(UserRole::from_str("user").unwrap(), UserRole::User);
        assert_eq!(UserRole::from_str("guest").unwrap(), UserRole::Guest);

        assert!(UserRole::from_str("invalid").is_err());

        assert_eq!(UserRole::Admin.as_str(), "admin");
        assert_eq!(UserRole::User.as_str(), "user");
        assert_eq!(UserRole::Guest.as_str(), "guest");
    }
}
