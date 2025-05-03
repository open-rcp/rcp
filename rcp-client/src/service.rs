use crate::error::{Error, Result};
use log::{debug, trace};
use rcp_core::{CommandId, Frame};
use std::fmt;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

/// Service type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceType {
    /// Display service for screen sharing
    Display,
    
    /// Input service for sending keyboard/mouse events
    Input,
    
    /// Audio service for streaming audio
    Audio,
    
    /// Clipboard service for clipboard synchronization
    Clipboard,
    
    /// File transfer service
    FileTransfer,
    
    /// Custom service
    Custom(u8),
}

impl ServiceType {
    /// Get the string representation of a service type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Display => "display",
            Self::Input => "input",
            Self::Audio => "audio",
            Self::Clipboard => "clipboard",
            Self::FileTransfer => "file-transfer",
            Self::Custom(_) => "custom",
        }
    }
    
    /// Get the command ID for subscribing to this service
    pub fn subscription_command(&self) -> u8 {
        match self {
            Self::Display => CommandId::SubscribeDisplay as u8,
            Self::Input => CommandId::SubscribeInput as u8,
            Self::Audio => CommandId::SubscribeAudio as u8,
            Self::Clipboard => CommandId::SubscribeClipboard as u8,
            Self::FileTransfer => CommandId::SubscribeFileTransfer as u8,
            Self::Custom(id) => *id,
        }
    }
    
    /// Get a service type from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "display" => Some(Self::Display),
            "input" => Some(Self::Input),
            "audio" => Some(Self::Audio),
            "clipboard" => Some(Self::Clipboard),
            "file-transfer" => Some(Self::FileTransfer),
            _ => None,
        }
    }
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Service message with request-response channel
#[derive(Debug)]
pub struct ServiceMessage {
    /// Message ID
    pub id: Uuid,
    
    /// Frame containing the message
    pub frame: Frame,
    
    /// Response channel
    pub response_tx: Option<oneshot::Sender<Result<Frame>>>,
}

impl Clone for ServiceMessage {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            frame: self.frame.clone(),
            response_tx: None, // Can't clone the oneshot sender
        }
    }
}

/// Generic service trait
#[async_trait::async_trait]
pub trait Service: Send + Sync {
    /// Start the service
    async fn start(&mut self) -> Result<()>;
    
    /// Stop the service
    async fn stop(&mut self) -> Result<()>;
    
    /// Handle an incoming message
    async fn handle_message(&mut self, message: ServiceMessage) -> Result<()>;
}

/// Client-side service client
#[derive(Debug, Clone)]
pub struct ServiceClient {
    /// Service type
    service_type: ServiceType,
    
    /// Service name
    service_name: String,
    
    /// Message sender channel
    tx: mpsc::Sender<ServiceMessage>,
}

impl ServiceClient {
    /// Create a new service client
    pub fn new(
        service_type: ServiceType,
        service_name: String,
        tx: mpsc::Sender<ServiceMessage>,
    ) -> Self {
        Self {
            service_type,
            service_name,
            tx,
        }
    }
    
    /// Get the service type
    pub fn service_type(&self) -> ServiceType {
        self.service_type
    }
    
    /// Get the service name
    pub fn service_name(&self) -> &str {
        &self.service_name
    }
    
    /// Send a message and get a response
    pub async fn send_request(&self, frame: Frame) -> Result<Frame> {
        let (tx, rx) = oneshot::channel();
        let msg = ServiceMessage {
            id: Uuid::new_v4(),
            frame,
            response_tx: Some(tx),
        };
        
        // Send the message to the service handler
        trace!("Sending request message to service {}", self.service_name);
        self.tx.send(msg).await.map_err(|_| {
            Error::Service(format!("Failed to send message to service {}", self.service_name))
        })?;
        
        // Wait for the response
        trace!("Waiting for response from service {}", self.service_name);
        let response = rx.await.map_err(|_| {
            Error::Service(format!("Failed to receive response from service {}", self.service_name))
        })??;
        
        Ok(response)
    }
    
    /// Send a message without expecting a response
    pub async fn send_fire_and_forget(&self, frame: Frame) -> Result<()> {
        let msg = ServiceMessage {
            id: Uuid::new_v4(),
            frame,
            response_tx: None,
        };
        
        // Send the message to the service handler
        trace!("Sending fire-and-forget message to service {}", self.service_name);
        self.tx.send(msg).await.map_err(|_| {
            Error::Service(format!("Failed to send message to service {}", self.service_name))
        })?;
        
        Ok(())
    }
}

/// Factory for creating service instances
pub struct ServiceFactory;

