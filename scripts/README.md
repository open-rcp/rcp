# RCP Build Scripts

This directory contains build and configuration scripts for the RCP project. These scripts are designed to simplify setting up the development environment and building the project across different operating systems.

## Directory Structure

- `windows/` - Scripts for Windows
- `linux/` - Scripts for Linux
- `macos/` - Scripts for macOS

## Usage

### Windows

#### Setup

1. Open Command Prompt or PowerShell as administrator
2. Run the setup script:
   ```
   scripts\windows\setup.bat
   ```

#### Building

```
scripts\windows\build.bat [options]
```

Options:
- `--release` - Build in release mode (default: debug)
- `--debug` - Build in debug mode
- `--server` - Build only the server component
- `--client` - Build only the client component
- `--ws-bridge` - Build only the WebSocket bridge component
- `--all` - Build all components (default)
- `--run` - Run the server after building
- `--run-server` - Run the server after building
- `--run-client` - Run the client after building
- `--run-ws-bridge` - Run the WebSocket bridge after building

Example:
```
scripts\windows\build.bat --release --server --run
```

### Linux

#### Setup

1. Make the script executable (if not already):
   ```bash
   chmod +x scripts/linux/setup.sh
   ```

2. Run the setup script as root:
   ```bash
   sudo ./scripts/linux/setup.sh
   ```

#### Building

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
- `--ws-bridge` - Build only the WebSocket bridge component
- `--all` - Build all components (default)
- `--run` - Run the server after building
- `--run-server` - Run the server after building
- `--run-client` - Run the client after building
- `--run-ws-bridge` - Run the WebSocket bridge after building

Example:
```bash
./scripts/linux/build.sh --release --client
```

### macOS

#### Setup

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

#### Building

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
- `--ws-bridge` - Build only the WebSocket bridge component
- `--all` - Build all components (default)
- `--run` - Run the server after building
- `--run-server` - Run the server after building
- `--run-client` - Run the client after building
- `--run-ws-bridge` - Run the WebSocket bridge after building

Example:
```bash
./scripts/macos/build.sh --release --ws-bridge --run-ws-bridge
```

## Note for Repository Maintainers

When checking out this repository on Linux or macOS, you'll need to make sure the shell scripts are executable:

```bash
chmod +x scripts/linux/*.sh scripts/macos/*.sh
```

This should be done before running any scripts.