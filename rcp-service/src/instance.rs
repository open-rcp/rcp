use crate::error::ServiceError;
use tokio::sync::mpsc;

pub struct ServiceInstance {
    shutdown_tx: mpsc::Sender<()>,
}

impl ServiceInstance {
    pub fn new(shutdown_tx: mpsc::Sender<()>) -> Self {
        Self { shutdown_tx }
    }

    pub async fn shutdown(&self) -> Result<(), ServiceError> {
        self.shutdown_tx.send(()).await
            .map_err(|_| ServiceError::Service("Failed to send shutdown signal".to_string()))?;
        Ok(())
    }
}
