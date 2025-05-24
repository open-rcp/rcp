# RCP Development Workflow

This document outlines the recommended development workflow for contributing to the RCP project.

## Project Structure

The RCP project is organized into three main components:

1. **rcpcore**: Protocol library with core definitions
2. **rcpcli**: Client library and CLI interface
3. **rcpdaemon**: Daemon with integrated server and API

## Development Environment Setup

### Prerequisites

- Rust toolchain (via rustup)
- Platform-specific dependencies:
  - **Linux**: build-essential, pkg-config
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Initial Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/open-rcp/rcp.git
   cd rcp
   ```

2. Run the setup script for your platform:
   ```bash
   # macOS
   chmod +x scripts/macos/setup.sh
   ./scripts/macos/setup.sh
   
   # Linux
   chmod +x scripts/linux/setup.sh
   sudo ./scripts/linux/setup.sh
   
   # Windows
   scripts\windows\setup.bat
   ```

3. Build all components to verify setup:
   ```bash
   # macOS/Linux
   ./scripts/macos/build.sh --all
   
   # Windows
   scripts\windows\build.bat all
   ```

## Development Workflow

### Making Changes to Individual Components

#### 1. Protocol Library (rcpcore)

The protocol library contains core definitions that are used by both the client and daemon:

```bash
# Build only the protocol library
./scripts/macos/build.sh --rcpcore

# Run tests for the protocol library
cargo test -p rcpcore
```

After making changes to the protocol library, you need to rebuild dependent components:

```bash
# Rebuild the client and daemon
./scripts/macos/build.sh --rcpcli --rcpdaemon
```

#### 2. Client Library (rcpcli)

The client library provides a consistent interface for RCP clients:

```bash
# Build and run the client
./scripts/macos/build.sh --rcpcli --run-rcpcli
```

#### 3. Daemon (rcpdaemon)

The daemon integrates the server and API components:

```bash
# Build daemon with API feature enabled
./scripts/macos/build.sh --rcpdaemon --api

# Run the daemon
./scripts/macos/build.sh --rcpdaemon --api --run-rcpdaemon
```

### Testing Changes

Each component has its own test suite:

```bash
# Run all tests
cargo test

# Run specific component tests
cargo test -p rcpcore
cargo test -p rcpcli
cargo test -p rcpdaemon
```

### Building for Release

When preparing for release, build all components in release mode:

```bash
# macOS/Linux
./scripts/macos/build.sh --release --all

# Windows
scripts\windows\build.bat --release all
```

## Common Development Tasks

### 1. Adding a New Command to the Protocol

1. Define the command in `rcpcore/src/command.rs`
2. Implement serialization/deserialization in the same file
3. Add the command ID to the protocol specification
4. Add handler in the daemon and client as needed

### 2. Extending the Client

1. Modify `rcpcli/src/client.rs` to add new functionality
2. Update any CLI interfaces in `rcpcli/src/main.rs`
3. Add tests for the new functionality

### 3. Adding API Endpoints to the Daemon

1. Build the daemon with the API feature:
   ```bash
   ./scripts/macos/build.sh --rcpdaemon --api
   ```
2. Add new endpoints in `rcpdaemon/src/api/handlers.rs`
3. Register routes in `rcpdaemon/src/api/mod.rs`
4. Add tests for the new endpoints

## CI/CD Integration

The build scripts are designed to work in CI/CD environments:

```bash
# Example CI command for Linux
chmod +x scripts/linux/setup.sh scripts/linux/build.sh
sudo ./scripts/linux/setup.sh
./scripts/linux/build.sh --release --all
```

## Troubleshooting

If you encounter build issues:

1. Make sure Rust is up to date:
   ```bash
   rustup update
   ```

2. Clean the build directory:
   ```bash
   cargo clean
   ```

3. Re-run the setup script for your platform

4. Check for specific platform issues in the documentation
