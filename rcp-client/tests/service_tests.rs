use async_trait::async_trait;
use rcp_client::{Service, ServiceMessage, ServiceType};
use rcp_core::Frame;
use tokio::sync::oneshot;
use tokio::test;
use uuid::Uuid;

/// Test service type conversions
#[test]
async fn test_service_type_string() {
    assert_eq!(ServiceType::Display.as_str(), "display");
    assert_eq!(ServiceType::Input.as_str(), "input");
    assert_eq!(ServiceType::Audio.as_str(), "audio");
    assert_eq!(ServiceType::Clipboard.as_str(), "clipboard");
    assert_eq!(ServiceType::FileTransfer.as_str(), "file-transfer");
    assert_eq!(ServiceType::App.as_str(), "app");
    assert_eq!(ServiceType::Custom(123).as_str(), "custom");
}

/// Simple mock service implementation for testing
#[derive(Debug)]
struct MockService {
    #[allow(dead_code)]
    id: uuid::Uuid,
    name: String,
    service_type: ServiceType,
}

impl MockService {
    fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: "mock-service".to_string(),
            service_type: ServiceType::Custom(99),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn service_type(&self) -> ServiceType {
        self.service_type
    }
}

#[async_trait]
impl Service for MockService {
    async fn start(&mut self) -> rcp_client::Result<()> {
        Ok(())
    }

    async fn stop(&mut self) -> rcp_client::Result<()> {
        Ok(())
    }

    async fn handle_message(&mut self, _message: ServiceMessage) -> rcp_client::Result<()> {
        // Simply acknowledge receipt of the message
        Ok(())
    }
}

/// Test creating and using a service
#[test]
async fn test_service_usage() {
    // Create a simple mock service
    let mut service = MockService::new();

    // Start the service
    let start_result = service.start().await;
    assert!(start_result.is_ok());

    // Create a simple service message
    let (tx, _rx) = oneshot::channel();
    let message = ServiceMessage {
        id: Uuid::new_v4(),
        frame: Frame::new(0x01, b"test message".to_vec()),
        response_tx: Some(tx),
    };

    // Handle the message
    let handle_result = service.handle_message(message).await;
    assert!(handle_result.is_ok());

    // Stop the service
    let stop_result = service.stop().await;
    assert!(stop_result.is_ok());
    assert_eq!(service.name(), "mock-service");
    assert_eq!(service.service_type(), ServiceType::Custom(99));
}
