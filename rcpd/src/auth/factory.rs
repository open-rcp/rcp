use crate::auth::mock_provider::MockAuthProvider;
use crate::auth::provider::AuthProvider;
use anyhow::{anyhow, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Platform-specific imports
#[cfg(target_os = "linux")]
use crate::auth::native_linux::LinuxAuthProvider;
#[cfg(target_os = "macos")]
use crate::auth::native_macos::MacOSAuthProvider;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
use crate::auth::native_unix::UnixAuthProvider;
#[cfg(target_os = "windows")]
use crate::auth::native_windows::WindowsAuthProvider;

/// Authentication provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthProviderType {
    /// Internal user database (default)
    Internal,

    /// Native OS authentication
    Native,

    /// LDAP-based authentication
    Ldap,

    /// OAuth-based authentication
    OAuth,

    /// Mock provider for testing
    #[serde(rename = "mock")]
    Mock,
}

impl Default for AuthProviderType {
    fn default() -> Self {
        Self::Internal
    }
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication provider type
    #[serde(default)]
    pub provider: AuthProviderType,

    /// Whether authentication is required
    #[serde(default = "default_true")]
    pub required: bool,

    /// Pre-shared key for simple authentication
    pub psk: Option<String>,

    /// Whether to fall back to internal authentication if native fails
    #[serde(default)]
    pub fallback_to_internal: bool,

    /// Native authentication configuration
    #[serde(default)]
    pub native: NativeAuthConfig,

    /// LDAP authentication configuration (not implemented in this example)
    #[serde(default)]
    pub ldap: HashMap<String, String>,

    /// OAuth authentication configuration (not implemented in this example)
    #[serde(default)]
    pub oauth: HashMap<String, String>,
}

fn default_true() -> bool {
    true
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            provider: AuthProviderType::Internal,
            required: true,
            psk: None,
            fallback_to_internal: false,
            native: NativeAuthConfig::default(),
            ldap: HashMap::new(),
            oauth: HashMap::new(),
        }
    }
}

/// Native authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeAuthConfig {
    /// Whether to allow all OS users
    #[serde(default)]
    pub allow_all_users: bool,

    /// Required OS group for RCP access
    pub require_group: Option<String>,

    /// Whether to map OS groups to RCP permissions
    #[serde(default = "default_true")]
    pub permission_mapping: bool,

    /// OS groups with admin privileges
    #[serde(default = "default_admin_groups")]
    pub admin_groups: Vec<String>,

    /// Custom permission mappings
    #[serde(default)]
    pub permission_mappings: HashMap<String, Vec<String>>,
}

fn default_admin_groups() -> Vec<String> {
    vec![
        "administrators".to_string(),
        "wheel".to_string(),
        "sudo".to_string(),
        "admin".to_string(),
    ]
}

impl Default for NativeAuthConfig {
    fn default() -> Self {
        Self {
            allow_all_users: false,
            require_group: Some("rcp-users".to_string()),
            permission_mapping: true,
            admin_groups: default_admin_groups(),
            permission_mappings: HashMap::new(),
        }
    }
}

/// Authentication provider factory
pub struct AuthProviderFactory;

