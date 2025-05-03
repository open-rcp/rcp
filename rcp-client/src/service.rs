use crate::client::Client;
use crate::error::{Error, Result};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use rcp_core::{CommandId, Frame};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

/// Event handler for services
#[async_trait]
pub trait ServiceEventHandler: Send + Sync {
    /// Handle a frame received for this service
    async fn handle_frame(&self, frame: Frame) -> Result<()>;
}

/// Client service
pub struct Service {
    /// Service name
    name: String,
    
    /// Service ID
    id: Uuid,
    
    /// The client
    client: Arc<Mutex<Client>>,
    
    /// Event handler
    event_handler: Option<Box<dyn ServiceEventHandler>>,
    
    /// Event receiver
    event_receiver: mpsc::Receiver<Frame>,
    
    /// Event sender
    event_sender: mpsc::Sender<Frame>,
}

impl Service {
    /// Create a new service
    pub fn new(name: String, client: Arc<Mutex<Client>>) -> Self {
        let (tx, rx) = mpsc::channel(100);
        
        Self {
            name,
            id: Uuid::new_v4(),
            client,
            event_handler: None,
            event_receiver: rx,
            event_sender: tx,
        }
    }
    
    /// Set the event handler for this service
    pub fn set_event_handler(&mut self, handler: Box<dyn ServiceEventHandler>) {
        self.event_handler = Some(handler);
    }
    
    /// Subscribe to the service on the server
    pub async fn subscribe(&self) -> Result<()> {
        let mut client = self.client.lock().await;
        
        // Send subscription frame
        let payload = self.name.as_bytes().to_vec();
        let frame = Frame::new(CommandId::ServiceSubscribe as u8, payload);
        
        client.send_frame(frame).await?;
        info!("Subscribed to service: {}", self.name);
        
        Ok(())
    }
    
    /// Unsubscribe from the service
    pub async fn unsubscribe(&self) -> Result<()> {
        let mut client = self.client.lock().await;
        
        // Send unsubscription frame
        let payload = self.name.as_bytes().to_vec();
        let frame = Frame::new(CommandId::ServiceUnsubscribe as u8, payload);
        
        client.send_frame(frame).await?;
        info!("Unsubscribed from service: {}", self.name);
        
        Ok(())
    }
    
    /// Send a frame to the service
    pub async fn send_frame(&self, frame: Frame) -> Result<()> {
        let mut client = self.client.lock().await;
        client.send_frame(frame).await
    }
    
    /// Process a frame received from the server
    pub async fn process_frame(&self, frame: Frame) -> Result<()> {
        if let Some(handler) = &self.event_handler {
            handler.handle_frame(frame).await?;
        }
        
        Ok(())
    }
}

/// Display service client
pub struct DisplayService {
    /// The underlying service
    service: Service,
    
    /// Frame receiver for display updates
    frame_receiver: mpsc::Receiver<Frame>,
    
    /// Frame sender for display updates
    frame_sender: mpsc::Sender<Frame>,
}

impl DisplayService {
    /// Create a new display service
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        let service = Service::new("display".to_string(), client);
        let (tx, rx) = mpsc::channel(100);
        
        Self {
            service,
            frame_receiver: rx,
            frame_sender: tx,
        }
    }
    
    /// Subscribe to the display service
    pub async fn subscribe(&self) -> Result<()> {
        self.service.subscribe().await
    }
    
    /// Unsubscribe from the display service
    pub async fn unsubscribe(&self) -> Result<()> {
        self.service.unsubscribe().await
    }
    
    /// Set the display quality
    pub async fn set_quality(&self, quality: u8) -> Result<()> {
        let frame = Frame::new(CommandId::VideoQuality as u8, vec![quality]);
        self.service.send_frame(frame).await
    }
}

/// Input service client
pub struct InputService {
    /// The underlying service
    service: Service,
}

impl InputService {
    /// Create a new input service
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        let service = Service::new("input".to_string(), client);
        
        Self {
            service,
        }
    }
    
    /// Subscribe to the input service
    pub async fn subscribe(&self) -> Result<()> {
        self.service.subscribe().await
    }
    
    /// Unsubscribe from the input service
    pub async fn unsubscribe(&self) -> Result<()> {
        self.service.unsubscribe().await
    }
    
    /// Send a key press event
    pub async fn send_key(&self, key_code: u16, pressed: bool) -> Result<()> {
        let mut payload = Vec::with_capacity(3);
        payload.push(1); // Event type: key
        payload.extend_from_slice(&key_code.to_le_bytes());
        payload.push(if pressed { 1 } else { 0 });
        
        let frame = Frame::new(CommandId::SendInput as u8, payload);
        self.service.send_frame(frame).await
    }
    
    /// Send a mouse move event
    pub async fn send_mouse_move(&self, x: u16, y: u16) -> Result<()> {
        let mut payload = Vec::with_capacity(5);
        payload.push(2); // Event type: mouse move
        payload.extend_from_slice(&x.to_le_bytes());
        payload.extend_from_slice(&y.to_le_bytes());
        
        let frame = Frame::new(CommandId::SendInput as u8, payload);
        self.service.send_frame(frame).await
    }
    
    /// Send a mouse button event
    pub async fn send_mouse_button(&self, button: u8, pressed: bool) -> Result<()> {
        let mut payload = Vec::with_capacity(3);
        payload.push(3); // Event type: mouse button
        payload.push(button);
        payload.push(if pressed { 1 } else { 0 });
        
        let frame = Frame::new(CommandId::SendInput as u8, payload);
        self.service.send_frame(frame).await
    }
    
    /// Send a mouse wheel event
    pub async fn send_mouse_wheel(&self, delta: i16) -> Result<()> {
        let mut payload = Vec::with_capacity(3);
        payload.push(4); // Event type: mouse wheel
        payload.extend_from_slice(&delta.to_le_bytes());
        
        let frame = Frame::new(CommandId::SendInput as u8, payload);
        self.service.send_frame(frame).await
    }
}

/// Clipboard service client
pub struct ClipboardService {
    /// The underlying service
    service: Service,
}

impl ClipboardService {
    /// Create a new clipboard service
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        let service = Service::new("clipboard".to_string(), client);
        
        Self {
            service,
        }
    }
    
    /// Subscribe to the clipboard service
    pub async fn subscribe(&self) -> Result<()> {
        self.service.subscribe().await
    }
    
    /// Unsubscribe from the clipboard service
    pub async fn unsubscribe(&self) -> Result<()> {
        self.service.unsubscribe().await
    }
    
    /// Send clipboard data to the server
    pub async fn send_clipboard(&self, data: &str) -> Result<()> {
        let payload = data.as_bytes().to_vec();
        let frame = Frame::new(CommandId::ClipboardData as u8, payload);
        
        self.service.send_frame(frame).await
    }
}