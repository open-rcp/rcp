# RCP Project Directory Structure

This document provides an overview of the RCP project's directory structure and explains the purpose of each component.

## Root Directory

The root directory contains configuration files, documentation, and the main project components:

- `BUILD.md` - Build instructions for all platforms
- `CONTRIBUTING.md` - Guidelines for contributors
- `LICENSE` - Project license information
- `README.md` - Project overview and getting started guide
- `CHANGELOG.md` - Project history and version changes
- `RELEASE.md` - Release notes and versioning information
- `SECURITY.md` - Security policy information

## Core Components

### `/rcp-core`

The foundation library that defines the protocol:

- Protocol definitions and constants
- Frame parsing and serialization
- Authentication mechanisms
- Error handling
- Common utilities

### `/rcpd`

The main daemon with integrated server functionality:

- Server implementation for handling client connections
- Feature-gated API for remote management
- Session management
- Application lifecycle management
- Service implementations (display, input, audio, clipboard, etc.)

### `/rcp-client`

Client library for connecting to RCP servers:

- Connection handling and management
- Protocol implementation (client side)
- Service subscriptions
- Event handling

### `/rcp-cli`

Command-line interface for server administration:

- Server management commands (start, stop, status)
- User management
- Configuration tools
- Diagnostics and troubleshooting
- Resource monitoring

### `/rcp-admin`

Web and desktop interface for server administration:

- SvelteKit-based web interface
- Tauri integration for desktop app
- User management interface
- Server configuration
- Monitoring dashboard

### `/rcp-desk`

End-user client application:

- SvelteKit-based interface
- Tauri integration for desktop functionality
- Virtual application launcher
- Connection management
- File transfer interface
- User preferences

### `/rcp-ws-bridge`

WebSocket bridge for browser-based clients:

- Protocol translation between RCP and WebSockets
- Frame transcoding
- Web client support

## Supporting Directories

### `/docs`

Project documentation:

- Architecture overview
- Protocol specification
- Development guidelines
- Component documentation
- Integration details

### `/examples`

Example implementations and tutorials:

- Client connection examples
- Application launch examples
- Service usage examples

### `/scripts`

Platform-specific build and setup scripts:

- `/scripts/windows` - Windows-specific scripts
- `/scripts/linux` - Linux-specific scripts
- `/scripts/macos` - macOS-specific scripts

### `/build-utils`

Build utilities and helpers:

- Native dependency handling
- Build configuration processing

### `/deps`

External dependencies and submodules:

- Modified dependencies
- Vendored libraries

## Build Artifacts

### `/target`

Rust build output directory (created by Cargo):

- Debug and release builds
- Documentation output
- Test artifacts
