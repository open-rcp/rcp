use anyhow::{Result, Context};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::Duration;
use crate::config::Config;

/// Service client for communication with the RCP service
pub struct ServiceClient {
    /// HTTP client for making requests
    client: Client,
    
    /// Base URL for the RCP service
    base_url: String,
    
    /// API key for authentication with the service
    api_key: String,
}

/// Server information from the service
#[derive(Debug, Clone, Deserialize)]
pub struct ServerInfo {
    /// Server ID
    pub id: String,
    
    /// Server name
    pub name: String,
    
    /// Server status (running, stopped, etc)
    pub status: String,
    
    /// Server address
    pub address: Option<String>,
    
    /// Server port
    pub port: Option<u16>,
    
    /// Creation timestamp
    pub created_at: String,
}

/// Session information from the service
#[derive(Debug, Clone, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    pub id: String,
    
    /// User ID associated with this session
    pub user_id: String,
    
    /// Server ID associated with this session
    pub server_id: String,
    
    /// Session status
    pub status: String,
    
    /// Session start time
    pub started_at: String,
    
    /// Session end time (if completed)
    pub ended_at: Option<String>,
}

/// Server creation request
#[derive(Debug, Serialize)]
pub struct CreateServerRequest {
    /// Server name
    pub name: String,
    
    /// Server configuration
    pub config: serde_json::Value,
}

/// Session creation request
#[derive(Debug, Serialize)]
pub struct CreateSessionRequest {
    /// User ID for the session
    pub user_id: String,
    
    /// Server ID to connect to
    pub server_id: String,
}

impl ServiceClient {
    /// Create a new service client
    pub fn new(config: Arc<Config>) -> Self {
        // Configure the HTTP client with reasonable defaults
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
            
        ServiceClient {
            client,
            base_url: config.rcp_service_url.clone(),
            api_key: config.rcp_service_api_key.clone(),
        }
    }
    
    /// Health check the RCP service
    pub async fn ping(&self) -> Result<String> {
        let url = format!("{}/health", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to connect to RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let body: serde_json::Value = response.json().await
                    .context("Failed to parse response from RCP service")?;
                    
                Ok(body["version"].as_str()
                    .unwrap_or("unknown")
                    .to_string())
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Service health check failed: {} - {}", status, body)
            }
        }
    }
    
    /// Get all servers
    pub async fn get_servers(&self) -> Result<Vec<ServerInfo>> {
        let url = format!("{}/servers", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch servers from RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let servers = response.json().await
                    .context("Failed to parse servers from RCP service")?;
                Ok(servers)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to get servers: {} - {}", status, body)
            }
        }
    }
    
    /// Get server by ID
    pub async fn get_server(&self, server_id: &str) -> Result<ServerInfo> {
        let url = format!("{}/servers/{}", self.base_url, server_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch server from RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let server = response.json().await
                    .context("Failed to parse server from RCP service")?;
                Ok(server)
            }
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Server not found: {}", server_id)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to get server: {} - {}", status, body)
            }
        }
    }
    
    /// Create a new server
    pub async fn create_server(&self, request: CreateServerRequest) -> Result<ServerInfo> {
        let url = format!("{}/servers", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to create server in RCP service")?;
            
        match response.status() {
            StatusCode::CREATED => {
                let server = response.json().await
                    .context("Failed to parse created server from RCP service")?;
                Ok(server)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to create server: {} - {}", status, body)
            }
        }
    }
    
    /// Start a server
    pub async fn start_server(&self, server_id: &str) -> Result<ServerInfo> {
        let url = format!("{}/servers/{}/start", self.base_url, server_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to start server in RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let server = response.json().await
                    .context("Failed to parse server from RCP service")?;
                Ok(server)
            }
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Server not found: {}", server_id)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to start server: {} - {}", status, body)
            }
        }
    }
    
    /// Stop a server
    pub async fn stop_server(&self, server_id: &str) -> Result<ServerInfo> {
        let url = format!("{}/servers/{}/stop", self.base_url, server_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to stop server in RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let server = response.json().await
                    .context("Failed to parse server from RCP service")?;
                Ok(server)
            }
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Server not found: {}", server_id)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to stop server: {} - {}", status, body)
            }
        }
    }
    
    /// Delete a server
    pub async fn delete_server(&self, server_id: &str) -> Result<()> {
        let url = format!("{}/servers/{}", self.base_url, server_id);
        
        let response = self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to delete server in RCP service")?;
            
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Server not found: {}", server_id)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to delete server: {} - {}", status, body)
            }
        }
    }
    
    /// Get all sessions
    pub async fn get_sessions(&self) -> Result<Vec<SessionInfo>> {
        let url = format!("{}/sessions", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch sessions from RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let sessions = response.json().await
                    .context("Failed to parse sessions from RCP service")?;
                Ok(sessions)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to get sessions: {} - {}", status, body)
            }
        }
    }
    
    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<SessionInfo> {
        let url = format!("{}/sessions/{}", self.base_url, session_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch session from RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let session = response.json().await
                    .context("Failed to parse session from RCP service")?;
                Ok(session)
            }
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Session not found: {}", session_id)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to get session: {} - {}", status, body)
            }
        }
    }
    
    /// Create a new session
    pub async fn create_session(&self, request: CreateSessionRequest) -> Result<SessionInfo> {
        let url = format!("{}/sessions", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to create session in RCP service")?;
            
        match response.status() {
            StatusCode::CREATED => {
                let session = response.json().await
                    .context("Failed to parse created session from RCP service")?;
                Ok(session)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to create session: {} - {}", status, body)
            }
        }
    }
    
    /// End a session
    pub async fn end_session(&self, session_id: &str) -> Result<SessionInfo> {
        let url = format!("{}/sessions/{}/end", self.base_url, session_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to end session in RCP service")?;
            
        match response.status() {
            StatusCode::OK => {
                let session = response.json().await
                    .context("Failed to parse session from RCP service")?;
                Ok(session)
            }
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Session not found: {}", session_id)
            }
            _ => {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to end session: {} - {}", status, body)
            }
        }
    }
}