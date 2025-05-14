use crate::server::error::{Error, Result};
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

impl std::str::FromStr for UserRole {
    type Err = Error;

    /// Convert a string to a UserRole
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            "guest" => Ok(UserRole::Guest),
            _ => Err(Error::InvalidArgument(format!("Invalid user role: {}", s))),
        }
    }
}

impl UserRole {
    /// Convert a UserRole to a string
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        }
    }
}

/// A user in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user ID
    pub id: Uuid,

    /// Username
    pub username: String,

    /// User's full name (optional)
    pub full_name: Option<String>,

    /// User's email (optional)
    pub email: Option<String>,

    /// Password hash
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// User role
    pub role: UserRole,

    /// When the user was created
    pub created_at: String,

    /// When the user was last updated
    pub updated_at: String,
}

/// Manager for user operations
pub struct UserManager {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl UserManager {
    /// Create a new user manager
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a user by ID
    pub async fn get_user(&self, id: &Uuid) -> Option<User> {
        let users = self.users.read().await;
        users.get(id).cloned()
    }

    /// Get a user by username
    pub async fn get_user_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.read().await;
        for user in users.values() {
            if user.username == username {
                return Some(user.clone());
            }
        }
        None
    }

    /// Add a new user
    pub async fn add_user(&self, user: User) -> Result<()> {
        let mut users = self.users.write().await;

        // Check if username already exists
        for existing in users.values() {
            if existing.username == user.username {
                return Err(Error::AlreadyExists(format!(
                    "User with username '{}' already exists",
                    user.username
                )));
            }
        }

        users.insert(user.id, user);
        Ok(())
    }

    /// Update a user
    pub async fn update_user(&self, user: User) -> Result<()> {
        let mut users = self.users.write().await;

        // Check if user exists
        if !users.contains_key(&user.id) {
            return Err(Error::NotFound(format!(
                "User with ID '{}' not found",
                user.id
            )));
        }

        users.insert(user.id, user);
        Ok(())
    }

    /// Remove a user
    pub async fn remove_user(&self, id: &Uuid) -> Result<()> {
        let mut users = self.users.write().await;

        if users.remove(id).is_none() {
            return Err(Error::NotFound(format!("User with ID '{}' not found", id)));
        }

        Ok(())
    }

    /// List all users
    pub async fn list_users(&self) -> Vec<User> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}
