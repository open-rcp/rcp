# RCP Architecture Overview

## System Architecture

The Rust Control Protocol (RCP) is organized as a modular service-oriented system with several components that work together:

```
┌─────────────┐     ┌─────────────┐       ┌─────────────┐
│  RCP Client │◄────┤ RCP Server  │◄──────┤ RCP Service │
└─────┬───────┘     └─────┬───────┘       └──────┬──────┘
      │                   │                      │
┌─────▼───────┐     ┌─────▼───────┐       ┌──────▼──────┐
│ Application │     │ Session Mgr │◄──────┤  RCP CLI    │
│   Control   │     └─────────────┘       └─────────────┘
└─────────────┘             │                    ▲
                    ┌───────▼────────┐    ┌──────┴──────┐
                    │  Application   │    │   RCP API   │
                    │   Processes    │    └──────┬──────┘
                    └────────────────┘           │
                                          ┌──────▼──────┐
                                          │   RCP Desk  │
                                          │ (Web/Tauri) │
                                          └─────────────┘
```

## Core Components

### 1. RCP Core Library (`rcp-core`)

The foundation of the RCP system providing:

- Protocol definitions
- Frame parsing and serialization
- Authentication mechanisms
- Common utilities

### 2. RCP Server (`rcp-server`)

- Listens for incoming TCP connections
- Manages authentication and sessions
- Spawns and controls applications
- Provides specialized services:
  - **Connection Service**: Handles core connection lifecycle
  - **Display Service**: Manages screen capture and display information 
  - **Input Service**: Processes remote keyboard and mouse events
  - **Audio Service**: Handles audio streaming
  - **Clipboard Service**: Manages clipboard synchronization
  - **File Transfer Service**: Handles file operations between peers

### 3. RCP Client (`rcp-client`)

- Connects to RCP servers
- Subscribes to required services
- Sends application control commands
- Handles user input
- Processes and displays streamed frames

### 4. RCP Service (`rcp-service`)

- Long-running daemon/service that manages RCP server instances
- Application lifecycle management
- Configuration handling and persistence
- System integration (startup service, user permissions)
- Logs and monitoring
- Communication channel with CLI and API

### 5. RCP CLI (`rcp-cli`)

- Command-line interface for server/service management
- Administrative tasks (user management, configuration)
- Server control (start/stop/restart)
- Session monitoring and management
- Status reporting and diagnostics
- Service installation/uninstallation

### 6. RCP API (`rcp-api`)

- RESTful API for remote management
- Authentication and authorization for admin access
- Server monitoring and management endpoints
- User and permission management
- Configuration management
- Session information and metrics
- Integration point for third-party systems

### 7. RCP Desk (`rcp-desk`)

- Unified administrative interface
- Built with SvelteKit for the web component
- Desktop application built with Tauri
- Server management and monitoring
- User management
- Session monitoring and control
- Configuration interface
- Analytics and reporting

### 8. WebSocket Bridge (`rcp-ws-bridge`)

An optional component that bridges RCP protocol to WebSockets for browser clients:

- Protocol translation
- Frame transcoding
- Web client interface

## Service Architecture

RCP uses a subscription-based service model:

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Display     │    │ Input       │    │ Clipboard   │
│ Service     │    │ Service     │    │ Service     │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                   │
       └──────────┬───────┴─────────┬─────────┘
                  │                 │
        ┌─────────▼─────────┐      ┌▼────────────────┐
        │  Connection #1    │      │  Connection #2   │
        │  (Subscribed to   │      │  (Subscribed to  │
        │   all services)   │      │   input only)    │
        └───────────────────┘      └─────────────────┘
```

Each client connection can subscribe to specific services based on permissions and needs.

## Data Flow

```
Client → Server Flow:
┌─────────┐    ┌──────────┐    ┌────────┐    ┌───────────┐
│ User    │ → │ Client    │ → │ Server │ → │ App        │
│ Input   │   │ Protocol  │   │ Session │   │ Process    │
└─────────┘    └──────────┘    └────────┘    └───────────┘

Server → Client Flow:
┌───────────┐    ┌────────┐    ┌──────────┐    ┌─────────┐
│ App       │ → │ Server  │ → │ Client    │ → │ Display  │
│ Output    │   │ Session │   │ Rendering │   │          │
└───────────┘    └────────┘    └──────────┘    └─────────┘

Management Flow:
┌─────────┐    ┌──────┐    ┌────────────┐    ┌──────────┐
│ RCP Desk │ → │ API  │ → │ RCP Service │ → │ RCP Server│
│         │   │      │   │            │   │          │
└─────────┘    └──────┘    └────────────┘    └──────────┘
```

## Runtime Service Architecture

The RCP Service provides a runtime management layer that orchestrates the overall system:

```
┌────────────────────────────────────────────────────────┐
│                     RCP Service                        │
│                                                        │
│  ┌─────────────┐  ┌────────────┐  ┌────────────────┐   │
│  │ Config      │  │ Server     │  │ App Lifecycle  │   │
│  │ Management  │  │ Management │  │ Management     │   │
│  └─────────────┘  └────────────┘  └────────────────┘   │
│                                                        │
│  ┌─────────────┐  ┌────────────┐  ┌────────────────┐   │
│  │ User        │  │ Session    │  │ Logging &      │   │
│  │ Management  │  │ Management │  │ Monitoring     │   │
│  └─────────────┘  └────────────┘  └────────────────┘   │
└────────────────────────────────────────────────────────┘
           ▲              ▲                 ▲
           │              │                 │
    ┌──────┴───────┐ ┌────┴─────┐    ┌─────┴─────┐
    │  RCP CLI     │ │ RCP API  │    │  RCP Desk  │
    └──────────────┘ └──────────┘    └───────────┘
```

## Connection Lifecycle

1. **Connection Establishment**: Client connects to server over TCP/IP
2. **Authentication**: Client authenticates using chosen method
3. **Service Subscription**: Client subscribes to required services
4. **Session Management**: Server tracks active sessions and handles reconnections
5. **Application Control**: Launching, interacting with, and terminating applications
6. **Connection Termination**: Graceful shutdown procedures

## Security Architecture

RCP implements security at multiple levels:

1. **Transport Security** - TLS encryption for all communications
2. **Authentication** - Multiple authentication mechanisms:
   - Pre-shared keys
   - Public-key authentication
   - Two-factor authentication
3. **Authorization** - Fine-grained permissions for different operations:
   - Input control
   - Clipboard access
   - Audio streaming
   - File transfer
4. **Session Isolation** - Separation between different client sessions
5. **Audit Logging** - Comprehensive activity logging for security monitoring

## Handling Special Cases

### Privacy Mode
RCP supports a privacy mode that temporarily blocks screen transmission while maintaining the connection.

### File Transfer
Secure file transfer with integrity verification and permission controls.

### Multi-Display Support
Dynamic handling of multiple displays with efficient screen region updates.

## Performance Considerations

The RCP architecture is designed for high performance:

- Binary protocol with minimal overhead
- Efficient frame structure with selective updates
- Stream-based data transfer
- Optimized for low-latency operations
- Adaptive quality settings based on network conditions
- Connection quality monitoring

## Extension Points

RCP is designed to be extensible:

- Pluggable authentication providers
- Custom command handlers
- Extensible frame types
- Middleware support
- Platform-specific adapters
- API extensions for third-party integration
- Plugin architecture for the RCP Desk interface