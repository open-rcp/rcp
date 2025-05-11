use crate::error::ServiceError;
use tokio::sync::mpsc;

pub struct ServiceLifecycle {
    shutdown_tx: mpsc::Sender<()>,
}

impl ServiceLifecycle {
    pub fn new(shutdown_tx: mpsc::Sender<()>) -> Self {
        Self { shutdown_tx }
    }

    pub async fn start(&self) -> Result<(), ServiceError> {
        // TODO: Implement lifecycle start
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), ServiceError> {
        self.shutdown_tx
            .send(())
            .await
            .map_err(|_| ServiceError::Service("Failed to send shutdown signal".to_string()))?;
        Ok(())
    }
}
