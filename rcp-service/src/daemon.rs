use tokio::sync::mpsc;
use crate::error::ServiceError;

pub struct ServiceDaemon {
    shutdown_rx: mpsc::Receiver<()>,
}

impl ServiceDaemon {
    pub fn new(shutdown_rx: mpsc::Receiver<()>) -> Self {
        Self { shutdown_rx }
    }

    pub async fn run(&mut self) -> Result<(), ServiceError> {
        tokio::select! {
            _ = self.shutdown_rx.recv() => {
                println!("Received shutdown signal");
                Ok(())
            }
        }
    }
}
