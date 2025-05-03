# RCP Implementation Guide

This document provides guidance on implementing components of the Rust Control Protocol (RCP) system.

## Getting Started

### Environment Setup

1. Ensure Rust 1.75+ is installed:
   ```bash
   rustup update stable
   ```

2. Required dependencies:
   - `tokio` for async networking
   - `bytes` for efficient byte manipulation
   - `thiserror` for error handling
   - `rustls` for TLS support

### Project Structure Recommendations

Follow this structure for RCP components:

```
src/
├── lib.rs         # Public API exports
├── error.rs       # Error definitions
├── protocol/      # Protocol implementation
│   ├── frame.rs   # Frame definitions
│   ├── header.rs  # Header parsing/serialization
│   └── commands/  # Command implementations
├── transport/     # Transport layer
│   ├── tcp.rs     # TCP implementation
│   └── tls.rs     # TLS implementation
└── utils/         # Utility functions
```

## Core Protocol Implementation

### Frame Parsing

Implement frame parsing with careful attention to binary format:

```rust
pub struct RcpHeader {
    version: u8,
    command: u8,
    payload_size: u32,
    flags: u16,
}

impl RcpHeader {
    pub fn parse(input: &[u8]) -> Result<Self, RcpError> {
        if input.len() < HEADER_SIZE {
            return Err(RcpError::InvalidHeader);
        }
        
        Ok(Self {
            version: input[0],
            command: input[1],
            payload_size: u32::from_le_bytes([input[2], input[3], input[4], input[5]]),
            flags: u16::from_le_bytes([input[6], input[7]]),
        })
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(HEADER_SIZE);
        buffer.push(self.version);
        buffer.push(self.command);
        buffer.extend_from_slice(&self.payload_size.to_le_bytes());
        buffer.extend_from_slice(&self.flags.to_le_bytes());
        buffer
    }
}
```

### Implementing Commands

Each command should implement the `RcpCommand` trait:

```rust
pub trait RcpCommand {
    fn command_id(&self) -> u8;
    fn serialize(&self) -> Result<Vec<u8>, RcpError>;
    fn parse(payload: &[u8]) -> Result<Self, RcpError> where Self: Sized;
}

// Example implementation for LaunchApp command
pub struct LaunchAppCommand {
    pub flags: u32,
    pub application_path: String,
}

impl RcpCommand for LaunchAppCommand {
    fn command_id(&self) -> u8 {
        0x01 // LaunchApp command ID
    }
    
    fn serialize(&self) -> Result<Vec<u8>, RcpError> {
        let path_bytes = self.application_path.as_bytes();
        let path_len = path_bytes.len() as u32;
        
        let mut buffer = Vec::with_capacity(8 + path_bytes.len());
        buffer.extend_from_slice(&self.flags.to_le_bytes());
        buffer.extend_from_slice(&path_len.to_le_bytes());
        buffer.extend_from_slice(path_bytes);
        
        Ok(buffer)
    }
    
    fn parse(payload: &[u8]) -> Result<Self, RcpError> {
        if payload.len() < 8 {
            return Err(RcpError::InvalidPayload);
        }
        
        let flags = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
        let path_len = u32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]) as usize;
        
        if payload.len() < 8 + path_len {
            return Err(RcpError::InvalidPayload);
        }
        
        let path = String::from_utf8(payload[8..8+path_len].to_vec())
            .map_err(|_| RcpError::InvalidUtf8)?;
            
        Ok(Self {
            flags,
            application_path: path,
        })
    }
}
```

## Server Implementation

### Connection Handling

Use Tokio's async/await for efficient connection handling:

```rust
pub async fn run_server(addr: SocketAddr) -> Result<(), RcpError> {
    let listener = TcpListener::bind(addr).await?;
    
    loop {
        let (socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream) -> Result<(), RcpError> {
    let mut session = RcpSession::new(socket);
    
    // Handle authentication
    if !session.authenticate().await? {
        return Err(RcpError::AuthenticationFailed);
    }
    
    // Main connection loop
    loop {
        match session.read_frame().await? {
            Some(frame) => {
                process_frame(&mut session, frame).await?;
            }
            None => {
                // Connection closed
                break;
            }
        }
    }
    
    Ok(())
}
```

### Application Management

Implement secure application launching:

```rust
pub async fn launch_application(
    cmd: &LaunchAppCommand,
    user_context: &UserContext,
) -> Result<ApplicationHandle, RcpError> {
    // Validate application path against allowed list
    if !is_application_allowed(&cmd.application_path, user_context) {
        return Err(RcpError::ApplicationNotAllowed);
    }
    
    // Launch the application
    let mut command = Command::new(&cmd.application_path);
    
    // Set appropriate environment and security context
    // ...
    
    let process = command.spawn()?;
    
    Ok(ApplicationHandle::new(process))
}
```

## Client Implementation

### Connection Establishment

```rust
pub async fn connect(
    server_addr: SocketAddr,
    credentials: Credentials,
) -> Result<RcpClient, RcpError> {
    let socket = TcpStream::connect(server_addr).await?;
    
    let mut client = RcpClient::new(socket);
    if !client.authenticate(credentials).await? {
        return Err(RcpError::AuthenticationFailed);
    }
    
    Ok(client)
}
```

### Screen Capture Processing

```rust
pub async fn process_screen_frame(
    frame_data: &[u8],
    width: u32,
    height: u32,
    format: FrameFormat,
) -> Result<Image, RcpError> {
    // Decode the image data based on format
    // ...
    
    // Return image for display
    // ...
}
```

## Testing Strategy

### Unit Testing

Write comprehensive unit tests for protocol parsers:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_parsing() {
        let data = [
            0x01, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        
        let header = RcpHeader::parse(&data).unwrap();
        
        assert_eq!(header.version, 0x01);
        assert_eq!(header.command, 0x02);
        assert_eq!(header.payload_size, 16);
        assert_eq!(header.flags, 0);
    }
}
```

### Integration Testing

Set up integration tests with in-memory transports:

```rust
#[tokio::test]
async fn test_client_server_interaction() {
    // Create in-memory transport pair
    let (client_transport, server_transport) = create_in_memory_transport_pair();
    
    // Set up server
    let server = TestServer::new(server_transport);
    let server_handle = tokio::spawn(async move {
        server.run().await.unwrap();
    });
    
    // Set up client
    let client = RcpClient::new(client_transport);
    
    // Test interaction
    let cmd = LaunchAppCommand {
        flags: 0,
        application_path: "notepad.exe".to_string(),
    };
    
    let result = client.send_command(&cmd).await;
    assert!(result.is_ok());
    
    // Shut down
    drop(client);
    server_handle.await.unwrap();
}
```

## Performance Optimization

- Use zero-copy operations where possible
- Implement buffer pooling for frame handling
- Consider compressed formats for screen streaming
- Use shared memory for local connections

## Security Best Practices

- Always validate user input before parsing
- Use allowlists for application launching
- Implement rate limiting for authentication attempts
- Regular security audits of the codebase