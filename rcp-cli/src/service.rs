use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::time::timeout;

use crate::cli::UserInfo;
use crate::error::CliError;

/// Application information returned by the service
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppInfo {
    /// Application ID
    pub id: String,

    /// Application name
    pub name: String,

    /// Application path
    pub path: String,

    /// Command-line arguments to pass to the application
    pub args: Option<String>,

    /// Application description
    pub description: Option<String>,

    /// Whether the application is enabled
    pub enabled: bool,

    /// Creation timestamp
    pub created_at: Option<String>,

    /// Last update timestamp
    pub updated_at: Option<String>,
}

/// Running application instance information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppInstanceInfo {
    /// Instance ID
    pub instance_id: String,

    /// Application ID
    pub app_id: String,

    /// Application name
    pub app_name: String,

    /// User ID who launched the application
    pub user_id: Option<String>,

    /// Process ID on the host system
    pub pid: Option<u32>,

    /// Instance status
    pub status: String,

    /// Start timestamp
    pub start_time: String,

    /// Resource usage information
    pub resources: Option<ResourceUsage>,
}

/// Resource usage information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_percent: f32,

    /// Memory usage in MB
    pub memory_mb: f32,
}

/// Client for communicating with the RCP service
pub struct ServiceClient {
    #[cfg(unix)]
    socket: tokio::net::UnixStream,

    #[cfg(windows)]
    socket: tokio::net::windows::named_pipe::NamedPipeClient,
}

/// Status information for the RCP service
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceStatus {
    pub service_status: String,
    pub uptime: u64,
    pub active_servers: Vec<ServerInfo>,
    pub active_connections: u32,
}

/// Server information returned by the service
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub port: u16,
    pub active_connections: u32,
}

impl ServiceClient {
    /// Connect to the RCP service
    pub async fn connect(socket_path: &str, timeout_seconds: u64) -> Result<Self> {
        // Set a timeout for connection attempts
        let timeout_duration = Duration::from_secs(timeout_seconds);

        #[cfg(unix)]
        let socket_result = timeout(
            timeout_duration,
            tokio::net::UnixStream::connect(socket_path),
        )
        .await;

        #[cfg(windows)]
        let socket_result = timeout(
            timeout_duration,
            tokio::net::windows::named_pipe::ClientOptions::new().open(socket_path),
        )
        .await;

        // Handle timeout and connection errors
        match socket_result {
            Ok(Ok(socket)) => {
                #[cfg(unix)]
                return Ok(Self { socket });

                #[cfg(windows)]
                return Ok(Self { socket });
            }
            Ok(Err(e)) => Err(CliError::ServiceError(format!("Failed to connect: {}", e)).into()),
            Err(_) => Err(CliError::Timeout.into()),
        }
    }

    /// Disconnect from the service
    pub async fn disconnect(self) -> Result<()> {
        // The socket will be closed when dropped
        Ok(())
    }

    /// Send a command to the service
    async fn send_command(&mut self, command: &str, args: &[u8]) -> Result<Vec<u8>> {
        // Format: [command length (u32)][command string][data length (u32)][data]
        let command_bytes = command.as_bytes();
        let command_len = command_bytes.len() as u32;
        let args_len = args.len() as u32;

        // Create the message
        let mut message = Vec::with_capacity(8 + command_bytes.len() + args.len());
        message.extend_from_slice(&command_len.to_le_bytes());
        message.extend_from_slice(command_bytes);
        message.extend_from_slice(&args_len.to_le_bytes());
        message.extend_from_slice(args);

        // Send the message
        #[cfg(unix)]
        self.socket.write_all(&message).await?;

        #[cfg(windows)]
        self.socket.write_all(&message).await?;

        // Read the response
        // Format: [status code (u32)][data length (u32)][data]
        let mut status_buf = [0u8; 4];
        #[cfg(unix)]
        self.socket.read_exact(&mut status_buf).await?;

        #[cfg(windows)]
        self.socket.read_exact(&mut status_buf).await?;

        let status = u32::from_le_bytes(status_buf);

        let mut data_len_buf = [0u8; 4];
        #[cfg(unix)]
        self.socket.read_exact(&mut data_len_buf).await?;

        #[cfg(windows)]
        self.socket.read_exact(&mut data_len_buf).await?;

        let data_len = u32::from_le_bytes(data_len_buf) as usize;

        let mut data = vec![0u8; data_len];
        #[cfg(unix)]
        self.socket.read_exact(&mut data).await?;

        #[cfg(windows)]
        self.socket.read_exact(&mut data).await?;

        // Check status code
        if status != 0 {
            // Try to parse the error message
            let error_msg = String::from_utf8_lossy(&data).to_string();
            return Err(CliError::ServiceError(error_msg).into());
        }

        Ok(data)
    }

