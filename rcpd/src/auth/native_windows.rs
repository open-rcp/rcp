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

/// Configuration for the Windows native auth provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsAuthConfig {
    /// Whether to allow all Windows users to authenticate
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

impl Default for WindowsAuthConfig {
    fn default() -> Self {
        Self {
            allow_all_users: false,
            require_group: Some("RCP Users".to_string()),
            permission_mapping: true,
            admin_groups: vec!["Administrators".to_string()],
            permission_mappings: HashMap::new(),
        }
    }
}

/// Windows native authentication provider
pub struct WindowsAuthProvider {
    /// Configuration for this provider
    config: WindowsAuthConfig,

    /// Cache of user information
    user_cache: HashMap<String, User>,

    /// Cache of group memberships
    group_cache: HashMap<String, Vec<String>>,
}

impl WindowsAuthProvider {
    /// Create a new Windows authentication provider
    pub fn new(config: WindowsAuthConfig) -> Self {
        Self {
            config,
            user_cache: HashMap::new(),
            group_cache: HashMap::new(),
        }
    }

    /// Check if a user is a member of a group using Windows commands
    fn is_member_of_group(&self, username: &str, group: &str) -> Result<bool> {
        // Use net user to check group membership
        let output = Command::new("net").args(&["user", username]).output()?;

        if !output.status.success() {
            return Ok(false);
        }

        // Parse the output to find group memberships
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Check if the group name appears in the "Local Group Memberships" section
        let group_section = output_str.find("Local Group Memberships");
        if let Some(idx) = group_section {
            let remaining = &output_str[idx..];
            // Find the end of the group list
            let end_idx = remaining.find("*").unwrap_or(remaining.len());
            let groups_text = &remaining[..end_idx];

            // Look for the group name
            return Ok(groups_text.contains(group));
        }

        Ok(false)
    }

    /// Get all groups a user belongs to
    fn get_user_groups(&self, username: &str) -> Result<Vec<String>> {
        // Check if cached
        if let Some(groups) = self.group_cache.get(username) {
            return Ok(groups.clone());
        }

        // Use net user to get all groups
        let output = Command::new("net").args(&["user", username]).output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to get groups for user: {}", username));
        }

        // Parse the output to find group memberships
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut groups = Vec::new();

        // Check if the group name appears in the "Local Group Memberships" section
        let group_section = output_str.find("Local Group Memberships");
        if let Some(idx) = group_section {
            let remaining = &output_str[idx..];
            // Find the end of the group list
            let end_idx = remaining.find("*").unwrap_or(remaining.len());
            let groups_text = &remaining[..end_idx];

            // Split by spaces and asterisks
            for line in groups_text.lines().skip(1) {
                // Skip header line
                for group in line.split_whitespace() {
                    if !group.trim().is_empty() && group.trim() != "*" {
                        groups.push(group.trim().to_string());
                    }
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
            if group.starts_with("RCP-App-") {
                let app = group.trim_start_matches("RCP-App-");
                permissions.push(format!("app:{}", app));
            }

            if group == "RCP-API-Users" {
                permissions.push("api:read".to_string());
            }

            if group == "RCP-API-Admins" {
                permissions.push("api:write".to_string());
            }

            // Add custom mappings
            if let Some(custom_perms) = self.config.permission_mappings.get(group) {
                permissions.extend(custom_perms.clone());
            }
        }

        permissions
    }

    /// Validate credentials using Windows authentication
    fn validate_system_credentials(&self, username: &str, _password: &[u8]) -> Result<bool> {
        // This is a simplified version - in a real implementation,
        // you would use Windows authentication APIs (LogonUser, etc.)

        // For now, we'll just check if the user exists
        let output = Command::new("net").args(&["user", username]).output()?;

        Ok(output.status.success())
    }
}

#[async_trait]
impl AuthProvider for WindowsAuthProvider {
    async fn initialize(&mut self) -> Result<()> {
        info!("Initializing Windows native authentication provider");

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
                self.validate_system_credentials(username, &[])
            }
            "password" => {
                // Validate system credentials
                self.validate_system_credentials(username, credentials)
            }
            "publickey" => {
                // Not implemented for Windows yet
                warn!("Public key authentication not implemented for Windows");
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
        let output = Command::new("net").args(&["user", username]).output()?;

        if !output.status.success() {
            return Ok(None);
        }

        // Get user information from output
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Extract full name from output
        let full_name = if let Some(idx) = output_str.find("Full Name") {
            let line = output_str[idx..].lines().next().unwrap_or("");
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                parts[2..].join(" ")
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
            display_name: full_name,
            email: None, // Windows doesn't have email in user DB by default
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
        warn!("Looking up Windows users by UUID is not efficient");

        // In a real implementation, maintain a reverse lookup cache
        Ok(None)
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        // Get all users using Windows commands
        let output = Command::new("net").args(&["user"]).output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to list users"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut users = Vec::new();

        // Skip the header lines and process each username
        for line in output_str.lines().skip(4) {
            // Stop at the footer
            if line.trim().is_empty() || line.contains("The command completed") {
                break;
            }

            // Extract usernames from line
            for username in line.split_whitespace() {
                if !username.is_empty() && username != "Name" {
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
            "User creation not supported by Windows native provider"
        ))
    }

    async fn update_user(&self, _user: User) -> Result<()> {
        Err(anyhow!(
            "User updates not supported by Windows native provider"
        ))
    }

    async fn delete_user(&self, _id: &Uuid) -> Result<()> {
        Err(anyhow!(
            "User deletion not supported by Windows native provider"
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
        false // Windows native provider doesn't support user management through RCP
    }

    fn supports_auth_method(&self, method: &str) -> bool {
        matches!(method, "psk" | "password")
    }

    fn name(&self) -> &str {
        "windows-native"
    }
}
