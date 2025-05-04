# RCP Build Scripts Documentation

This document provides information about the build and configuration scripts available for the RCP project across different operating systems.

## Overview

The RCP project includes a set of scripts designed to simplify the development workflow across Windows, Linux, and macOS environments. These scripts handle:

- Environment setup and dependency installation
- Building specific components or the entire project
- Running components after building
- Supporting both debug and release builds

## Script Locations

Scripts are organized in the `scripts` directory with platform-specific subdirectories:

```
scripts/
├── README.md
├── windows/
│   ├── setup.bat
│   └── build.bat
├── linux/
│   ├── setup.sh
│   └── build.sh
└── macos/
    ├── setup.sh
    └── build.sh
```

## Common Usage Patterns

### Building Components

All build scripts support similar arguments across platforms:

```bash
# On Windows:
scripts\windows\build.bat --release --server --run

# On Linux/macOS:
./scripts/linux/build.sh --release --client
```

### Making Scripts Executable (Linux/macOS)

When checking out the repository on Linux or macOS systems, you need to make the shell scripts executable:

```bash
chmod +x scripts/linux/*.sh scripts/macos/*.sh
```

## Platform-Specific Instructions

### Windows

#### Setup Environment

Open Command Prompt or PowerShell as administrator and run:

```
scripts\windows\setup.bat
```

This script:
- Checks for and installs Rust if needed
- Configures the Rust toolchain
- Installs OpenSSL and CMake via Chocolatey (if available)
- Sets up environment variables
- Verifies dependencies with `cargo check`

#### Build Options

```
scripts\windows\build.bat [options]
```

Available options:
- `--release` - Build in release mode (optimized binaries)
- `--debug` - Build in debug mode (default)
- `--server` - Build only the server component
- `--client` - Build only the client component
- `--ws-bridge` - Build only the WebSocket bridge component
- `--all` - Build all components (default)
- `--run` - Run the server component after building
- `--run-server` - Run the server component after building
- `--run-client` - Run the client component after building
- `--run-ws-bridge` - Run the WebSocket bridge component after building

### Linux

#### Setup Environment

Make the setup script executable and run with sudo:

```bash
chmod +x scripts/linux/setup.sh
sudo ./scripts/linux/setup.sh
```

This script:
- Installs essential build tools (build-essential, cmake, pkg-config)
- Installs SSL development libraries
- Installs additional dependencies (libclang-dev, llvm-dev)
- Installs or updates Rust
- Configures the Rust toolchain
- Verifies dependencies with `cargo check`

#### Build Options

```bash
./scripts/linux/build.sh [options]
```

Available options are the same as those listed for Windows.

### macOS

#### Setup Environment

Make the setup script executable and run:

```bash
chmod +x scripts/macos/setup.sh
./scripts/macos/setup.sh
```

This script:
- Installs Homebrew if not already installed
- Installs build dependencies via Homebrew
- Installs and configures OpenSSL
- Sets up environment variables in ~/.zshrc
- Installs or updates Rust
- Configures the Rust toolchain
- Installs LLVM
- Verifies dependencies with `cargo check`

After setup completes, you may need to restart your terminal or run:
```bash
source ~/.zshrc
```

#### Build Options

```bash
./scripts/macos/build.sh [options]
```

Available options are the same as those listed for Windows.

## Environment Variables

The following environment variables are set by the setup scripts:

### Windows

```
OPENSSL_DIR=C:\Program Files\OpenSSL-Win64
OPENSSL_INCLUDE_DIR=C:\Program Files\OpenSSL-Win64\include
OPENSSL_LIB_DIR=C:\Program Files\OpenSSL-Win64\lib
```

### macOS

```
OPENSSL_DIR=[Homebrew OpenSSL path]
OPENSSL_INCLUDE_DIR=[Homebrew OpenSSL include path]
OPENSSL_LIB_DIR=[Homebrew OpenSSL lib path]
PATH=[Updated to include OpenSSL and LLVM]
```

## Examples

### Building the Server in Release Mode and Running It

Windows:
```
scripts\windows\build.bat --release --server --run
```

Linux:
```bash
./scripts/linux/build.sh --release --server --run
```

macOS:
```bash
./scripts/macos/build.sh --release --server --run
```

### Building Only the Client in Debug Mode

Windows:
```
scripts\windows\build.bat --debug --client
```

Linux:
```bash
./scripts/linux/build.sh --debug --client
```

macOS:
```bash
./scripts/macos/build.sh --debug --client
```

### Building All Components in Release Mode

Windows:
```
scripts\windows\build.bat --release --all
```

Linux:
```bash
./scripts/linux/build.sh --release --all
```

macOS:
```bash
./scripts/macos/build.sh --release --all
```

## Script Implementation Details

### Windows Setup Script

The Windows setup script performs the following operations:

1. Checks for Rust installation and installs/updates as needed
2. Configures Rust toolchain with rustfmt and clippy
3. Installs OpenSSL using Chocolatey if available
4. Installs CMake if needed
5. Sets up environment variables for OpenSSL
6. Verifies project dependencies

### Linux/macOS Setup Scripts

The Linux and macOS scripts follow a similar pattern:

1. Install system-specific package manager dependencies
2. Install/update Rust
3. Configure Rust toolchain
4. Set up environment variables
5. Verify project dependencies

### Build Scripts

All build scripts follow a similar pattern:

1. Parse command line arguments
2. Set build type and target components
3. Configure environment variables if needed
4. Create build directory if it doesn't exist
5. Run cargo build with appropriate options
6. Run the component if requested

## Troubleshooting

### OpenSSL Not Found

If you encounter OpenSSL-related errors during building:

- On Windows: Ensure OpenSSL is installed at `C:\Program Files\OpenSSL-Win64` or set OPENSSL_DIR manually
- On Linux: Run `sudo apt-get install libssl-dev` (or your distro's equivalent)
- On macOS: Run `brew install openssl@3` and ensure environment variables are set correctly

### Permission Denied on Linux/macOS

If you get "Permission denied" when running scripts:

```bash
chmod +x scripts/linux/*.sh scripts/macos/*.sh
```

### Build Errors

If you encounter build errors:

1. Ensure all dependencies are installed
2. Run `cargo check` to verify dependencies
3. Try building in debug mode first
4. Check compiler error messages for specific issues