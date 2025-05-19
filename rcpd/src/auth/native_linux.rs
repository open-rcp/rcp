use crate::auth::provider::AuthProvider;
use crate::server::user::{User, UserRole};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

/// Configuration for the Linux native auth provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxAuthConfig {
    /// Whether to allow all Linux users to authenticate
    pub allow_all_users: bool,

    /// Required group for RCP access (if not allowing all users)
    pub require_group: Option<String>,

    /// Whether to map OS groups to RCP permissions
    pub permission_mapping: bool,

    /// Groups that have admin privileges
    pub admin_groups: Vec<String>,

    /// Custom permission mappings (group -> permission)
    pub permission_mappings: HashMap<String, Vec<String>>,
}

impl Default for LinuxAuthConfig {
    fn default() -> Self {
        Self {
            allow_all_users: false,
            require_group: Some("rcp-users".to_string()),
            permission_mapping: true,
            admin_groups: vec!["sudo".to_string(), "wheel".to_string(), "admin".to_string()],
            permission_mappings: HashMap::new(),
        }
    }
}

/// Linux native authentication provider
pub struct LinuxAuthProvider {
    /// Configuration for this provider
    config: LinuxAuthConfig,

    /// Cache of user information
    user_cache: HashMap<String, User>,

    /// Cache of group memberships
    group_cache: HashMap<String, Vec<String>>,
}

impl LinuxAuthProvider {
    /// Create a new Linux authentication provider
    pub fn new(config: LinuxAuthConfig) -> Self {
        Self {
            config,
            user_cache: HashMap::new(),
            group_cache: HashMap::new(),
        }
    }

    /// Check if a user is a member of a group
    fn is_member_of_group(&self, username: &str, group: &str) -> Result<bool> {
        // Use getent to check group membership
        let output = Command::new("getent").args(&["group", group]).output()?;

        if !output.status.success() {
            return Ok(false);
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let members = output_str.split(':').nth(3).unwrap_or("");

        Ok(members.split(',').any(|m| m.trim() == username))
    }

    /// Get all groups a user belongs to
    fn get_user_groups(&self, username: &str) -> Result<Vec<String>> {
        // Check if cached
        if let Some(groups) = self.group_cache.get(username) {
            return Ok(groups.clone());
        }

        // Use groups command to get all groups
        let output = Command::new("groups").arg(username).output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to list groups for user: {}", username));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut groups = Vec::new();

        // Parse output: "username : group1 group2 group3"
        if let Some(groups_part) = output_str.split(':').nth(1) {
            for group in groups_part.split_whitespace() {
                groups.push(group.to_string());
            }
        }

        Ok(groups)
    }

    /// Map OS groups to RCP permissions
    fn map_permissions(&self, groups: &[String]) -> Vec<String> {
        let mut permissions = Vec::new();

        // Check for admin groups
        let is_admin = groups.iter().any(|g| self.config.admin_groups.contains(g));
        if is_admin {
            permissions.push("admin:*".to_string());
        }

        // Basic connect permission if they got this far
        permissions.push("connect:*".to_string());

        // Check for app-specific groups
        for group in groups {
            if group.starts_with("rcp-app-") {
                let app = group.trim_start_matches("rcp-app-");
                permissions.push(format!("app:{}", app));
            }

            if group == "rcp-api-users" {
                permissions.push("api:read".to_string());
            }

            if group == "rcp-api-admins" {
                permissions.push("api:write".to_string());
            }

            // Add custom mappings
            if let Some(custom_perms) = self.config.permission_mappings.get(group) {
                permissions.extend(custom_perms.clone());
            }
        }

        permissions
    }

    /// Validate credentials using PAM
    fn validate_system_credentials(&self, username: &str, _password: &[u8]) -> Result<bool> {
        // This is a simplified version for demonstration
        // In a real implementation, you would use PAM for authentication

        // For now, just check if the user exists
        let output = Command::new("id").arg(username).output()?;

        Ok(output.status.success())
    }
}

#[async_trait]
impl AuthProvider for LinuxAuthProvider {
    async fn initialize(&mut self) -> Result<()> {
        info!("Initializing Linux native authentication provider");

        // Clear caches
        self.user_cache.clear();
        self.group_cache.clear();

        Ok(())
    }

