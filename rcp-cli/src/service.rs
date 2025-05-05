use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::unix::pipe;
use tokio::time::timeout;

use crate::cli::UserInfo;
use crate::error::CliError;

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
            tokio::net::UnixStream::connect(socket_path)
        ).await;
        
        #[cfg(windows)]
        let socket_result = timeout(
            timeout_duration,
            tokio::net::windows::named_pipe::ClientOptions::new()
                .open(socket_path)
        ).await;
        
        // Handle timeout and connection errors
        match socket_result {
            Ok(Ok(socket)) => {
                #[cfg(unix)]
                return Ok(Self { socket });
                
                #[cfg(windows)]
                return Ok(Self { socket });
            }
            Ok(Err(e)) => Err(CliError::ServiceError(format!("Failed to connect: {}", e)).into()),
            Err(_) => Err(CliError::ConnectionTimeout.into()),
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
    pub async fn start_server(&mut self, name: &str) -> Result<()> {
        let args = serde_json::to_vec(&name)?;
        self.send_command("start-server", &args).await?;
        Ok(())
    }
    
    /// Stop a server
    pub async fn stop_server(&mut self, name: &str) -> Result<()> {
        let args = serde_json::to_vec(&name)?;
        self.send_command("stop-server", &args).await?;
        Ok(())
    }
    
    /// Restart a server
    pub async fn restart_server(&mut self, name: &str) -> Result<()> {
        let args = serde_json::to_vec(&name)?;
        self.send_command("restart-server", &args).await?;
        Ok(())
    }
    
    /// List users
    pub async fn list_users(&mut self) -> Result<Vec<UserInfo>> {
        let response = self.send_command("list-users", &[]).await?;
        let users: Vec<UserInfo> = serde_json::from_slice(&response)?;
        Ok(users)
    }
    
    /// Add a user
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
    
    // Additional methods will be implemented as needed
}