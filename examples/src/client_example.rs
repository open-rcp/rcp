use rcp_client::{
    Client, ClientState,
    service::{ServiceType}
};
use rcp_core::AuthMethod;
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
    
    println!("Connecting to RCP server...");
    
    // Create client using the builder pattern
    let client = Client::builder()
        .host("localhost")
        .port(8716)
        .client_name("RCP Example Client")
        .client_id(Uuid::new_v4())
        .auth_method(AuthMethod::PreSharedKey)
        .auth_psk("test_key")
        .connection_timeout(10)
        .build();
    
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
        Ok(_) => {
            println!("Authentication successful!");
            if let Some(session) = client.session_info().await {
                println!("Session ID: {}", session.session_id);
                println!("Permissions: {:?}", session.permissions);
            }
        }
        Err(e) => {
            eprintln!("Authentication failed: {}", e);
            return Ok(());
        }
    }
    
    // Start the client message processor
    client.start().await?;
    
    // Create client reference for services
    let client_arc = Arc::new(Mutex::new(client));
    
    // Subscribe to display service
    println!("Subscribing to display service...");
    let display_service = {
        let mut client = client_arc.lock().await;
        client.subscribe_service(ServiceType::Display).await?
    };
    
    // Subscribe to input service
    println!("Subscribing to input service...");
    let input_service = {
        let mut client = client_arc.lock().await;
        client.subscribe_service(ServiceType::Input).await?
    };
    
    // Subscribe to clipboard service
    println!("Subscribing to clipboard service...");
    let clipboard_service = {
        let mut client = client_arc.lock().await;
        client.subscribe_service(ServiceType::Clipboard).await?
    };
    
    // Send video quality request
    println!("Setting display quality...");
    let quality_frame = rcp_core::Frame::new(rcp_core::CommandId::VideoQuality as u8, vec![90]); // 90% quality
    if let Err(e) = display_service.send_fire_and_forget(quality_frame).await {
        eprintln!("Failed to set display quality: {}", e);
    }
    
    // Simulate mouse movements
    println!("Simulating mouse movements...");
    for i in 0..5 {
        let x = i * 100;
        let y = i * 100;
        
        // Create mouse move command
        let mouse_data = format!("{{\"type\":\"move\",\"x\":{},\"y\":{}}}", x, y).into_bytes();
        let mouse_frame = rcp_core::Frame::new(rcp_core::CommandId::SendInput as u8, mouse_data);
        
        if let Err(e) = input_service.send_fire_and_forget(mouse_frame).await {
            eprintln!("Failed to send mouse movement: {}", e);
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // Send clipboard data
    println!("Sending clipboard data...");
    let clipboard_data = "RCP example clipboard data".as_bytes().to_vec();
    let clipboard_frame = rcp_core::Frame::new(rcp_core::CommandId::ClipboardData as u8, clipboard_data);
    
    if let Err(e) = clipboard_service.send_fire_and_forget(clipboard_frame).await {
        eprintln!("Failed to send clipboard data: {}", e);
    }
    
    // Process events for a while
    println!("Running for 10 seconds...");
    
    // Create a task to monitor the connection state
    let client_state = client_arc.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let state = {
                let client = client_state.lock().await;
                client.state().await
            };
            println!("Current client state: {:?}", state);
            if state == ClientState::Disconnected {
                break;
            }
        }
    });
    
    // Wait for 10 seconds
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    // Unsubscribe from services
    println!("Cleanup and disconnecting...");
    
    // Disconnect
    let mut client = client_arc.lock().await;
    if let Err(e) = client.disconnect().await {
        eprintln!("Failed to disconnect: {}", e);
    }
    
    println!("Client example completed");
    Ok(())
}