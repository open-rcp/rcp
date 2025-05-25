use crate::{
    error::{AppError, Result},
    models::Application,
};
use reqwest::Client;
use serde_json::Value;

pub struct ApplicationService {
    client: Client,
    daemon_url: String,
}

impl ApplicationService {
    pub fn new(daemon_url: String) -> Self {
        Self {
            client: Client::new(),
            daemon_url,
        }
    }

    pub async fn get_applications(&self) -> Result<Vec<Application>> {
        let url = format!("{}/api/v1/applications", self.daemon_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to fetch applications: {}",
                response.status()
            )));
        }
        
        let applications: Vec<Application> = response.json().await?;
        Ok(applications)
    }

    pub async fn get_application(&self, id: &str) -> Result<Application> {
        let url = format!("{}/api/v1/applications/{}", self.daemon_url, id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to fetch application: {}",
                response.status()
            )));
        }
        
        let application: Application = response.json().await?;
        Ok(application)
    }

    pub async fn launch_application(&self, id: &str) -> Result<Value> {
        let url = format!("{}/api/v1/applications/{}/launch", self.daemon_url, id);
        let response = self.client.post(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to launch application: {}",
                response.status()
            )));
        }
        
        let result: Value = response.json().await?;
        Ok(result)
    }

    pub async fn stop_application(&self, id: &str) -> Result<Value> {
        let url = format!("{}/api/v1/applications/{}/stop", self.daemon_url, id);
        let response = self.client.post(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to stop application: {}",
                response.status()
            )));
        }
        
        let result: Value = response.json().await?;
        Ok(result)
    }
}