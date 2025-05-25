use crate::{
    error::{AppError, Result},
    models::Session,
};
use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

pub struct SessionService {
    client: Client,
    daemon_url: String,
}

impl SessionService {
    pub fn new(daemon_url: String) -> Self {
        Self {
            client: Client::new(),
            daemon_url,
        }
    }

    pub async fn get_sessions(&self) -> Result<Vec<Session>> {
        let url = format!("{}/api/v1/sessions", self.daemon_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to fetch sessions: {}",
                response.status()
            )));
        }

        let sessions: Vec<Session> = response.json().await?;
        Ok(sessions)
    }

    pub async fn get_session(&self, id: Uuid) -> Result<Session> {
        let url = format!("{}/api/v1/sessions/{}", self.daemon_url, id);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to fetch session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn terminate_session(&self, id: Uuid) -> Result<Value> {
        let url = format!("{}/api/v1/sessions/{}/terminate", self.daemon_url, id);
        let response = self.client.post(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to terminate session: {}",
                response.status()
            )));
        }

        let result: Value = response.json().await?;
        Ok(result)
    }

    pub async fn get_session_metrics(&self, id: Uuid) -> Result<Value> {
        let url = format!("{}/api/v1/sessions/{}/metrics", self.daemon_url, id);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Failed to fetch session metrics: {}",
                response.status()
            )));
        }

        let metrics: Value = response.json().await?;
        Ok(metrics)
    }
}
