use crate::error::Result;
use async_trait::async_trait;
use rcp_core::Frame;
use std::fmt::Debug;

/// Service trait for implementing different RCP services
#[async_trait]
pub trait Service: Debug + Send + Sync {
    /// Get the name of the service
    fn name(&self) -> &str;

    /// Start the service
    async fn start(&mut self) -> Result<()>;

    /// Stop the service
    async fn stop(&mut self) -> Result<()>;

    /// Process an incoming frame from the client
    async fn process_frame(&mut self, frame: &Frame) -> Result<()>;

    /// Get a frame from the service (if available)
    async fn get_frame(&mut self) -> Result<Option<Frame>>;
}

/// Factory for creating service instances
pub mod services {
    use super::*;
    use crate::error::Error;
    use rcp_core::{CommandId, Frame};
    use std::time::Duration;
    use tokio::sync::mpsc::{self, Receiver, Sender};
    use uuid::Uuid;

    /// Display service for screen sharing
    #[derive(Debug)]
    pub struct DisplayService {
        /// Service name
        name: String,

        /// Service ID
        #[allow(dead_code)]
        id: Uuid,

        /// Sender for outgoing frames
        frame_sender: Sender<Frame>,

        /// Receiver for outgoing frames
        frame_receiver: Receiver<Frame>,

        /// Whether the service is running
        running: bool,
    }

    impl Default for DisplayService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl DisplayService {
        /// Create a new display service
        pub fn new() -> Self {
            let (tx, rx) = mpsc::channel(100);

            Self {
                name: "display".to_string(),
                id: Uuid::new_v4(),
                frame_sender: tx,
                frame_receiver: rx,
                running: false,
            }
        }
    }

    #[async_trait]
    impl Service for DisplayService {
        fn name(&self) -> &str {
            &self.name
        }

        async fn start(&mut self) -> Result<()> {
            if self.running {
                return Err(Error::Service(
                    "Display service already running".to_string(),
                ));
            }

            self.running = true;

            // TODO: Implement actual screen capture
            // For now, just send a dummy frame periodically for testing

            let sender = self.frame_sender.clone();
            tokio::spawn(async move {
                loop {
                    // Create a dummy frame with some metadata
                    let dummy_data =
                        r#"{"width":1280,"height":720,"format":"jpeg"}"#.as_bytes().to_vec();
                    let frame = Frame::new(CommandId::DisplayInfo as u8, dummy_data);

                    if sender.send(frame).await.is_err() {
                        break;
                    }

                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            });

            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Display service not running".to_string()));
            }

            self.running = false;

            // The channel will be dropped when this object is dropped
            Ok(())
        }

        async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
            // Display service mainly receives configuration frames from client
            match frame.command_id() {
                cmd if cmd == CommandId::VideoQuality as u8 => {
                    // TODO: Adjust quality based on frame content
                    log::info!("Received video quality settings");
                }
                cmd if cmd == CommandId::SubscribeDisplay as u8 => {
                    // TODO: Start streaming
                    log::info!("Received start stream request");
                }
                _ => {
                    return Err(Error::Protocol(format!(
                        "Display service: Unsupported command: {:02x}",
                        frame.command_id()
                    )));
                }
            }

            Ok(())
        }

        async fn get_frame(&mut self) -> Result<Option<Frame>> {
            if !self.running {
                return Err(Error::Service("Display service not running".to_string()));
            }

            match self.frame_receiver.try_recv() {
                Ok(frame) => Ok(Some(frame)),
                Err(mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(mpsc::error::TryRecvError::Disconnected) => Err(Error::Service(
                    "Display service channel disconnected".to_string(),
                )),
            }
        }
    }

    /// Input service for handling keyboard and mouse input
    #[derive(Debug)]
    pub struct InputService {
        /// Service name
        name: String,

        /// Service ID
        #[allow(dead_code)]
        id: Uuid,

        /// Whether the service is running
        running: bool,
    }

    impl Default for InputService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InputService {
        /// Create a new input service
        pub fn new() -> Self {
            Self {
                name: "input".to_string(),
                id: Uuid::new_v4(),
                running: false,
            }
        }
    }

    #[async_trait]
    impl Service for InputService {
        fn name(&self) -> &str {
            &self.name
        }

        async fn start(&mut self) -> Result<()> {
            if self.running {
                return Err(Error::Service("Input service already running".to_string()));
            }

            self.running = true;
            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Input service not running".to_string()));
            }

            self.running = false;
            Ok(())
        }

        async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Input service not running".to_string()));
            }

            match frame.command_id() {
                cmd if cmd == CommandId::SendInput as u8 => {
                    // TODO: Process input event
                    log::info!(
                        "Received input event, payload size: {}",
                        frame.payload().len()
                    );
                }
                _ => {
                    return Err(Error::Protocol(format!(
                        "Input service: Unsupported command: {:02x}",
                        frame.command_id()
                    )));
                }
            }

            Ok(())
        }

        async fn get_frame(&mut self) -> Result<Option<Frame>> {
            // Input service doesn't generate frames
            Ok(None)
        }
    }

    /// Clipboard service for synchronizing clipboard content
    #[derive(Debug)]
    pub struct ClipboardService {
        /// Service name
        name: String,

        /// Service ID
        #[allow(dead_code)]
        id: Uuid,

        /// Whether the service is running
        running: bool,
    }

    impl Default for ClipboardService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ClipboardService {
        /// Create a new clipboard service
        pub fn new() -> Self {
            Self {
                name: "clipboard".to_string(),
                id: Uuid::new_v4(),
                running: false,
            }
        }
    }

    #[async_trait]
    impl Service for ClipboardService {
        fn name(&self) -> &str {
            &self.name
        }

        async fn start(&mut self) -> Result<()> {
            if self.running {
                return Err(Error::Service(
                    "Clipboard service already running".to_string(),
                ));
            }

            self.running = true;
            // TODO: Setup clipboard monitoring
            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Clipboard service not running".to_string()));
            }

            self.running = false;
            Ok(())
        }

        async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Clipboard service not running".to_string()));
            }

            match frame.command_id() {
                cmd if cmd == CommandId::ClipboardData as u8 => {
                    // TODO: Process clipboard data from client
                    log::info!(
                        "Received clipboard data, size: {} bytes",
                        frame.payload().len()
                    );
                }
                _ => {
                    return Err(Error::Protocol(format!(
                        "Clipboard service: Unsupported command: {:02x}",
                        frame.command_id()
                    )));
                }
            }

            Ok(())
        }

        async fn get_frame(&mut self) -> Result<Option<Frame>> {
            // Return None for now, later this will return clipboard updates
            Ok(None)
        }
    }
}

/// Service factory for creating service instances
pub struct ServiceFactory;

impl ServiceFactory {
    /// Create a new service instance by name
    pub fn create(name: &str) -> Option<Box<dyn Service + Send>> {
        use self::services::{ClipboardService, DisplayService, InputService};

        match name {
            "display" => Some(Box::new(DisplayService::new())),
            "input" => Some(Box::new(InputService::new())),
            "clipboard" => Some(Box::new(ClipboardService::new())),
            _ => None,
        }
    }

    /// Get a list of available service names
    pub fn available_services() -> Vec<&'static str> {
        vec!["display", "input", "clipboard"]
    }
}
