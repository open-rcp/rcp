use crate::{
    error::{AppError, Result},
    models::SystemMetrics,
};
use reqwest::Client;
use serde_json::Value;

pub struct MetricsService {
    client: Client,
    daemon_url: String,
}

impl MetricsService {
    pub fn new(daemon_url: String) -> Self {
        Self {
            client: Client::new(),
            daemon_url,
        }
    }

    pub async fn get_system_metrics(&self) -> Result<SystemMetrics> {
        let url = format!("{}/api/v1/metrics/system", self.daemon_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to fetch system metrics: {}",
                response.status()
            )));
        }
        
        let metrics: SystemMetrics = response.json().await?;
        Ok(metrics)
    }

    pub async fn get_performance_metrics(&self) -> Result<Value> {
        let url = format!("{}/api/v1/metrics/performance", self.daemon_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to fetch performance metrics: {}",
                response.status()
            )));
        }
        
        let metrics: Value = response.json().await?;
        Ok(metrics)
    }

    pub async fn get_health_check(&self) -> Result<Value> {
        let url = format!("{}/api/v1/health", self.daemon_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Health check failed: {}",
                response.status()
            )));
        }
        
        let health: Value = response.json().await?;
        Ok(health)
    }
}