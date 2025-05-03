# RCP Project Outline and Development Guidelines

## Project Overview

The Rust Control Protocol (RCP) is a low-level, high-performance protocol designed for remote application control. It enables secure remote control of desktop applications over TCP/IP using the Rust programming language. This document outlines the project structure, development guidelines, and roadmap.

## Architecture

RCP follows a modular architecture with these core components:

1. **rcp-core**: Protocol definitions, frame handling, authentication, and common utilities
2. **rcp-server**: The server application that accepts connections and manages sessions
3. **rcp-client**: Client library for connecting to RCP servers and controlling applications
4. **rcp-ws-bridge**: Optional WebSocket bridge for browser clients

## Codebase Structure

```
rcp/
├── rcp-core/           # Core protocol definitions
│   ├── src/
│       ├── auth.rs     # Authentication modules
│       ├── command.rs  # Protocol commands
│       ├── error.rs    # Error types
│       ├── frame.rs    # Frame parsing/serialization
│       ├── header.rs   # Protocol headers
│       ├── lib.rs      # Main library entry
│       ├── protocol.rs # Protocol handling
│       └── utils.rs    # Utilities
├── rcp-server/         # Server implementation
│   ├── src/
│       ├── config.rs   # Server configuration
│       ├── error.rs    # Error types
│       ├── main.rs     # Entry point
│       ├── server.rs   # Server implementation
│       ├── service.rs  # Service interface
│       └── session.rs  # Client session handling
├── rcp-client/         # Client library
│   └── src/            # (TBD)
├── rcp-ws-bridge/      # WebSocket bridge
│   └── src/            # (TBD)
├── examples/           # Example code
│   └── src/            # (TBD)
└── docs/               # Documentation
```

## Implementation Status

### Completed
- Core protocol definition (frames, headers)
- Authentication mechanism 
- Server configuration
- Basic session management
- Service interface definition

### In Progress
- Service implementations (display, input, clipboard)
- Application launch and control
- Client library

### Planned
- Advanced authentication (public key)
- WebSocket bridge
- File transfer service
- Audio streaming service
- Client examples

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

1. Add a new variant to the `CommandId` enum in `rcp-core/src/command.rs`
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

## Project Roadmap

### Phase 1: Core Protocol (Current)
- Complete core protocol implementation
- Implement server with basic services
- Basic client library

### Phase 2: Feature Expansion
- Complete all core services (display, input, clipboard, file transfer)
- Application launching and control
- WebSocket bridge for browser clients

### Phase 3: Advanced Features
- Audio/video streaming optimization
- Compression and performance tuning
- Multiple session management
- Load balancing capabilities

### Phase 4: Extra Integrations
- Browser-based client
- Mobile client library
- Integration with popular remote access protocols