    /// Get service status
    pub async fn get_status(&mut self) -> Result<ServiceStatus> {
        let response = self.send_command("status", &[]).await?;
        let status: ServiceStatus = serde_json::from_slice(&response)?;
        Ok(status)
    }

    /// Start a server
    #[allow(dead_code)]
    pub async fn start_server(&mut self, name: &str) -> Result<()> {
        let args = serde_json::to_vec(&name)?;
        self.send_command("start-server", &args).await?;
        Ok(())
    }

    /// Stop a server
    #[allow(dead_code)]
    pub async fn stop_server(&mut self, name: &str) -> Result<()> {
        let args = serde_json::to_vec(&name)?;
        self.send_command("stop-server", &args).await?;
        Ok(())
    }

    /// Restart a server
    #[allow(dead_code)]
    pub async fn restart_server(&mut self, name: &str) -> Result<()> {
        let args = serde_json::to_vec(&name)?;
        self.send_command("restart-server", &args).await?;
        Ok(())
    }

    /// List users
    #[allow(dead_code)]
    pub async fn list_users(&mut self) -> Result<Vec<UserInfo>> {
        let response = self.send_command("list-users", &[]).await?;

        // Parse the JSON response which should include a "users" field
        let result: serde_json::Value = serde_json::from_slice(&response)?;

        if let Some(users) = result.get("users") {
            let users: Vec<UserInfo> = serde_json::from_value(users.clone())?;
            Ok(users)
        } else {
            Ok(Vec::new())
        }
    }

    /// Add a user
    #[allow(dead_code)]
    pub async fn add_user(&mut self, username: &str, password: &str, role: &str) -> Result<()> {
        #[derive(Serialize)]
        struct NewUser<'a> {
            username: &'a str,
            password: &'a str,
            role: &'a str,
        }

        let new_user = NewUser {
            username,
            password,
            role,
        };

