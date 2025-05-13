use crate::error::ServiceError;
use tokio::sync::mpsc;

#[allow(dead_code)]
pub struct ServiceInstance {
    shutdown_tx: mpsc::Sender<()>,
}

impl ServiceInstance {
    #[allow(dead_code)]
    pub fn new(shutdown_tx: mpsc::Sender<()>) -> Self {
        Self { shutdown_tx }
    }

    #[allow(dead_code)]
    pub async fn shutdown(&self) -> Result<(), ServiceError> {
        self.shutdown_tx
            .send(())
            .await
            .map_err(|_| ServiceError::Service("Failed to send shutdown signal".to_string()))?;
        Ok(())
    }
}
