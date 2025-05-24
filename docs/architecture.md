# RCP Architecture Overview

## System Architecture

The Rust/Remote Control Protocol (RCP) is a modular service-oriented system with several components that work together. The architecture uses three main components: Protocol (rcpcore), Client (rcpcli), and Daemon (rcpdaemon).

```
┌─────────────┐        ┌───────────────────────────┐
│ RCP Client  │◄───────┤   rcpdaemon (RCP Daemon)       │
│  (rcpcli)     │        │   ┌──────────┐ ┌───────┐  │
└──────┬──────┘        │   │  Server  │ │  API  │  │
       │               │   └──────────┘ └───────┘  │
 ┌─────▼─────┐         └──────────────────────────┘
 │ User Apps │                    ▲
 └───────────┘                    │
                                  │
                         ┌────────┴───────┐
                         │  RCP Protocol  │
                         │     (rcpcore)     │
                         └────────────────┘
```

## Core Components

### 1. RCP Protocol Library (`rcpcore`)

- Protocol definitions and frame handling
- Authentication mechanisms
- Common utilities and data structures
- Core binary format specifications

### 2. RCP Client (`rcpcli`)

- Connects to RCP servers within the daemon
- Handles application control and user input
- Processes and displays streamed frames
- Provides client-side connection management

### 3. rcpdaemon (RCP Daemon) (`rcpdaemon`)

- Core component with integrated Server and API functionality
- Long-running daemon/system service architecture
- Handles connections, sessions, and protocol handling
- Provides RESTful endpoints for management (via "api" feature flag)
- Manages application lifecycle and configuration
- Provides specialized services for display, input, audio, clipboard, and file transfer
- Includes CLI functionality for service management

- Administrative web and desktop interface for server management
- Built with SvelteKit (web) and Tauri (desktop)

### 6. RCP Desk (`rcp-desk`)

- End-user client application for accessing virtual applications
- Built with SvelteKit and Tauri

### 7. WebSocket Bridge (`rcp-ws-bridge`)

- Bridges RCP protocol to WebSockets for browser clients
- Handles protocol translation and frame transcoding for web compatibility
- Enables web applications to connect without native clients

## Service Architecture

RCP uses a subscription-based service model where each client connection subscribes to specific services (display, input, clipboard, etc.) based on permissions and needs.

## Connection Lifecycle

1. Client connects to server over TCP/IP
2. Client authenticates using chosen method
3. Client subscribes to required services
4. Server tracks active sessions and handles reconnections
5. Client controls application launching and interaction
6. Connection terminates with graceful shutdown

## Security Architecture

- Transport Security: TLS encryption for all communications
- Authentication: Pre-shared keys, public-key authentication, and 2FA
- Authorization: Fine-grained permissions for different operations
- Session Isolation: Separation between different client sessions
- Audit Logging: Comprehensive activity logging for security monitoring

## Performance Considerations

- Binary protocol with minimal overhead
- Efficient frame structure with selective updates
- Optimized for low-latency operations
- Adaptive quality settings based on network conditions
- Connection quality monitoring

## Configuration Management

- Server configurations stored in TOML files
- Dynamic configuration updates without restart
- User permissions and application settings in structured storage

## Cross-Platform Strategy

All components are designed to work on:
- Windows
- macOS
- Linux (major distributions)

The client applications provide platform-specific optimizations while maintaining a consistent user experience.

## Extension Points

RCP is designed to be extensible:

- Pluggable authentication providers
- Custom command handlers
- Extensible frame types
- Middleware support
- Platform-specific adapters
- API extensions for third-party integration
- Plugin architecture for the admin and client interfaces