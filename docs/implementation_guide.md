# RCP Implementation Guide

This guide provides detailed instructions for implementing and using the Rust Control Protocol (RCP) in your applications.

## Table of Contents

1. [Introduction](#introduction)
2. [Architecture Overview](#architecture-overview)
3. [Client Implementation](#client-implementation)
4. [Server Implementation](#server-implementation)
5. [Authentication](#authentication)
6. [Services](#services)
7. [Error Handling](#error-handling)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

## Introduction

The Rust Control Protocol (RCP) is a lightweight, secure protocol for remote control and service sharing between applications. It is designed to be efficient, extensible, and easy to integrate into Rust applications.

Key features:
- Binary protocol with minimal overhead
- Secure authentication mechanisms
- Extensible service architecture
- Efficient data streaming
- Reconnection support
- Cross-platform compatibility

## Architecture Overview

RCP follows a client-server model with the following key components:

1. **Core Protocol**: Defines the basic message format, framing, and protocol state machine.
2. **Authentication**: Mechanisms for authenticating clients with the server.
3. **Services**: Pluggable components that provide specific functionality.
4. **Session Management**: Handling of client sessions and permissions.

### Protocol Flow

1. Client connects to server
2. Client authenticates with server
3. Server creates a session for the client
4. Client subscribes to services
5. Client and server exchange service-specific messages
6. Client unsubscribes from services
7. Client disconnects

## Client Implementation

### Basic Client Usage

```rust
use rcp_client::{Client, ClientConfig, AuthMethod};
use std::sync::Arc;
use tokio::sync::Mutex;

async fn example_client() -> Result<(), Box<dyn std::error::Error>> {
    // Create client configuration
    let config = ClientConfig {
        host: "example.com".to_string(),
        port: 8716,
        auth_method: AuthMethod::PreSharedKey,
        psk: Some("your_secret_key".to_string()),
        ..Default::default()
    };
    
    // Create and connect client
    let mut client = Client::new(config);
    client.connect().await?;
    
    // Authenticate
    let session = client.authenticate().await?;
    println!("Connected with session ID: {}", session.session_id);
    
    // Create shared client reference for services
    let client_arc = Arc::new(Mutex::new(client));
    
    // Work with services...
    
    // Disconnect when done
    let mut client = client_arc.lock().await;
    client.disconnect().await?;
    
    Ok(())
}
```

### Working with Services

```rust
use rcp_client::{DisplayService, InputService, ClipboardService};

async fn use_services(client_arc: Arc<Mutex<Client>>) -> Result<(), Box<dyn std::error::Error>> {
    // Create service clients
    let display = DisplayService::new(Arc::clone(&client_arc));
    let input = InputService::new(Arc::clone(&client_arc));
    let clipboard = ClipboardService::new(Arc::clone(&client_arc));
    
    // Subscribe to services
    display.subscribe().await?;
    input.subscribe().await?;
    clipboard.subscribe().await?;
    
    // Use display service
    display.set_quality(90).await?;
    
    // Use input service
    input.send_mouse_move(100, 200).await?;
    input.send_key(0x41, true).await?;  // Press 'A'
    input.send_key(0x41, false).await?; // Release 'A'
    
    // Use clipboard service
    clipboard.send_clipboard("Shared clipboard text").await?;
    
    // Unsubscribe when done
    display.unsubscribe().await?;
    input.unsubscribe().await?;
    clipboard.unsubscribe().await?;
    
    Ok(())
}
```

### Handling Events

```rust
use futures_util::StreamExt;
use rcp_client::ClientEvent;

async fn handle_events(client: &mut Client) {
    let mut receiver = client.event_receiver();
    
    while let Some(event) = receiver.next().await {
        match event {
            ClientEvent::StateChanged(state) => {
                println!("Client state changed to {:?}", state);
            }
            ClientEvent::FrameReceived(frame) => {
                println!("Received frame: command={:02x}, size={} bytes",
                         frame.command_id(), frame.payload().len());
                
                // Handle specific frame types
                if frame.command_id() == rcp_client::CommandId::VideoFrame as u8 {
                    // Process video frame...
                }
            }
            ClientEvent::Disconnected(reason) => {
                println!("Disconnected: {:?}", reason);
                break;
            }
            ClientEvent::Error(error) => {
                println!("Error: {}", error);
            }
            _ => {}
        }
    }
}
```

## Server Implementation

### Basic Server Setup

```rust
use rcp_server::{Server, ServerConfig, AuthMethod, AuthConfig};

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Configure authentication
    let auth_config = AuthConfig::new()
        .with_psk("your_secret_key")
        .with_allowed_methods(&[AuthMethod::PreSharedKey]);
    
    // Create server configuration
    let config = ServerConfig {
        bind_address: "0.0.0.0".to_string(),
        port: 8716,
        auth_config,
        ..Default::default()
    };
    
    // Create and start server
    let server = Server::new(config);
    server.start().await?;
    
    // Wait for server to stop
    server.wait().await;
    
    Ok(())
}
```

### Implementing Services

```rust
use rcp_server::{Service, ServiceContext, ServiceConfig, Session, Frame};
use async_trait::async_trait;

struct MyCustomService {
    // Service state...
}

#[async_trait]
impl Service for MyCustomService {
    async fn initialize(&mut self, _config: ServiceConfig) -> rcp_server::Result<()> {
        // Initialize service
        Ok(())
    }
    
    async fn client_subscribed(&mut self, session: &Session, ctx: &mut ServiceContext) -> rcp_server::Result<()> {
        // Handle new client subscription
        println!("Client {} subscribed to service", session.client_name);
        Ok(())
    }
    
    async fn handle_frame(&mut self, frame: Frame, session: &Session, ctx: &mut ServiceContext) -> rcp_server::Result<()> {
        // Handle client request
        println!("Received frame from client {}: {:02x}", session.client_name, frame.command_id());
        
        // Send response if needed
        let response = Frame::new(0x42, b"response data".to_vec());
        ctx.send_frame(response, session).await?;
        
        Ok(())
    }
    
    async fn client_unsubscribed(&mut self, session: &Session, _ctx: &mut ServiceContext) -> rcp_server::Result<()> {
        // Handle client unsubscription
        println!("Client {} unsubscribed from service", session.client_name);
        Ok(())
    }
}
```

### Registering Services with the Server

```rust
async fn register_services(server: &mut Server) -> Result<(), Box<dyn std::error::Error>> {
    // Register built-in services
    server.register_service("display", Box::new(DisplayService::new())).await?;
    server.register_service("input", Box::new(InputService::new())).await?;
    server.register_service("clipboard", Box::new(ClipboardService::new())).await?;
    
    // Register custom services
    server.register_service("custom", Box::new(MyCustomService{})).await?;
    
    Ok(())
}
```

## Authentication

RCP supports multiple authentication mechanisms:

### Pre-Shared Key (PSK)

The simplest authentication method using a shared secret key.

**Server configuration:**
```rust
let auth_config = AuthConfig::new()
    .with_psk("your_secret_key")
    .with_allowed_methods(&[AuthMethod::PreSharedKey]);
```

**Client configuration:**
```rust
let config = ClientConfig {
    auth_method: AuthMethod::PreSharedKey,
    psk: Some("your_secret_key".to_string()),
    ..Default::default()
};
```

### Certificate-Based (Future Implementation)

For enhanced security, certificate-based authentication will be supported in future versions.

## Services

RCP includes several built-in services:

### Display Service

Handles screen capture and streaming from server to client.

```rust
// Client usage
let display = DisplayService::new(client_arc);
display.subscribe().await?;
display.set_quality(90).await?; // Set quality level (0-100)
```

### Input Service

Handles keyboard and mouse input from client to server.

```rust
// Client usage
let input = InputService::new(client_arc);
input.subscribe().await?;
input.send_key(0x41, true).await?;  // Press 'A'
input.send_key(0x41, false).await?; // Release 'A'
input.send_mouse_move(100, 200).await?;
input.send_mouse_button(1, true).await?;  // Press left button
input.send_mouse_button(1, false).await?; // Release left button
```

### Clipboard Service

Syncs clipboard contents between client and server.

```rust
// Client usage
let clipboard = ClipboardService::new(client_arc);
clipboard.subscribe().await?;
clipboard.send_clipboard("Shared clipboard text").await?;
```

### Custom Services

You can implement your own services by implementing the `Service` trait.

## Error Handling

RCP uses proper error types and propagation throughout the codebase:

```rust
use rcp_client::Error;

async fn handle_errors() -> Result<(), Error> {
    let mut client = Client::new(ClientConfig::default());
    
    match client.connect().await {
        Ok(_) => println!("Connected successfully"),
        Err(Error::Connection(msg)) => println!("Connection error: {}", msg),
        Err(Error::Timeout(msg)) => println!("Connection timed out: {}", msg),
        Err(e) => println!("Other error: {}", e),
    }
    
    Ok(())
}
```

## Best Practices

1. **Authentication**: Always use secure authentication methods and protect credentials.
2. **Error Handling**: Implement proper error handling and recovery mechanisms.
3. **Reconnection**: Enable automatic reconnection for better user experience during network issues.
4. **Resource Management**: Properly close connections and unsubscribe from services when done.
5. **Permissions**: Implement appropriate permissions on the server to restrict client actions.

## Examples

See the `examples/` directory for complete working examples:

- `client_example.rs`: Basic RCP client usage
- `server_example.rs`: Basic RCP server setup
- `custom_service.rs`: Implementing a custom service

---

For more information, refer to the [Architecture Overview](architecture.md) and [Protocol Specification](spec.md) documents.