impl ServiceFactory {
    /// Create a new service instance
    pub fn create(service_type: ServiceType) -> Option<Box<dyn Service>> {
        match service_type {
            ServiceType::Display => Some(Box::new(builtin::DisplayService::new())),
            ServiceType::Input => Some(Box::new(builtin::InputService::new())),
            ServiceType::Clipboard => Some(Box::new(builtin::ClipboardService::new())),
            ServiceType::FileTransfer => Some(Box::new(builtin::FileTransferService::new())),
            _ => None,
        }
    }
}

/// Built-in service implementations
pub mod builtin {
    use super::*;
    
    /// Display service implementation
    pub struct DisplayService {}
    
    impl DisplayService {
        /// Create a new display service
        pub fn new() -> Self {
            Self {}
        }
    }
    
    #[async_trait::async_trait]
    impl Service for DisplayService {
        async fn start(&mut self) -> Result<()> {
            debug!("Starting display service");
            Ok(())
        }
        
        async fn stop(&mut self) -> Result<()> {
            debug!("Stopping display service");
            Ok(())
        }
        
        async fn handle_message(&mut self, message: ServiceMessage) -> Result<()> {
            trace!("Display service handling message: {:?}", message.id);
            
            // Process message based on command ID
            match message.frame.command_id() {
                cmd if cmd == CommandId::DisplayInfo as u8 => {
                    // Parse display info and store it
                    // For now, just acknowledge receipt
                    if let Some(tx) = message.response_tx {
                        let response = Frame::new(CommandId::Ack as u8, Vec::new());
                        let _ = tx.send(Ok(response));
                    }
                }
                cmd if cmd == CommandId::StreamFrame as u8 => {
                    // Process frame data (e.g., decode and display)
                    // No response needed for streaming data
                }
                _ => {
                    debug!("Unknown command for display service: {:02x}", message.frame.command_id());
                    if let Some(tx) = message.response_tx {
                        let response = Frame::new(CommandId::Error as u8, b"Unknown command".to_vec());
                        let _ = tx.send(Ok(response));
                    }
                }
            }
            
            Ok(())
        }
    }
    
    /// Input service implementation
    pub struct InputService {}
    
    impl InputService {
        /// Create a new input service
        pub fn new() -> Self {
            Self {}
        }
    }
    
    #[async_trait::async_trait]
    impl Service for InputService {
        async fn start(&mut self) -> Result<()> {
            debug!("Starting input service");
            Ok(())
        }
        
        async fn stop(&mut self) -> Result<()> {
            debug!("Stopping input service");
            Ok(())
        }
        
        async fn handle_message(&mut self, message: ServiceMessage) -> Result<()> {
            trace!("Input service handling message: {:?}", message.id);
            
            // Basic acknowledgment for now
            if let Some(tx) = message.response_tx {
                let response = Frame::new(CommandId::Ack as u8, Vec::new());
                let _ = tx.send(Ok(response));
            }
            
            Ok(())
        }
    }
    
    /// Clipboard service implementation
    pub struct ClipboardService {}
    
    impl ClipboardService {
        /// Create a new clipboard service
        pub fn new() -> Self {
            Self {}
        }
    }
    
    #[async_trait::async_trait]
    impl Service for ClipboardService {
        async fn start(&mut self) -> Result<()> {
            debug!("Starting clipboard service");
            Ok(())
        }
        
        async fn stop(&mut self) -> Result<()> {
            debug!("Stopping clipboard service");
            Ok(())
        }
        
        async fn handle_message(&mut self, message: ServiceMessage) -> Result<()> {
            trace!("Clipboard service handling message: {:?}", message.id);
            
            // Basic acknowledgment for now
            if let Some(tx) = message.response_tx {
                let response = Frame::new(CommandId::Ack as u8, Vec::new());
                let _ = tx.send(Ok(response));
            }
            
            Ok(())
        }
    }
    
    /// File transfer service implementation
    pub struct FileTransferService {}
    
    impl FileTransferService {
        /// Create a new file transfer service
        pub fn new() -> Self {
            Self {}
        }
    }
    
    #[async_trait::async_trait]
    impl Service for FileTransferService {
        async fn start(&mut self) -> Result<()> {
            debug!("Starting file transfer service");
            Ok(())
        }
        
        async fn stop(&mut self) -> Result<()> {
            debug!("Stopping file transfer service");
            Ok(())
        }
        
        async fn handle_message(&mut self, message: ServiceMessage) -> Result<()> {
            trace!("File transfer service handling message: {:?}", message.id);
            
            // Basic acknowledgment for now
            if let Some(tx) = message.response_tx {
                let response = Frame::new(CommandId::Ack as u8, Vec::new());
                let _ = tx.send(Ok(response));
            }
            
            Ok(())
        }
    }
}