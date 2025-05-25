use crate::error::{AppError, Result};
use crate::models::{Application, CreateApplication, ServerStatus, Session, SystemMetrics};
use reqwest::{Client, StatusCode};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub struct RcpDaemonClient {
    client: Client,
    base_url: String,
    auth_token: Option<String>,
}

impl RcpDaemonClient {
    pub async fn new(base_url: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create HTTP client: {}", e)))?;
        
        let mut rcp_client = Self {
            client,
            base_url: base_url.to_string(),
            auth_token: None,
        };
        
        // Test connection to rcpdaemon
        match rcp_client.get_status().await {
            Ok(_) => info!("Successfully connected to RCP daemon at {}", base_url),
            Err(e) => {
                error!("Failed to connect to RCP daemon: {}", e);
                // Continue anyway, as the daemon might become available later
            }
        }
        
        Ok(rcp_client)
    }
    
    pub fn with_auth_token(mut self, token: Option<String>) -> Self {
        self.auth_token = token;
        self
    }
    
    // Server Management
    
    pub async fn get_status(&self) -> Result<ServerStatus> {
        let response = self.client
            .get(&format!("{}/v1/status", self.base_url))
            .send()
            .await?;
            
        if response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to get server status: HTTP {}", 
                response.status()
            )));
        }
        
        let status = response.json::<ServerStatus>().await?;
        Ok(status)
    }
    
    // Session Management
    
    pub async fn get_sessions(&self) -> Result<Vec<Session>> {
        let response = self.client
            .get(&format!("{}/v1/sessions", self.base_url))
            .send()
            .await?;
            
        if response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to get sessions: HTTP {}", 
                response.status()
            )));
        }
        
        let sessions = response.json::<Vec<Session>>().await?;
        Ok(sessions)
    }
    
    pub async fn get_session(&self, session_id: Uuid) -> Result<Session> {
        let response = self.client
            .get(&format!("{}/v1/sessions/{}", self.base_url, session_id))
            .send()
            .await?;
            
        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound(format!("Session {} not found", session_id)));
        } else if response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to get session: HTTP {}", 
                response.status()
            )));
        }
        
        let session = response.json::<Session>().await?;
        Ok(session)
    }
    
    pub async fn close_session(&self, session_id: Uuid) -> Result<()> {
        let response = self.client
            .delete(&format!("{}/v1/sessions/{}", self.base_url, session_id))
            .send()
            .await?;
            
        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound(format!("Session {} not found", session_id)));
        } else if response.status() != StatusCode::OK && response.status() != StatusCode::NO_CONTENT {
            return Err(AppError::RcpDaemon(format!(
                "Failed to close session: HTTP {}", 
                response.status()
            )));
        }
        
        Ok(())
    }
    
    // Application Management
    
    pub async fn get_applications(&self) -> Result<Vec<Application>> {
        let response = self.client
            .get(&format!("{}/v1/apps", self.base_url))
            .send()
            .await?;
            
        if response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to get applications: HTTP {}", 
                response.status()
            )));
        }
        
        let apps = response.json::<Vec<Application>>().await?;
        Ok(apps)
    }
    
    pub async fn get_application(&self, app_id: &str) -> Result<Application> {
        let response = self.client
            .get(&format!("{}/v1/apps/{}", self.base_url, app_id))
            .send()
            .await?;
            
        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound(format!("Application {} not found", app_id)));
        } else if response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to get application: HTTP {}", 
                response.status()
            )));
        }
        
        let app = response.json::<Application>().await?;
        Ok(app)
    }
    
    pub async fn create_application(&self, app: CreateApplication) -> Result<Application> {
        let response = self.client
            .post(&format!("{}/v1/apps", self.base_url))
            .json(&app)
            .send()
            .await?;
            
        if response.status() == StatusCode::CONFLICT {
            return Err(AppError::Validation(format!(
                "Application with name '{}' already exists", 
                app.name
            )));
        } else if response.status() != StatusCode::CREATED && response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to create application: HTTP {}", 
                response.status()
            )));
        }
        
        let created_app = response.json::<Application>().await?;
        Ok(created_app)
    }
    
    pub async fn update_application(&self, app_id: &str, app: CreateApplication) -> Result<Application> {
        let response = self.client
            .put(&format!("{}/v1/apps/{}", self.base_url, app_id))
            .json(&app)
            .send()
            .await?;
            
        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound(format!("Application {} not found", app_id)));
        } else if response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to update application: HTTP {}", 
                response.status()
            )));
        }
        
        let updated_app = response.json::<Application>().await?;
        Ok(updated_app)
    }
    
    pub async fn delete_application(&self, app_id: &str) -> Result<()> {
        let response = self.client
            .delete(&format!("{}/v1/apps/{}", self.base_url, app_id))
            .send()
            .await?;
            
        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound(format!("Application {} not found", app_id)));
        } else if response.status() != StatusCode::OK && response.status() != StatusCode::NO_CONTENT {
            return Err(AppError::RcpDaemon(format!(
                "Failed to delete application: HTTP {}", 
                response.status()
            )));
        }
        
        Ok(())
    }
    
    // System monitoring
    
    pub async fn get_system_metrics(&self) -> Result<SystemMetrics> {
        let response = self.client
            .get(&format!("{}/v1/system/metrics", self.base_url))
            .send()
            .await?;
            
        if response.status() != StatusCode::OK {
            return Err(AppError::RcpDaemon(format!(
                "Failed to get system metrics: HTTP {}", 
                response.status()
            )));
        }
        
        let metrics = response.json::<SystemMetrics>().await?;
        Ok(metrics)
    }
}