    async fn validate_credentials(
        &self,
        username: &str,
        credentials: &[u8],
        method: &str,
    ) -> Result<bool> {
        match method {
            "psk" => {
                // For PSK, we just check if the user exists and is allowed
                if !self.config.allow_all_users {
                    if let Some(ref required_group) = self.config.require_group {
                        return Ok(self.is_member_of_group(username, required_group)?);
                    }
                }

                // Check if user exists
                let output = Command::new("id").arg(username).output()?;

                Ok(output.status.success())
            }
            "password" => {
                // Validate system credentials
                self.validate_system_credentials(username, credentials)
            }
            "publickey" => {
                // Public key auth could be implemented by checking ~/.ssh/authorized_keys
                warn!("Public key authentication not fully implemented for Linux");
                Ok(false)
            }
            _ => Err(anyhow!("Unsupported authentication method: {}", method)),
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        // Check if cached
        if let Some(user) = self.user_cache.get(username) {
            return Ok(Some(user.clone()));
        }

        // Check if user exists
        let output = Command::new("id").arg(username).output()?;

        if !output.status.success() {
            return Ok(None);
        }

        // Get user information
        let gecos_output = Command::new("getent")
            .args(&["passwd", username])
            .output()?;

        let display_name = if gecos_output.status.success() {
            let output_str = String::from_utf8_lossy(&gecos_output.stdout);
            let parts: Vec<&str> = output_str.split(':').collect();
            if parts.len() >= 5 {
                let gecos = parts[4];
                let name_part = gecos.split(',').next().unwrap_or("");
                if !name_part.is_empty() {
                    name_part.to_string()
                } else {
                    username.to_string()
                }
            } else {
                username.to_string()
            }
        } else {
            username.to_string()
        };

        // Get user's groups
        let groups = self.get_user_groups(username)?;

        // Determine role based on group membership
        let role = if groups.iter().any(|g| self.config.admin_groups.contains(g)) {
            UserRole::Admin
        } else {
            UserRole::User
        };

        // Create user object
        let user = User {
            id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, username.as_bytes()),
            username: username.to_string(),
            display_name,
            email: None,
            role,
            password_hash: "".to_string(), // We don't store passwords
            enabled: true,
            created_at: 0, // Not tracked
            last_login: 0, // Not tracked
        };

        Ok(Some(user))
    }

    async fn get_user(&self, id: &Uuid) -> Result<Option<User>> {
        // Since we generate UUIDs based on usernames, we can't easily
        // look up by UUID without listing all users. For efficiency,
        // we'll return None and let the caller use get_user_by_username instead.
        warn!("Looking up Linux users by UUID is not efficient");

        // In a real implementation, maintain a reverse lookup cache
        Ok(None)
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        // Get all users from /etc/passwd
        let output = Command::new("getent").arg("passwd").output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to list users"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut users = Vec::new();

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let username = parts[0];
                let uid: i32 = parts[2].parse().unwrap_or(0);

                // Skip system users (typically UID < 1000)
                if uid >= 1000 {
                    if let Ok(Some(user)) = self.get_user_by_username(username).await {
                        users.push(user);
                    }
                }
            }
        }

        Ok(users)
    }

    async fn create_user(&self, _user: User) -> Result<()> {
        Err(anyhow!(
            "User creation not supported by Linux native provider"
        ))
    }

    async fn update_user(&self, _user: User) -> Result<()> {
        Err(anyhow!(
            "User updates not supported by Linux native provider"
        ))
    }

    async fn delete_user(&self, _id: &Uuid) -> Result<()> {
        Err(anyhow!(
            "User deletion not supported by Linux native provider"
        ))
    }

    async fn has_permission(&self, user: &User, permission: &str) -> Result<bool> {
        // Get user's groups
        let groups = self.get_user_groups(&user.username)?;

        // Map groups to permissions
        let permissions = self.map_permissions(&groups);

        // Check for wildcard permissions
        for perm in &permissions {
            if perm.ends_with(":*") {
                let prefix = perm.trim_end_matches(":*");
                if permission.starts_with(prefix) {
                    return Ok(true);
                }
            }

            if perm == permission {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn get_permissions(&self, user: &User) -> Result<Vec<String>> {
        let groups = self.get_user_groups(&user.username)?;
        Ok(self.map_permissions(&groups))
    }

    fn supports_user_management(&self) -> bool {
        false // Linux native provider doesn't support user management through RCP
    }

    fn supports_auth_method(&self, method: &str) -> bool {
        matches!(method, "psk" | "password")
    }

    fn name(&self) -> &str {
        "linux-native"
    }
}
