//! CLI service client
//!
//! This module provides functionality for CLI commands to communicate with the daemon.

#[cfg(feature = "cli")]
use crate::cli::error::CliError;
#[cfg(feature = "cli")]
use anyhow::Result;
#[cfg(feature = "cli")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "cli")]
use std::time::Duration;
#[cfg(feature = "cli")]
use tokio::net::TcpStream;
#[cfg(feature = "cli")]
use tokio::time::timeout;
#[cfg(feature = "cli")]
use uuid::Uuid;

/// Service status information
#[cfg(feature = "cli")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub uptime: Option<String>,
    pub version: String,
}

/// Application instance information
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppInstanceInfo {
    pub id: String,
    pub app_id: String,
    pub name: String,
    pub user_id: String,
    pub status: String,
    pub created_at: String,
}

/// Application information
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub publisher: Option<String>,
    pub icon_path: Option<String>,
    pub executable_path: String,
}

/// Server information
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInfo {
    pub version: String,
    pub uptime: String,
    pub address: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub active_sessions: usize,
    pub total_sessions: usize,
}

/// Session information
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub client_ip: String,
    pub created_at: String,
    pub expires_at: String,
    pub last_active: String,
    pub active: bool,
}

/// Service client for CLI to communicate with the daemon
#[cfg(feature = "cli")]
pub struct ServiceClient {
    pub host: String,
    pub port: u16,
    pub timeout_seconds: u64,
    pub auth_token: Option<String>,
}

#[cfg(feature = "cli")]
impl ServiceClient {
    /// Create a new service client
    pub fn new(host: String, port: u16, timeout_seconds: u64) -> Self {
        Self {
            host,
            port,
            timeout_seconds,
            auth_token: None,
        }
    }

    /// Set authentication token
    pub fn with_auth(mut self, token: Option<String>) -> Self {
        self.auth_token = token;
        self
    }

    /// Get service status
    pub async fn get_status(&self) -> Result<ServiceStatus, CliError> {
        let request = self.build_request("status", serde_json::Value::Null)?;
        let response = self.send_request(request).await?;

        let status: ServiceStatus = serde_json::from_value(response)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        Ok(status)
    }

    /// Get server information
    pub async fn get_server_info(&self) -> Result<ServerInfo, CliError> {
        let request = self.build_request("server/info", serde_json::Value::Null)?;
        let response = self.send_request(request).await?;

        let info: ServerInfo = serde_json::from_value(response)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        Ok(info)
    }

    /// Get list of applications
    pub async fn list_apps(&self) -> Result<Vec<AppInfo>, CliError> {
        let request = self.build_request("apps/list", serde_json::Value::Null)?;
        let response = self.send_request(request).await?;

        let apps: Vec<AppInfo> = serde_json::from_value(response)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        Ok(apps)
    }

    /// Get list of application instances
    pub async fn list_app_instances(&self) -> Result<Vec<AppInstanceInfo>, CliError> {
        let request = self.build_request("apps/instances", serde_json::Value::Null)?;
        let response = self.send_request(request).await?;

        let instances: Vec<AppInstanceInfo> = serde_json::from_value(response)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        Ok(instances)
    }

    /// Launch an application
    pub async fn launch_app(
        &self,
        app_id: &str,
        user_id: Option<&str>,
        args: Option<Vec<String>>,
    ) -> Result<AppInstanceInfo, CliError> {
        let params = serde_json::json!({
            "app_id": app_id,
            "user_id": user_id,
            "arguments": args
        });

        let request = self.build_request("apps/launch", params)?;
        let response = self.send_request(request).await?;

        let instance: AppInstanceInfo = serde_json::from_value(response)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        Ok(instance)
    }

    /// Stop an application instance
    pub async fn stop_app(&self, instance_id: &str) -> Result<(), CliError> {
        let params = serde_json::json!({
            "instance_id": instance_id
        });

        let request = self.build_request("apps/stop", params)?;
        let _response = self.send_request(request).await?;

        Ok(())
    }

    /// Get list of active sessions
    pub async fn list_sessions(&self) -> Result<Vec<SessionInfo>, CliError> {
        let request = self.build_request("sessions/list", serde_json::Value::Null)?;
        let response = self.send_request(request).await?;

        let sessions: Vec<SessionInfo> = serde_json::from_value(response)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        Ok(sessions)
    }

    /// Disconnect a session
    pub async fn disconnect_session(&self, session_id: &str) -> Result<(), CliError> {
        let params = serde_json::json!({
            "session_id": session_id
        });

        let request = self.build_request("sessions/disconnect", params)?;
        let _response = self.send_request(request).await?;

        Ok(())
    }

    /// Build a request to the service
    fn build_request(&self, method: &str, params: serde_json::Value) -> Result<String, CliError> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4().to_string(),
            "method": method,
            "params": params,
            "auth": self.auth_token
        });

        serde_json::to_string(&request).map_err(|e| CliError::SerializationError(e.to_string()))
    }

    /// Send a request to the service
    async fn send_request(&self, request: String) -> Result<serde_json::Value, CliError> {
        // Connect to the service
        let address = format!("{}:{}", self.host, self.port);
        let stream_result = timeout(
            Duration::from_secs(self.timeout_seconds),
            TcpStream::connect(&address),
        )
        .await;

        let mut stream = match stream_result {
            Ok(Ok(stream)) => stream,
            Ok(Err(e)) => return Err(CliError::CommunicationError(e.to_string())),
            Err(_) => {
                return Err(CliError::CommunicationError(format!(
                    "Operation timed out after {} seconds",
                    self.timeout_seconds
                )))
            }
        };

        // Send the request
        let result = timeout(Duration::from_secs(self.timeout_seconds), async {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};

            // Write request with length prefix
            let bytes = request.as_bytes();
            let len = bytes.len() as u32;
            stream.write_all(&len.to_be_bytes()).await?;
            stream.write_all(bytes).await?;

            // Read length prefix
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).await?;
            let len = u32::from_be_bytes(len_buf) as usize;

            // Read response
            let mut response = vec![0u8; len];
            stream.read_exact(&mut response).await?;

            let response_str = String::from_utf8(response)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            Ok::<String, std::io::Error>(response_str)
        })
        .await;

        let response_str = match result {
            Ok(Ok(response)) => response,
            Ok(Err(e)) => return Err(CliError::FileSystemError(e.to_string())),
            Err(_) => {
                return Err(CliError::CommunicationError(format!(
                    "Operation timed out after {} seconds",
                    self.timeout_seconds
                )))
            }
        };

        // Parse the response
        let response: serde_json::Value = serde_json::from_str(&response_str)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        // Check for errors
        if let Some(error) = response.get("error") {
            let error_msg = error["message"].as_str().unwrap_or("Unknown error");
            return Err(CliError::CommunicationError(error_msg.to_string()));
        }

        // Extract result
        if let Some(result) = response.get("result") {
            Ok(result.clone())
        } else {
            Err(CliError::CommunicationError(
                "Invalid response format".to_string(),
            ))
        }
    }
}
