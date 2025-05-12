use crate::error::CliError;
use crate::service::{
    AppInfo, AppInstanceInfo, ServiceClient, ServiceStatus
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// CLI configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CliConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default = "default_format")]
    pub format: String,

    #[serde(default = "default_color")]
    pub color: bool,

    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,

    #[serde(default)]
    pub connection: ConnectionConfig,

    #[serde(default)]
    pub auth: AuthConfig,

    #[serde(default = "default_json_output")]
    pub json_output: bool,

    #[serde(default = "default_quiet")]
    pub quiet: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            format: default_format(),
            color: default_color(),
            timeout_seconds: default_timeout_seconds(),
            connection: ConnectionConfig::default(),
            auth: AuthConfig::default(),
            json_output: default_json_output(),
            quiet: default_quiet(),
        }
    }
}

/// Connection configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConnectionConfig {
    #[serde(default = "default_socket_path")]
    pub socket: String,

    #[serde(default = "default_api_url")]
    pub api_url: String,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            socket: default_socket_path(),
            api_url: default_api_url(),
        }
    }
}

/// Authentication configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    #[serde(default = "default_save_token")]
    pub save_token: bool,

    #[serde(default = "default_token_path")]
    pub token_path: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            save_token: default_save_token(),
            token_path: default_token_path(),
        }
    }
}

// Default values for configuration
fn default_log_level() -> String {
    "info".to_string()
}
fn default_format() -> String {
    "table".to_string()
}
fn default_color() -> bool {
    true
}
fn default_timeout_seconds() -> u64 {
    5
}
fn default_json_output() -> bool {
    false
}
fn default_quiet() -> bool {
    false
}

fn default_socket_path() -> String {
    if cfg!(windows) {
        "\\\\.\\pipe\\rcp-service".to_string()
    } else {
        "/var/run/rcp-service.sock".to_string()
    }
}

fn default_api_url() -> String {
    "http://localhost:8080/api/v1".to_string()
}
fn default_save_token() -> bool {
    true
}
fn default_token_path() -> String {
    if cfg!(windows) {
        "%APPDATA%\\RCP\\token".to_string()
    } else {
        "~/.config/rcp/token".to_string()
    }
}

/// User information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub username: String,
    pub role: String,
}

/// Main CLI class for interacting with the RCP service
pub struct Cli {
    config: CliConfig,
    service_client: Option<ServiceClient>,
}

impl Cli {
    /// Create a new CLI instance with the provided configuration
    pub fn new(config: CliConfig) -> Self {
        Self {
            config,
            service_client: None,
        }
    }

    /// Connect to the RCP service
    pub async fn connect(&mut self) -> Result<()> {
        // Create a service client
        let client =
            ServiceClient::connect(&self.config.connection.socket, self.config.timeout_seconds)
                .await?;

        self.service_client = Some(client);
        Ok(())
    }

    /// Disconnect from the RCP service
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(client) = self.service_client.take() {
            client.disconnect().await?;
        }
        Ok(())
    }

    /// Get a mutable reference to the service client
    pub fn get_service_client_mut(&mut self) -> Result<&mut ServiceClient> {
        self.service_client
            .as_mut()
            .ok_or_else(|| CliError::NotConnected.into())
    }

    /// Get service status
    pub async fn get_status(&mut self) -> Result<ServiceStatus> {
        self.get_service_client_mut()?.get_status().await
    }

    /// Start a server
    #[allow(dead_code)]
    pub async fn start_server(&mut self, name: &str) -> Result<()> {
        self.get_service_client_mut()?.start_server(name).await
    }

    /// Stop a server
    #[allow(dead_code)]
    pub async fn stop_server(&mut self, name: &str) -> Result<()> {
        self.get_service_client_mut()?.stop_server(name).await
    }

    /// Restart a server
    #[allow(dead_code)]
    pub async fn restart_server(&mut self, name: &str) -> Result<()> {
        self.get_service_client_mut()?.restart_server(name).await
    }

    /// List users
    #[allow(dead_code)]
    pub async fn list_users(&mut self) -> Result<Vec<UserInfo>> {
        self.get_service_client_mut()?.list_users().await
    }

    /// Add a user
    #[allow(dead_code)]
    pub async fn add_user(&mut self, username: &str, password: &str, role: &str) -> Result<()> {
        self.get_service_client_mut()?
            .add_user(username, password, role)
            .await
    }

    /// Delete a user
    #[allow(dead_code)]
    pub async fn delete_user(&mut self, username: &str) -> Result<()> {
        self.get_service_client_mut()?.delete_user(username).await
    }

    /// Update a user's role
    #[allow(dead_code)]
    pub async fn update_user_role(&mut self, username: &str, role: &str) -> Result<()> {
        self.get_service_client_mut()?
            .update_user_role(username, role)
            .await
    }

    /// Reset a user's password
    #[allow(dead_code)]
    pub async fn reset_user_password(&mut self, username: &str, new_password: &str) -> Result<()> {
        self.get_service_client_mut()?
            .reset_user_password(username, new_password)
            .await
    }

    /// List applications
    #[allow(dead_code)]
    pub async fn list_apps(&mut self) -> Result<Vec<AppInfo>> {
        self.get_service_client_mut()?.list_apps().await
    }

    /// Get application details
    #[allow(dead_code)]
    pub async fn get_app(&mut self, id: &str) -> Result<AppInfo> {
        self.get_service_client_mut()?.get_app(id).await
    }

    /// Create a new application
    #[allow(dead_code)]
    pub async fn create_app(
        &mut self,
        name: &str,
        path: &str,
        args: Option<&str>,
        description: Option<&str>,
    ) -> Result<AppInfo> {
        self.get_service_client_mut()?
            .create_app(name, path, args, description)
            .await
    }

    /// Update an application
    #[allow(dead_code)]
    pub async fn update_app(
        &mut self,
        id: &str,
        name: Option<&str>,
        path: Option<&str>,
        args: Option<&str>,
        description: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<AppInfo> {
        self.get_service_client_mut()?
            .update_app(id, name, path, args, description, enabled)
            .await
    }

    /// Delete an application
    #[allow(dead_code)]
    pub async fn delete_app(&mut self, id: &str) -> Result<()> {
        self.get_service_client_mut()?.delete_app(id).await
    }

    /// Enable an application
    #[allow(dead_code)]
    pub async fn enable_app(&mut self, id: &str) -> Result<()> {
        self.get_service_client_mut()?.enable_app(id).await
    }

    /// Disable an application
    #[allow(dead_code)]
    pub async fn disable_app(&mut self, id: &str) -> Result<()> {
        self.get_service_client_mut()?.disable_app(id).await
    }

    /// Launch an application
    #[allow(dead_code)]
    pub async fn launch_app(
        &mut self,
        id: &str,
        user_id: Option<&str>,
    ) -> Result<serde_json::Value> {
        self.get_service_client_mut()?.launch_app(id, user_id).await
    }

    /// List running application instances
    #[allow(dead_code)]
    pub async fn list_app_instances(&mut self) -> Result<Vec<AppInstanceInfo>> {
        self.get_service_client_mut()?.list_app_instances().await
    }

    /// Terminate an application instance
    #[allow(dead_code)]
    pub async fn terminate_app_instance(&mut self, instance_id: &str) -> Result<()> {
        self.get_service_client_mut()?
            .terminate_app_instance(instance_id)
            .await
    }

    /// Get a reference to the configuration
    #[allow(dead_code)]
    pub fn get_config(&self) -> &CliConfig {
        &self.config
    }
}

impl CliConfig {
    /// Load configuration from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str::<CliConfig>(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    #[allow(dead_code)]
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
