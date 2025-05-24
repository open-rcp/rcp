# RCP Protocol (rcpcore)

## Overview

The RCP Protocol library (`rcpcore`) is the core component that defines the binary protocol format, data structures, and communication patterns used throughout the RCP ecosystem. It provides the foundation upon which the client and server components are built.

## Features

- Binary frame format definition and parsing
- Header structure with version control
- Authentication methods (PSK and future methods)
- Command structure and encoding/decoding
- Protocol state machine for connection lifecycle
- Utility functions for protocol-related operations

## Components

### Frame Handling

The `Frame` struct and associated methods handle the binary serialization and deserialization of protocol messages. Each frame consists of a header and payload.

```rust
pub struct Frame {
    pub header: Header,
    pub payload: Vec<u8>,
}
```

### Authentication

The auth module provides several authentication methods:

```rust
pub enum AuthMethod {
    None,
    PreSharedKey,
    PublicKey,
}
```

Authentication flow consists of challenge-response sequences to establish secure sessions.

### Commands

Commands represent specific operations within the protocol:

```rust
pub enum Command {
    LaunchApp(LaunchAppCommand),
    SendInput(InputCommand),
    StreamFrame(FrameData),
    // ... additional commands
}
```

### Protocol State Machine

The protocol module manages the connection state transitions:

```rust
pub enum ConnectionState {
    Connected,
    Authenticating,
    Authenticated,
    Closing,
    Closed,
}
```

## Integration

The RCP Protocol library is used by both the client (`rcpcli`) and daemon (`rcpdaemon`) components. It ensures consistent message handling and protocol behavior across components.

## Extension Points

The protocol is designed for extensibility through:

1. Version field in the header for protocol evolution
2. Command ID namespace for adding new command types
3. Flag fields for optional features
