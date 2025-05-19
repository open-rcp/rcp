use crate::auth::provider::AuthProvider;
use crate::server::user::{User, UserRole};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

/// Mock authentication provider for testing
pub struct MockAuthProvider {
    /// Users in the system
    users: HashMap<String, User>,

    /// Credentials (username -> password)
    credentials: HashMap<String, Vec<u8>>,

    /// Permissions (username -> permissions)
    permissions: HashMap<String, Vec<String>>,

    /// Whether initialization was called
    initialized: bool,
}

impl MockAuthProvider {
    /// Create a new mock provider with default settings
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            credentials: HashMap::new(),
            permissions: HashMap::new(),
            initialized: false,
        }
    }

    /// Add a user to the mock provider
    pub fn with_user(mut self, user: User) -> Self {
        self.users.insert(user.username.clone(), user);
        self
    }

    /// Add credentials for a user
    pub fn with_credential(mut self, username: &str, password: &[u8]) -> Self {
        self.credentials
            .insert(username.to_string(), password.to_vec());
        self
    }

    /// Add a permission for a user
    pub fn with_permission(mut self, username: &str, permission: &str) -> Self {
        let entry = self
            .permissions
            .entry(username.to_string())
            .or_insert_with(Vec::new);
        entry.push(permission.to_string());
        self
    }
}

#[async_trait]
impl AuthProvider for MockAuthProvider {
    async fn initialize(&mut self) -> Result<()> {
        self.initialized = true;
        Ok(())
    }

    async fn validate_credentials(
        &self,
        username: &str,
        credentials: &[u8],
        method: &str,
    ) -> Result<bool> {
        match method {
            "password" => {
                if let Some(stored_creds) = self.credentials.get(username) {
                    Ok(stored_creds == credentials)
                } else {
                    Ok(false)
                }
            }
            "psk" => {
                // For PSK, we just check if the user exists
                Ok(self.users.contains_key(username))
            }
            _ => Ok(false),
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        Ok(self.users.get(username).cloned())
    }

    async fn get_user(&self, id: &Uuid) -> Result<Option<User>> {
        for user in self.users.values() {
            if &user.id == id {
                return Ok(Some(user.clone()));
            }
        }
        Ok(None)
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        Ok(self.users.values().cloned().collect())
    }

    async fn create_user(&self, _user: User) -> Result<()> {
        Ok(())
    }

    async fn update_user(&self, _user: User) -> Result<()> {
        Ok(())
    }

    async fn delete_user(&self, _id: &Uuid) -> Result<()> {
        Ok(())
    }

    async fn has_permission(&self, user: &User, permission: &str) -> Result<bool> {
        if let Some(perms) = self.permissions.get(&user.username) {
            // Check for exact permission match
            if perms.contains(&permission.to_string()) {
                return Ok(true);
            }

            // Check for wildcard permissions
            for perm in perms {
                if perm.ends_with(":*") {
                    let prefix = perm.trim_end_matches(":*");
                    if permission.starts_with(prefix) {
                        return Ok(true);
                    }
                }
            }
        }

        // Admin users have all permissions
        if user.role == UserRole::Admin {
            return Ok(true);
        }

        Ok(false)
    }

    async fn get_permissions(&self, user: &User) -> Result<Vec<String>> {
        if let Some(perms) = self.permissions.get(&user.username) {
            Ok(perms.clone())
        } else {
            Ok(Vec::new())
        }
    }

    fn supports_user_management(&self) -> bool {
        true
    }

    fn supports_auth_method(&self, method: &str) -> bool {
        matches!(method, "password" | "psk")
    }

    fn name(&self) -> &str {
        "mock-provider"
    }
}
