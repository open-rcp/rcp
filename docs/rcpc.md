# RCP Client (rcpc)

## Overview

The RCP Client library (`rcpc`) provides a robust client implementation for connecting to RCPD servers. It manages connections, authentication, session handling, and provides both a library API and CLI interface for client operations.

## Features

- TCP connection management with auto-reconnect capabilities
- Client authentication using multiple methods
- Connection string parsing for simplified connections
- Service discovery and management
- Frame processing and dispatch
- Error handling and recovery
- Command-line interface for manual control

## Components

### Client Implementation

The core client implementation handles connection management, authentication, and message processing:

```rust
pub struct Client {
    // Internal state
    config: ClientConfig,
    connection: Option<Connection>,
    
    // Event channels
    event_tx: mpsc::Sender<ClientEvent>,
    event_rx: mpsc::Receiver<ClientEvent>,
    
    // Connection state
    state: ConnectionState,
}
```

### Connection String

The connection string module provides easy-to-use connection specifications:

```
rcp://[user[:password]@]host[:port][/app][?param=value[&param2=value2]]
```

Example:
```
rcp://admin@192.168.1.100:9277/notepad?fullscreen=true&auth=psk
```

### Service Client

The service client handles higher-level operations:

```rust
pub struct ServiceClient {
    // Client connection
    client: Client,
    
    // Service-specific functions
    services: HashMap<String, Box<dyn ServiceTrait>>,
    
    // Session information
    session_id: Option<Uuid>,
}
```

## Command-Line Interface

The client includes a command-line interface for manual operations:

```
$ rcpc connect rcp://server.example.com:9277
$ rcpc launch notepad
$ rcpc list apps
$ rcpc disconnect
```

## Integration

The RCP Client uses the RCP Protocol (`rcpp`) library for message format and protocol handling. It can be:

1. Used as a library in custom applications
2. Run as a command-line tool
3. Integrated into larger applications like RCP Desk

## Extension Points

The client is designed for extensibility through:

1. Pluggable authentication methods
2. Service trait implementation for custom services
3. Connection handlers for different transport protocols
