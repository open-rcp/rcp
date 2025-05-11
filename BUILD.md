# RCP Build & Development Guide

This document provides comprehensive instructions for building and developing the RCP (Rust/Remote Control Protocol) project across different operating systems. The guide includes setting up development environments, building various components, and running the project for testing.

## Prerequisites

Before you can build the RCP project, ensure you have the following dependencies installed on your system:

### Common Dependencies
- Rust toolchain (install via [rustup](https://rustup.rs/))
- Git

### Windows-Specific Dependencies
- Visual Studio Build Tools with C++ toolchain
- [pkg-config](https://chocolatey.org/packages/pkgconfiglite) (can be installed via Chocolatey: `choco install pkgconfiglite`)
- [CMake](https://chocolatey.org/packages/cmake) (can be installed via Chocolatey: `choco install cmake`)

### Linux-Specific Dependencies
- build-essential package
- pkg-config
- libssl-dev
- cmake

**For Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev cmake
```

**For Fedora/RHEL/CentOS:**
```bash
sudo dnf install gcc gcc-c++ make pkg-config openssl-devel cmake
```

### macOS-Specific Dependencies
- Xcode Command Line Tools
- [Homebrew](https://brew.sh/) (recommended)
- pkg-config and cmake via Homebrew:
```bash
brew install pkg-config cmake
```

## Build Scripts Location

All build scripts are located in the `scripts/` directory with platform-specific subdirectories:

- `scripts/windows/` - Scripts for Windows
- `scripts/linux/` - Scripts for Linux
- `scripts/macos/` - Scripts for macOS

## Platform-Specific Instructions

### Windows

#### Setup Development Environment

1. Open Command Prompt or PowerShell as administrator
2. Run the setup script:
   ```
   scripts\windows\setup.bat
   ```

#### Building Components

```
scripts\windows\build.bat [options]
```

Options:
- `--release` - Build in release mode (default: debug)
- `--debug` - Build in debug mode
- `--server` - Build only the server component
- `--client` - Build only the client component
- `--service` - Build only the service component
- `--cli` - Build only the CLI component
- `--api` - Build only the API component
- `--desk` - Build only the Desk interface component
- `--ws-bridge` - Build only the WebSocket bridge component
- `--all` - Build all components (default)
- `--run` - Run the server after building
- `--run-server` - Run the server after building
- `--run-client` - Run the client after building
- `--run-service` - Run the service after building
- `--run-cli` - Run the CLI after building
- `--run-api` - Run the API after building
- `--run-desk` - Run the Desk interface after building
- `--run-ws-bridge` - Run the WebSocket bridge after building

Example:
```
scripts\windows\build.bat --release --server --run
```

### Linux

#### Setup Development Environment

1. Make the script executable (if not already):
   ```bash
   chmod +x scripts/linux/setup.sh
   ```

2. Run the setup script as root:
   ```bash
   sudo ./scripts/linux/setup.sh
   ```

#### Building Components

First, make the script executable if not already:
```bash
chmod +x scripts/linux/build.sh
```

Then build:
```bash
./scripts/linux/build.sh [options]
```

Options:
- `--release` - Build in release mode (default: debug)
- `--debug` - Build in debug mode
- `--server` - Build only the server component
- `--client` - Build only the client component
- `--service` - Build only the service component
- `--cli` - Build only the CLI component
- `--api` - Build only the API component
- `--desk` - Build only the Desk interface component
- `--ws-bridge` - Build only the WebSocket bridge component
- `--all` - Build all components (default)
- `--run` - Run the server after building
- `--run-server` - Run the server after building
- `--run-client` - Run the client after building
- `--run-service` - Run the service after building
- `--run-cli` - Run the CLI after building
- `--run-api` - Run the API after building
- `--run-desk` - Run the Desk interface after building
- `--run-ws-bridge` - Run the WebSocket bridge after building

Example:
```bash
./scripts/linux/build.sh --release --client
```

### macOS

#### Setup Development Environment

1. Make the script executable (if not already):
   ```bash
   chmod +x scripts/macos/setup.sh
   ```

2. Run the setup script:
   ```bash
   ./scripts/macos/setup.sh
   ```

3. After setup completes, you may need to restart your terminal or run:
   ```bash
   source ~/.zshrc
   ```

#### Building Components

First, make the script executable if not already:
```bash
chmod +x scripts/macos/build.sh
```

Then build:
```bash
./scripts/macos/build.sh [options]
```

Options:
- `--release` - Build in release mode (default: debug)
- `--debug` - Build in debug mode
- `--server` - Build only the server component
- `--client` - Build only the client component
- `--service` - Build only the service component
- `--cli` - Build only the CLI component
- `--api` - Build only the API component
- `--desk` - Build only the Desk interface component
- `--ws-bridge` - Build only the WebSocket bridge component
- `--all` - Build all components (default)
- `--run` - Run the server after building
- `--run-server` - Run the server after building
- `--run-client` - Run the client after building
- `--run-service` - Run the service after building
- `--run-cli` - Run the CLI after building
- `--run-api` - Run the API after building
- `--run-desk` - Run the Desk interface after building
- `--run-ws-bridge` - Run the WebSocket bridge after building

Example:
```bash
./scripts/macos/build.sh --release --ws-bridge --run-ws-bridge
```

## Quick Reference - Common Tasks

### Build All Components (Debug)
```bash
# Windows
scripts\windows\build.bat --all

# Linux/macOS
./scripts/linux/build.sh --all
./scripts/macos/build.sh --all
```

### Build and Run Service
```bash
# Windows
scripts\windows\build.bat --service --run

# Linux/macOS
./scripts/linux/build.sh --service --run
./scripts/macos/build.sh --service --run
```

### Build Release Version of Desk UI
```bash
# Windows
scripts\windows\build.bat --release --desk

# Linux/macOS
./scripts/linux/build.sh --release --desk
./scripts/macos/build.sh --release --desk
```

## Note for Repository Maintainers

When checking out this repository on Linux or macOS, you'll need to make sure the shell scripts are executable:

```bash
chmod +x scripts/linux/*.sh scripts/macos/*.sh
```

This should be done before running any scripts.

## Troubleshooting

If you encounter build issues:

1. Ensure all dependencies are installed (run the setup script)
2. Check that your Rust toolchain is up-to-date (`rustup update`)
3. Try cleaning the build with `cargo clean` before rebuilding
4. For platform-specific issues, see the README files in each platform directory