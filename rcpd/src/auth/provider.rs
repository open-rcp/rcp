use crate::server::user::{User, UserRole};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use uuid::Uuid;

/// Authentication provider interface for RCP
///
/// This trait defines the contract that all authentication providers must fulfill.
/// Implementations can use internal user databases, OS-native authentication,
/// or external identity providers.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Initialize the authentication provider
    async fn initialize(&mut self) -> Result<()>;

    /// Validate credentials for a user
    async fn validate_credentials(
        &self,
        username: &str,
        credentials: &[u8],
        method: &str,
    ) -> Result<bool>;

    /// Get a user by their username
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Get a user by their ID
    async fn get_user(&self, id: &Uuid) -> Result<Option<User>>;

    /// List all users
    async fn list_users(&self) -> Result<Vec<User>>;

    /// Create a new user (if supported by the provider)
    async fn create_user(&self, user: User) -> Result<()>;

    /// Update an existing user (if supported by the provider)
    async fn update_user(&self, user: User) -> Result<()>;

    /// Delete a user (if supported by the provider)
    async fn delete_user(&self, id: &Uuid) -> Result<()>;

    /// Check if a user has the specified permission
    async fn has_permission(&self, user: &User, permission: &str) -> Result<bool>;

    /// Get all permissions for a user
    async fn get_permissions(&self, user: &User) -> Result<Vec<String>>;

    /// Check if the provider supports user management operations
    fn supports_user_management(&self) -> bool;

    /// Check if the provider supports a specific authentication method
    fn supports_auth_method(&self, method: &str) -> bool;

    /// Get the name of the provider
    fn name(&self) -> &str;
}