        let args = serde_json::to_vec(&new_user)?;
        self.send_command("add-user", &args).await?;
        Ok(())
    }

    /// Delete a user
    #[allow(dead_code)]
    pub async fn delete_user(&mut self, username: &str) -> Result<()> {
        let args = serde_json::to_vec(&username)?;
        self.send_command("delete-user", &args).await?;
        Ok(())
    }

    /// Update a user's role
    #[allow(dead_code)]
    pub async fn update_user_role(&mut self, username: &str, role: &str) -> Result<()> {
        #[derive(Serialize)]
        struct UpdateRole<'a> {
            username: &'a str,
            role: &'a str,
        }

        let update = UpdateRole { username, role };
        let args = serde_json::to_vec(&update)?;
        self.send_command("update-user-role", &args).await?;
        Ok(())
    }

    /// Reset a user's password (admin only)
    #[allow(dead_code)]
    pub async fn reset_user_password(&mut self, username: &str, new_password: &str) -> Result<()> {
        #[derive(Serialize)]
        struct ResetPassword<'a> {
            username: &'a str,
            new_password: &'a str,
        }

        let reset = ResetPassword {
            username,
            new_password,
        };
        let args = serde_json::to_vec(&reset)?;
        self.send_command("reset-user-password", &args).await?;
        Ok(())
    }

    pub async fn is_service_installed(&mut self) -> Result<bool> {
        let response = self.send_command("check-install", &[]).await?;
        let installed: bool = serde_json::from_slice(&response)?;
        Ok(installed)
    }

    pub async fn install_service(&mut self, options: &serde_json::Value) -> Result<()> {
        let args = serde_json::to_vec(options)?;
        self.send_command("install", &args).await?;
        Ok(())
    }

    pub async fn uninstall_service(&mut self) -> Result<()> {
        self.send_command("uninstall", &[]).await?;
        Ok(())
    }

    pub async fn start_service(&mut self) -> Result<()> {
        self.send_command("start", &[]).await?;
        Ok(())
    }

    pub async fn stop_service(&mut self) -> Result<()> {
        self.send_command("stop", &[]).await?;
        Ok(())
    }

    pub async fn restart_service(&mut self) -> Result<()> {
        self.send_command("restart", &[]).await?;
        Ok(())
    }

    /// List applications
    #[allow(dead_code)]
    pub async fn list_apps(&mut self) -> Result<Vec<AppInfo>> {
        let response = self.send_command("list-apps", &[]).await?;

        // Parse the JSON response
        let apps: Vec<AppInfo> = serde_json::from_slice(&response)?;

        Ok(apps)
    }

    /// Get application details by id
    #[allow(dead_code)]
    pub async fn get_app(&mut self, id: &str) -> Result<AppInfo> {
        let args = serde_json::to_vec(&serde_json::json!({
            "id": id
        }))?;

        let response = self.send_command("get-app", &args).await?;
        let app: AppInfo = serde_json::from_slice(&response)?;

        Ok(app)
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
        #[derive(Serialize)]
        struct NewApp<'a> {
            name: &'a str,
            path: &'a str,
            args: Option<&'a str>,
            description: Option<&'a str>,
        }

        let new_app = NewApp {
            name,
            path,
            args,
            description,
        };

        let args = serde_json::to_vec(&new_app)?;
        let response = self.send_command("create-app", &args).await?;

        let app: AppInfo = serde_json::from_slice(&response)?;
        Ok(app)
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
        #[derive(Serialize)]
        struct UpdateApp<'a> {
            id: &'a str,
            name: Option<&'a str>,
            path: Option<&'a str>,
            args: Option<&'a str>,
            description: Option<&'a str>,
            enabled: Option<bool>,
        }

        let update = UpdateApp {
            id,
            name,
            path,
            args,
            description,
            enabled,
        };

        let args = serde_json::to_vec(&update)?;
        let response = self.send_command("update-app", &args).await?;

        let app: AppInfo = serde_json::from_slice(&response)?;
        Ok(app)
    }

    /// Delete an application
    #[allow(dead_code)]
    pub async fn delete_app(&mut self, id: &str) -> Result<()> {
        let args = serde_json::to_vec(&serde_json::json!({
            "id": id
        }))?;

        self.send_command("delete-app", &args).await?;
        Ok(())
    }

    /// Enable an application
    #[allow(dead_code)]
    pub async fn enable_app(&mut self, id: &str) -> Result<()> {
        let args = serde_json::to_vec(&serde_json::json!({
            "id": id,
            "enabled": true
        }))?;

        self.send_command("update-app", &args).await?;
        Ok(())
    }

    /// Disable an application
    #[allow(dead_code)]
    pub async fn disable_app(&mut self, id: &str) -> Result<()> {
        let args = serde_json::to_vec(&serde_json::json!({
            "id": id,
            "enabled": false
        }))?;

        self.send_command("update-app", &args).await?;
        Ok(())
    }

    /// Launch an application
    #[allow(dead_code)]
    pub async fn launch_app(
        &mut self,
        id: &str,
        user_id: Option<&str>,
    ) -> Result<serde_json::Value> {
        let args = serde_json::to_vec(&serde_json::json!({
            "id": id,
            "user_id": user_id
        }))?;

        let response = self.send_command("launch-app", &args).await?;
        let result: serde_json::Value = serde_json::from_slice(&response)?;

        Ok(result)
    }

    /// List running application instances
    #[allow(dead_code)]
    pub async fn list_app_instances(&mut self) -> Result<Vec<AppInstanceInfo>> {
        let response = self.send_command("list-app-instances", &[]).await?;
        let instances: Vec<AppInstanceInfo> = serde_json::from_slice(&response)?;

        Ok(instances)
    }

    /// Terminate an application instance
    #[allow(dead_code)]
    pub async fn terminate_app_instance(&mut self, instance_id: &str) -> Result<()> {
        let args = serde_json::to_vec(&serde_json::json!({
            "instance_id": instance_id
        }))?;

        self.send_command("terminate-app-instance", &args).await?;
        Ok(())
    }
}
