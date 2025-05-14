# RCP Core Library

The RCP Core library provides the foundational components for the Rust/Remote Control Protocol (RCP) system. It defines the protocol specification, framing format, authentication mechanisms, and command structures used throughout the entire RCP ecosystem.

## Purpose

This library serves as the shared foundation for all RCP components, ensuring protocol compatibility between servers and clients. Both RCP server implementations and client libraries depend on this core to establish consistent communication.

## Key Features

- **Protocol Definition**: Standardized communication protocol
- **Frame Format**: Binary framing format for efficient data transfer
- **Authentication System**: Secure authentication mechanisms
- **Command Structure**: Extensible command system
- **Error Handling**: Comprehensive error types

## Architecture Design

The RCP Core is designed with the following principles:

- **Versatile Deployment**: Supports both individual use and data center/SaaS models
- **Application Isolation**: Users only access their assigned applications and directories
- **Standard Core**: Remains fully open source and identical across all deployment models
- **Security First**: Authentication and authorization are non-optional
- **Centralized Management**: All application management happens server-side
- **Client Isolation**: End users only access their reserved applications and directories
- **Clustering Support**: Multiple servers can be managed by a single admin or gateway

## Components

- `protocol.rs` - Main protocol implementation
- `frame.rs` - Binary frame format
- `header.rs` - Frame header structure
- `command.rs` - Command definitions
- `auth.rs` - Authentication system
- `error.rs` - Error types
- `utils.rs` - Utility functions

## Usage

This library is not meant to be used directly by applications but serves as a dependency for RCP server, client, and bridge implementations.

```rust
use rcpp::{Frame, Header, Command, Auth, Protocol};
```
