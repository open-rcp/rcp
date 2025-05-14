use crate::error::ServiceError;
use tokio::sync::mpsc;

#[allow(dead_code)]
pub struct ServiceLifecycle {
    shutdown_tx: mpsc::Sender<()>,
}

impl ServiceLifecycle {
    #[allow(dead_code)]
    pub fn new(shutdown_tx: mpsc::Sender<()>) -> Self {
        Self { shutdown_tx }
    }

    #[allow(dead_code)]
    pub async fn start(&self) -> Result<(), ServiceError> {
        // TODO: Implement lifecycle start
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn stop(&self) -> Result<(), ServiceError> {
        self.shutdown_tx
            .send(())
            .await
            .map_err(|_| ServiceError::Service("Failed to send shutdown signal".to_string()))?;
        Ok(())
    }
}
