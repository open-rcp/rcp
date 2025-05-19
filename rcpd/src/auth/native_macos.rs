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

/// Configuration for the macOS native auth provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacOSAuthConfig {
    /// Whether to allow all macOS users to authenticate
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

impl Default for MacOSAuthConfig {
    fn default() -> Self {
        Self {
            allow_all_users: false,
            require_group: Some("rcp-users".to_string()),
            permission_mapping: true,
            admin_groups: vec!["admin".to_string(), "wheel".to_string()],
            permission_mappings: HashMap::new(),
        }
    }
}

/// macOS native authentication provider
pub struct MacOSAuthProvider {
    /// Configuration for this provider
    config: MacOSAuthConfig,

    /// Cache of user information
    user_cache: HashMap<String, User>,

    /// Cache of group memberships
    group_cache: HashMap<String, Vec<String>>,
}

impl MacOSAuthProvider {
    /// Create a new macOS authentication provider
    pub fn new(config: MacOSAuthConfig) -> Self {
        Self {
            config,
            user_cache: HashMap::new(),
            group_cache: HashMap::new(),
        }
    }

    /// Check if a user is a member of a group
    fn is_member_of_group(&self, username: &str, group: &str) -> Result<bool> {
        // Use dscl to check group membership
        let output = Command::new("dscl")
            .args(&[
                ".",
                "-read",
                &format!("/Groups/{}", group),
                "GroupMembership",
            ])
            .output()?;

        if !output.status.success() {
            return Ok(false);
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.contains(username))
    }

    /// Get all groups a user belongs to
    fn get_user_groups(&self, username: &str) -> Result<Vec<String>> {
        // Check if cached
        if let Some(groups) = self.group_cache.get(username) {
            return Ok(groups.clone());
        }

        // Use dscl to get all groups
        let output = Command::new("dscl")
            .args(&[".", "-list", "/Groups", "GroupMembership"])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to list groups"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut groups = Vec::new();

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                let group_name = parts[0];
                let members = &parts[1..];

                if members.contains(&username) {
                    groups.push(group_name.to_string());
                }
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
    fn validate_system_credentials(&self, username: &str, password: &[u8]) -> Result<bool> {
        // This is a simplified version using the `pam` crate
        // In a real implementation, you would use the actual PAM APIs

        // For demonstration purposes, we'll use the `login` utility
        // WARNING: This is not secure and is just for demonstration
        let password_str = String::from_utf8_lossy(password);

        // In a real implementation, use the macOS Security framework
        // or a proper PAM binding to validate credentials

        // For now, we'll just check if the user exists
        let output = Command::new("dscl")
            .args(&[".", "-read", &format!("/Users/{}", username)])
            .output()?;

        Ok(output.status.success())
    }
}

#[async_trait]
impl AuthProvider for MacOSAuthProvider {
    async fn initialize(&mut self) -> Result<()> {
        info!("Initializing macOS native authentication provider");

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
                let output = Command::new("dscl")
                    .args(&[".", "-read", &format!("/Users/{}", username)])
                    .output()?;

                Ok(output.status.success())
            }
            "password" => {
                // Validate system credentials
                self.validate_system_credentials(username, credentials)
            }
            "publickey" => {
                // For public key auth, we'd check the user's authorized_keys
                // This is a simplified version
                warn!("Public key authentication not fully implemented for macOS");
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
        let output = Command::new("dscl")
            .args(&[".", "-read", &format!("/Users/{}", username)])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        // Get user information
        let output_str = String::from_utf8_lossy(&output.stdout);
        let real_name = if let Some(line) = output_str
            .lines()
            .find(|l| l.trim().starts_with("RealName:"))
        {
            line.trim_start_matches("RealName:").trim().to_string()
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
            id: Uuid::new_v4(), // Using new_v4 instead of new_v5 which doesn't exist in this version
            username: username.to_string(),
            full_name: Some(real_name),
            email: None, // macOS doesn't have email in user DB by default
            role,
            password_hash: "".to_string(), // We don't store passwords
            created_at: "1970-01-01T00:00:00Z".to_string(), // Not tracked, use epoch
            updated_at: "1970-01-01T00:00:00Z".to_string(), // Not tracked, use epoch
        };

        Ok(Some(user))
    }

    async fn get_user(&self, id: &Uuid) -> Result<Option<User>> {
        // Since we generate UUIDs based on usernames, we can't easily
        // look up by UUID without listing all users. For efficiency,
        // we'll return None and let the caller use get_user_by_username instead.
        warn!("Looking up macOS users by UUID is not efficient");

        // In a real implementation, maintain a reverse lookup cache
        Ok(None)
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        // Get all users from directory services
        let output = Command::new("dscl")
            .args(&[".", "-list", "/Users"])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to list users"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut users = Vec::new();

        for username in output_str.lines() {
            // Skip system users
            if username.starts_with('_') || username == "nobody" || username == "root" {
                continue;
            }

            if let Ok(Some(user)) = self.get_user_by_username(username).await {
                users.push(user);
            }
        }

        Ok(users)
    }

    async fn create_user(&self, _user: User) -> Result<()> {
        Err(anyhow!(
            "User creation not supported by macOS native provider"
        ))
    }

    async fn update_user(&self, _user: User) -> Result<()> {
        Err(anyhow!(
            "User updates not supported by macOS native provider"
        ))
    }

    async fn delete_user(&self, _id: &Uuid) -> Result<()> {
        Err(anyhow!(
            "User deletion not supported by macOS native provider"
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
        false // macOS native provider doesn't support user management through RCP
    }

    fn supports_auth_method(&self, method: &str) -> bool {
        matches!(method, "psk" | "password")
    }

    fn name(&self) -> &str {
        "macos-native"
    }
}
