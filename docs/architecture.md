# RCP Architecture Overview

## System Architecture

The Rust/Remote Control Protocol (RCP) is a modular service-oriented system with several components that work together. The architecture has been optimized by integrating the server and API capabilities into the service component, while maintaining the CLI as a separate component for flexibility and separation of concerns.

```
┌─────────────────┐      ┌────────────────────────────────────┐
│ RCP Client Lib  │◄─────┤            RCP Service             │
└────────┬────────┘      │   ┌────────────┐  ┌────────────┐   │
         │               │   │   Server   │  │    API     │   │
    ┌────▼────┐          │   │ Component  │  │ Component  │   │
    │ RCP Desk│          │   └────────────┘  └────────────┘   │
    │(End-user│          └───────────────────┬────────────────┘
    │  App)   │                              │
    └─────────┘                   ┌──────────▼─────────┐
                                  │     Session Mgr    │◄──────┐
                                  └──────────┬─────────┘       │
                                             │                 │
                                  ┌──────────▼─────────┐  ┌────▼────┐
                                  │     Application    │  │ RCP CLI │
                                  │      Processes     │  └────┬────┘
                                  └────────────────────┘       │
                                                          ┌────▼────┐
                                                          │RCP Admin│
                                                          └─────────┘
```

## Core Components

### 1. RCP Core Library (`rcp-core`)

The foundation of the RCP system providing:

- Protocol definitions
- Frame parsing and serialization
- Authentication mechanisms
- Common utilities

### 2. RCP Client (`rcp-client`)

- Connects to RCP servers within the integrated service
- Subscribes to required services
- Sends application control commands
- Handles user input
- Processes and displays streamed frames

### 3. RCP Service (`rcp-service`)

- Core component with integrated Server and API functionality
- Long-running daemon/system service architecture
- Component-based internal design with clear separation of concerns:
  - **Server Component**: Handles connections, sessions, and protocol handling (fully integrated)
  - **API Component**: Provides RESTful endpoints for management (optional via "api" feature flag)
  - **Service Core**: Handles lifecycle management, configuration, and orchestration
- Listens for incoming TCP connections (server capability)
- Manages authentication and sessions
- Spawns and controls applications
- Provides specialized services:
  - **Connection Service**: Handles core connection lifecycle
  - **Display Service**: Manages screen capture and display information 
  - **Input Service**: Processes remote keyboard and mouse events
  - **Audio Service**: Handles audio streaming
  - **Clipboard Service**: Manages clipboard synchronization
  - **File Transfer Service**: Handles file operations between peers
- Application lifecycle management
- Configuration handling and persistence
- System integration (startup service, user permissions)
- Logs and monitoring
- Communication channel with CLI and Admin interface

### 4. RCP CLI (`rcp-cli`)

- Command-line interface for server administration only
- Administrative tasks (user management, configuration, server control)
- Server and service management
- Session monitoring and management
- Status reporting and diagnostics
- Service installation/uninstallation
- Deliberately maintained as a separate component from the service for:
  - Separation of concerns (UI vs. backend)
  - Deployment flexibility (can be installed separately)
  - Reduced binary size and dependencies
  - Independent development lifecycle

### 5. RCP Admin (`rcp-admin`)

- Administrative interface for server management
- Built with SvelteKit for the web component
- Desktop application built with Tauri
- Server configuration and monitoring
- User and access management
- Application publishing and configuration
- Communicates with the service via its integrated API

### 6. RCP Desk (`rcp-desk`)

- End-user client application for accessing virtual applications
- Built with SvelteKit and Tauri
- Connection management to RCP servers
- Virtual application launcher
- File transfer capabilities
- Settings and profile management
- User management
- Session monitoring and control
- Configuration interface
- Analytics and reporting

### 9. WebSocket Bridge (`rcp-ws-bridge`)

An optional component that bridges RCP protocol to WebSockets for browser clients:

- Protocol translation
- Frame transcoding
- Web client interface

## Service Architecture

RCP uses a subscription-based service model:

```
┌─────────────┐    ┌─────────────┐     ┌─────────────┐
│ Display     │    │ Input       │     │ Clipboard   │
│ Service     │    │ Service     │     │ Service     │
└──────┬──────┘    └──────┬──────┘     └──────┬──────┘
       │                  │                   │
       └──────────┬───────┴─────────┬─────────┘
                  │                 │
        ┌─────────▼─────────┐      ┌▼─────────────────┐
        │  Connection #1    │      │  Connection #2   │
        │  (Subscribed to   │      │  (Subscribed to  │
        │   all services)   │      │   input only)    │
        └───────────────────┘      └──────────────────┘
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
┌───────────┐   ┌──────┐    ┌─────────────┐    ┌───────────┐
│ RCP Admin │ → │ API  │ →  │ RCP Service │ →  │ RCP Server│
└───────────┘   └──────┘    └─────────────┘    └───────────┘
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
    │  RCP CLI     │ │ RCP API  │    │ RCP Admin  │
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

## Communication Flow

1. **Administration Flow**:
   - rcp-admin -> rcp-service/api-component (Management interface)
   - rcp-cli -> rcp-service/server-component (Command-line administration)

2. **End-User Flow**:
   - rcp-desk -> rcp-client -> rcp-service/server-component (Native desktop client)
   - Web Client -> rcp-ws-bridge -> rcp-service/server-component (Browser client)

## Configuration Management

- Server configurations stored in TOML files
- Dynamic configuration updates without restart
- User permissions and application settings in structured storage
- Audit logging for security events

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