impl AuthProviderFactory {
    /// Create a new authentication provider based on configuration
    pub fn create_provider(config: &AuthConfig) -> Result<Box<dyn AuthProvider>> {
        match config.provider {
            AuthProviderType::Internal => {
                info!("Using internal authentication provider");
                Err(anyhow!("Internal provider not implemented in this example"))
            }
            AuthProviderType::Native => {
                info!("Using native OS authentication provider");

                #[cfg(target_os = "macos")]
                {
                    use crate::auth::native_macos::MacOSAuthConfig;

                    let macos_config = MacOSAuthConfig {
                        allow_all_users: config.native.allow_all_users,
                        require_group: config.native.require_group.clone(),
                        permission_mapping: config.native.permission_mapping,
                        admin_groups: config.native.admin_groups.clone(),
                        permission_mappings: config.native.permission_mappings.clone(),
                    };

                    Ok(Box::new(MacOSAuthProvider::new(macos_config)))
                }

                #[cfg(target_os = "windows")]
                {
                    use crate::auth::native_windows::WindowsAuthConfig;

                    let windows_config = WindowsAuthConfig {
                        allow_all_users: config.native.allow_all_users,
                        require_group: config.native.require_group.clone(),
                        permission_mapping: config.native.permission_mapping,
                        admin_groups: config.native.admin_groups.clone(),
                        permission_mappings: config.native.permission_mappings.clone(),
                    };

                    Ok(Box::new(WindowsAuthProvider::new(windows_config)))
                }

                #[cfg(target_os = "linux")]
                {
                    use crate::auth::native_linux::LinuxAuthConfig;

                    let linux_config = LinuxAuthConfig {
                        allow_all_users: config.native.allow_all_users,
                        require_group: config.native.require_group.clone(),
                        permission_mapping: config.native.permission_mapping,
                        admin_groups: config.native.admin_groups.clone(),
                        permission_mappings: config.native.permission_mappings.clone(),
                    };

                    Ok(Box::new(LinuxAuthProvider::new(linux_config)))
                }

                #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
                #[cfg(unix)]
                {
                    use crate::auth::native_unix::UnixAuthConfig;

                    let unix_config = UnixAuthConfig {
                        allow_all_users: config.native.allow_all_users,
                        require_group: config.native.require_group.clone(),
                        permission_mapping: config.native.permission_mapping,
                        admin_groups: config.native.admin_groups.clone(),
                        permission_mappings: config.native.permission_mappings.clone(),
                    };

                    Ok(Box::new(crate::auth::native_unix::UnixAuthProvider::new(
                        unix_config,
                    )))
                }

                #[cfg(not(any(
                    target_os = "macos",
                    target_os = "windows",
                    target_os = "linux",
                    unix
                )))]
                {
                    Err(anyhow!(
                        "Native authentication not supported on this platform"
                    ))
                }

                #[cfg(not(any(
                    target_os = "macos",
                    target_os = "windows",
                    target_os = "linux",
                    unix
                )))]
                {
                    Err(anyhow!(
                        "Native authentication not supported on this platform"
                    ))
                }
            }
            AuthProviderType::Ldap => {
                info!("Using LDAP authentication provider");
                Err(anyhow!("LDAP provider not implemented yet"))
            }
            AuthProviderType::OAuth => {
                info!("Using OAuth authentication provider");
                Err(anyhow!("OAuth provider not implemented yet"))
            }
            AuthProviderType::Mock => {
                info!("Using mock authentication provider for testing");
                Ok(Box::new(MockAuthProvider::new()))
            }
        }
    }

    /// Create a mock provider with pre-configured test data
    #[cfg(test)]
    pub fn create_mock_provider() -> Box<dyn AuthProvider> {
        use crate::server::user::{User, UserRole};
        use uuid::Uuid;

        let test_user_id = Uuid::new_v4();
        let admin_user_id = Uuid::new_v4();

        let test_user = User {
            id: test_user_id,
            username: "testuser".to_string(),
            full_name: Some("Test User".to_string()),
            email: Some("test@example.com".to_string()),
            role: UserRole::User,
            password_hash: "hashed_password".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let admin_user = User {
            id: admin_user_id,
            username: "admin".to_string(),
            full_name: Some("Admin User".to_string()),
            email: Some("admin@example.com".to_string()),
            role: UserRole::Admin,
            password_hash: "hashed_admin_password".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let provider = MockAuthProvider::new()
            .with_user(test_user)
            .with_user(admin_user)
            .with_credential("testuser", b"password123")
            .with_credential("admin", b"admin123")
            .with_permission("testuser", "app:safari")
            .with_permission("testuser", "connect:*")
            .with_permission("admin", "admin:*");

        Box::new(provider)
    }
}
