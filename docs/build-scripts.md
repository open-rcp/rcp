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
scripts\windows\build.bat --release --rcpd --run

# On Linux/macOS:
./scripts/linux/build.sh --release --rcpc
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
- Sets up environment variables for OpenSSL
- Verifies dependencies with `cargo check`

#### Build Options

```
scripts\windows\build.bat [options]
```

Available options:
- `--release` - Build in release mode (optimized binaries)
- `--debug` - Build in debug mode (default)
- `--rcpp` - Build the RCP protocol library
- `--rcpc` - Build the RCP client library and CLI
- `--rcpd` - Build the RCP daemon with integrated server
- `--examples` - Build the example applications
- `--all` - Build all components (default)
- `--run-rcpd` - Run the RCP daemon after building
- `--run-rcpc` - Run the RCP client after building
- `--run-examples` - Run the example applications after building
- `--api` - Enable the API feature when building rcpd

### Linux

#### Setup Environment

Make the setup script executable and run with sudo:

```bash
chmod +x scripts/linux/setup.sh
sudo ./scripts/linux/setup.sh
```

This script:
- Installs essential build tools (build-essential, pkg-config)
- Installs SSL development libraries
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
PATH=[Updated to include OpenSSL]
```

## Examples

### Building the RCP Daemon in Release Mode and Running It

Windows:
```
scripts\windows\build.bat --release --rcpd --run-rcpd
```

Linux:
```bash
./scripts/linux/build.sh --release --rcpd --run-rcpd
```

macOS:
```bash
./scripts/macos/build.sh --release --rcpd --run-rcpd
```

### Building Only the RCP Client in Debug Mode

Windows:
```
scripts\windows\build.bat --debug --rcpc
```

Linux:
```bash
./scripts/linux/build.sh --debug --rcpc
```

macOS:
```bash
./scripts/macos/build.sh --debug --rcpc
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
3. Sets up environment variables for OpenSSL
4. Verifies project dependencies

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
3. Execute build commands with appropriate flags
4. Run components if requested

## Additional Resources

For more information:

- See [scripts/README.md](../scripts/README.md) for a detailed guide to all available scripts
- See [development-workflow.md](development-workflow.md) for a recommended development workflow