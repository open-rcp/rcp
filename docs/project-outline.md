# RCP Project Outline and Development Guidelines

This document outlines the Rust/Remote Control Protocol (RCP) project structure, components, and planned development roadmap.

## Project Architecture

RCP follows a modular architecture with three main components that work together in a cohesive system:

### Current Components

1. **rcpp (RCP Protocol)**: Core protocol library
   - Protocol definitions and frame handling
   - Authentication mechanisms
   - Common utilities and data structures
   - Binary format and command definitions

2. **rcpc (RCP Client)**: Client library and CLI
   - Connects to RCP servers using the protocol
   - Handles application control and user input
   - Provides connection string parsing
   - Service discovery and management

3. **rcpd (RCP Daemon)**: Runtime daemon with integrated components
   - Server functionality: Handling client connections
   - API functionality: Feature-gated REST API for remote management (optional)
   - Application lifecycle management
   - Configuration and service management
   - CLI functionality for server administration

### Planned Components

4. **RCP Desk**: End-user client application
   - Virtual application launcher
   - File transfer capabilities
   - User settings and preferences

5. **RCP Admin**: Administration interface
   - Web interface (SvelteKit-based)
   - Desktop application (Tauri)
   - Server configuration and monitoring

## Implementation Progress

### Completed Components
1. ✅ Core Protocol Library (rcpp)
2. ✅ Client Library (rcpc)
3. ✅ Server/Daemon with integrated components (rcpd)
3. ✅ Service Architecture
4. ✅ Authentication System
5. ✅ WebSocket Bridge
6. ✅ RCPD with Integrated Components
   - Integrated server functionality
   - Feature-gated API functionality
   - Runtime management of applications
   - Configuration persistence
   - System integration

### In-Progress Components
7. 🔄 RCP CLI
   - Service management commands
   - User administration
   - Configuration utility
   - Diagnostics tools

8. 🔄 RCP Admin
   - SvelteKit-based web interface
   - Tauri integration for desktop app
   - Real-time monitoring
   - Service configuration

9. 🔄 RCP Desk
   - End-user client application
   - Connection management
   - File transfer capabilities

## Codebase Structure

```
rcp/
├── rcpp/           # Core protocol definitions
│   └── src/
│       ├── auth.rs     # Authentication modules
│       ├── command.rs  # Protocol commands
│       ├── error.rs    # Error types
│       ├── frame.rs    # Frame parsing/serialization
│       ├── header.rs   # Protocol headers
│       ├── lib.rs      # Main library entry
│       ├── protocol.rs # Protocol handling
│       └── utils.rs    # Utilities
├── rcpc/         # Client library
│   └── src/            # Client implementation
├── rcpd/               # Runtime daemon with integrated server and API
│   └── src/
│       ├── config.rs   # Daemon configuration
│       ├── error.rs    # Error types
│       ├── main.rs     # Entry point
│       ├── manager.rs  # Daemon manager implementation
│       ├── server/     # Integrated server functionality
│       │   ├── config.rs  # Server configuration
│       │   ├── server.rs  # Server implementation
│       │   └── session.rs # Client session handling
│       ├── api/        # Integrated API functionality (feature-gated)
│       │   ├── config.rs  # API configuration
│       │   ├── server.rs  # API server implementation
│       │   └── handlers.rs # API request handlers
│       └── service.rs  # Service interface
├── rcp-cli/            # Command line interface
│   └── src/            # CLI implementation
├── rcp-desk/           # End-user client application
│   ├── src/            # Shared components
│   └── src-tauri/      # Desktop app implementation (Tauri)
├── rcp-ws-bridge/      # WebSocket bridge
│   └── src/            # Bridge implementation
├── examples/           # Example code
│   └── src/            # Example implementations
└── docs/               # Documentation
```

## Technical Implementation Status

### Current Focus Areas
- Service implementations (display, input, clipboard)
- Application launch and control
- Client library optimization
- Admin interface development
- CLI tool enhancements

### Upcoming Work
- Advanced authentication (public key)
- File transfer service optimization
- Audio streaming service
- Performance tuning
- Multi-platform packaging
- Client examples and documentation

## Development Guidelines

### Code Style

- Follow Rust standard style guidelines
- Use `rustfmt` for code formatting
- Document all public APIs with rustdoc comments
- Use appropriate error handling with `thiserror`
- Use `async/await` for asynchronous code

### Testing

- Write unit tests for all core functionality
- Include integration tests for protocol components
- Test both success and error paths

### Protocol Versioning

RCP follows semantic versioning with the following strategy:

- Protocol version is a single byte (0x01 for RCP/1.0)
- Breaking changes require a protocol version increment
- Non-breaking additions use command flags for extension

### Dealing with Concurrent Tasks

When working on the RCP project, it's common to have to switch between different components. Here are guidelines to manage concurrent tasks effectively:

1. **Component separation**: Keep changes contained within their respective modules
2. **Documentation updates**: After implementing a new feature, update relevant docs
3. **Cross-component changes**: When making changes that affect multiple components:
   - Start with the core library
   - Update dependent components in order (core → server → client → bridge)
4. **Testing continuity**: Ensure tests pass after each component update

## Extension Guidelines

### Adding New Commands

To add a new command to the protocol:

1. Add a new variant to the `CommandId` enum in `rcpp/src/command.rs`
2. Define the command payload structure(s) in the same file
3. Update the command handler in the session implementation

### Adding New Services

To add a new service:

1. Define a new service struct that implements the `Service` trait
2. Add the service to the `ServiceFactory`
3. Update permissions and service subscription handling

## Challenges and Considerations

- **Performance**: Keep overhead minimal, especially for real-time services
- **Security**: Ensure proper authentication and encrypted communications
- **Cross-platform**: Handle platform-specific aspects through abstract interfaces
- **Memory safety**: Avoid unsafe code unless absolutely necessary
- **Resource management**: Properly handle cleanup for all resources

## Development Phases

The project is currently in Phase 2 with some elements of Phase 3 in progress.

### Phase 1: Core Protocol (Completed)
- Core protocol implementation
- Basic server and client libraries
- Authentication system

### Phase 2: Feature Expansion (Current)
- Service implementations (display, input, clipboard, file transfer)
- Application launch and control
- WebSocket bridge for browser clients
- Runtime service architecture
- CLI management tools

### Phase 3: Management Layer (Starting)
- RESTful management API (integrated)
- Admin interface (in progress)
- End-user client application
- Authentication enhancements
- Advanced configuration management
- Logging and monitoring improvements

### Phase 4: Advanced Features (Planned)
- Audio/video streaming optimization
- Performance tuning and compression
- Multiple session management
- Load balancing capabilities
- Security hardening

### Phase 5: Integration & Deployment (Planned)
- Platform-specific packaging
- Enterprise deployment models
- Cloud-native deployment options
- Advanced monitoring and analytics