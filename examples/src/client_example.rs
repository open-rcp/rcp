use futures_util::StreamExt;
use rcp_client::{
    Client, ClientConfig, ClientEvent, ClipboardService, DisplayService, InputService,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    
    // Create client configuration
    let config = ClientConfig {
        host: "localhost".to_string(),
        port: 8716,
        client_id: Uuid::new_v4(),
        client_name: "RCP Example Client".to_string(),
        auth_method: rcp_client::AuthMethod::PreSharedKey,
        psk: Some("test_key".to_string()),
        timeout_secs: 10,
        ..Default::default()
    };
    
    // Create client
    let mut client = Client::new(config);
    println!("Connecting to RCP server...");
    
    // Connect to server
    match client.connect().await {
        Ok(_) => println!("Connected to server"),
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Ok(());
        }
    }
    
    // Authenticate
    println!("Authenticating...");
    match client.authenticate().await {
        Ok(session) => {
            println!("Authentication successful!");
            println!("Session ID: {}", session.session_id);
            println!("Permissions: {:?}", session.permissions);
        }
        Err(e) => {
            eprintln!("Authentication failed: {}", e);
            return Ok(());
        }
    }
    
    // Create client reference for services
    let client_arc = Arc::new(Mutex::new(client));
    
    // Create services
    let display_service = DisplayService::new(Arc::clone(&client_arc));
    let input_service = InputService::new(Arc::clone(&client_arc));
    let clipboard_service = ClipboardService::new(Arc::clone(&client_arc));
    
    // Subscribe to services
    println!("Subscribing to services...");
    if let Err(e) = display_service.subscribe().await {
        eprintln!("Failed to subscribe to display service: {}", e);
    }
    
    if let Err(e) = input_service.subscribe().await {
        eprintln!("Failed to subscribe to input service: {}", e);
    }
    
    if let Err(e) = clipboard_service.subscribe().await {
        eprintln!("Failed to subscribe to clipboard service: {}", e);
    }
    
    // Set display quality
    println!("Setting display quality...");
    if let Err(e) = display_service.set_quality(90).await {
        eprintln!("Failed to set display quality: {}", e);
    }
    
    // Send some mouse movements for demonstration
    println!("Simulating mouse movements...");
    for i in 0..5 {
        if let Err(e) = input_service.send_mouse_move(i * 100, i * 100).await {
            eprintln!("Failed to send mouse movement: {}", e);
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // Send clipboard data
    println!("Sending clipboard data...");
    if let Err(e) = clipboard_service.send_clipboard("RCP example clipboard data").await {
        eprintln!("Failed to send clipboard data: {}", e);
    }
    
    // Process events for a while
    println!("Processing events for 10 seconds...");
    let mut event_count = 0;
    
    {
        let mut client = client_arc.lock().await;
        let mut event_receiver = client.event_receiver();
        
        // Set a timeout for event processing
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        
        loop {
            tokio::select! {
                Some(event) = event_receiver.next() => {
                    match &event {
                        ClientEvent::FrameReceived(frame) => {
                            println!("Received frame: command={:02x}, size={} bytes", 
                                    frame.command_id(), frame.payload().len());
                            event_count += 1;
                        }
                        ClientEvent::StateChanged(state) => {
                            println!("Client state changed: {:?}", state);
                        }
                        ClientEvent::Error(error) => {
                            println!("Error: {}", error);
                        }
                        ClientEvent::Disconnected(reason) => {
                            println!("Disconnected: {:?}", reason);
                            break;
                        }
                        _ => {}
                    }
                }
                _ = interval.tick() => {
                    println!("Timeout reached, processed {} events", event_count);
                    break;
                }
            }
        }
    }
    
    // Unsubscribe from services
    println!("Unsubscribing from services...");
    if let Err(e) = display_service.unsubscribe().await {
        eprintln!("Failed to unsubscribe from display service: {}", e);
    }
    
    if let Err(e) = input_service.unsubscribe().await {
        eprintln!("Failed to unsubscribe from input service: {}", e);
    }
    
    if let Err(e) = clipboard_service.unsubscribe().await {
        eprintln!("Failed to unsubscribe from clipboard service: {}", e);
    }
    
    // Disconnect
    println!("Disconnecting...");
    let mut client = client_arc.lock().await;
    if let Err(e) = client.disconnect().await {
        eprintln!("Failed to disconnect: {}", e);
    }
    
    println!("Client example completed");
    Ok(())
}