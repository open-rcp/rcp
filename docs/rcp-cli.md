# RCP CLI

This document outlines the RCP CLI (Command Line Interface), which provides administrative tools for managing and controlling the RCP system.

## Overview

RCP CLI is a command-line utility designed for administrators to interact with RCP Service and RCP Servers. It provides functionality for installation, configuration, monitoring, and management operations.

Key features of the RCP CLI include:

- Service management (install, start, stop, status)
- Server management and monitoring
- Session control
- User administration
- Configuration management
- Diagnostic tools

## Architecture

The RCP CLI interfaces with the RCP Service via either a local socket connection or the RCP API, depending on the command and availability:

```
┌─────────────┐     ┌─────────────────┐     ┌─────────────┐
│  RCP CLI    │────►│  Local Socket   │────►│ RCP Service │
└─────────────┘     └─────────────────┘     └─────────────┘
       │                                          │
       │            ┌─────────────────┐           │
       └───────────►│     RCP API     │◄──────────┘
                    └─────────────────┘
```

## Command Structure

RCP CLI follows a git-like command structure:

```
rcp-cli [global options] command [command options] [arguments...]
```

### Global Options

| Option | Description |
|--------|-------------|
| `--config FILE` | Specify alternative config file |
| `--log-level LEVEL` | Set log level (debug, info, warn, error) |
| `--json` | Output in JSON format |
| `--quiet` | Suppress non-error output |
| `--help, -h` | Show help |
| `--version, -v` | Show version |

## Available Commands

### Service Management

```bash
# Install RCP service
rcp-cli service install [--auto-start] [--user USERNAME]

# Start the service
rcp-cli service start

# Stop the service
rcp-cli service stop

# Restart the service
rcp-cli service restart

# Get service status
rcp-cli service status

# Uninstall the service
rcp-cli service uninstall
```

### Server Management

```bash
# List configured servers
rcp-cli server list

# Start a server instance
rcp-cli server start [name]

# Stop a server instance
rcp-cli server stop [name]

# Restart a server instance
rcp-cli server restart [name]

# Get server status
rcp-cli server status [name]

# Create a new server configuration
rcp-cli server create [name] [--port PORT] [--max-conn MAX] [--tls]
```

### Session Management

```bash
# List active sessions
rcp-cli session list [--server SERVER]

# Get session details
rcp-cli session info SESSION_ID

# Terminate a session
rcp-cli session terminate SESSION_ID [--reason REASON]

# Send message to session
rcp-cli session message SESSION_ID "Your message"
```

### User Management

```bash
# List users
rcp-cli user list

# Add a new user
rcp-cli user add USERNAME [--admin] [--password PASSWORD]

# Set user password
rcp-cli user passwd USERNAME

# Delete user
rcp-cli user remove USERNAME

# Modify user roles
rcp-cli user roles USERNAME [--add ROLE] [--remove ROLE]
```

### Configuration

```bash
# View current configuration
rcp-cli config show [--section SECTION]

# Set configuration value
rcp-cli config set KEY VALUE

# Reset configuration to defaults
rcp-cli config reset [--section SECTION]

# Validate configuration
rcp-cli config validate
```

### Diagnostic Tools

```bash
# Run diagnostics
rcp-cli diag run [--tests TEST1,TEST2]

# Check connectivity
rcp-cli diag connectivity [--server SERVER]

# View system logs
rcp-cli logs [--level LEVEL] [--limit NUM] [--since TIME]
```

## Installation

RCP CLI is installed as part of the standard RCP package:

```bash
# Using cargo
cargo install rcp-cli

# From release package
tar -xzf rcp-cli-1.0.0.tar.gz
cd rcp-cli-1.0.0
make install
```

## Configuration

The CLI looks for configuration in these locations (in order):

1. Path specified by `--config` option
2. `./rcp-cli.toml` in the current directory
3. `~/.config/rcp/cli.toml` (Unix) or `%APPDATA%\RCP\cli.toml` (Windows)
4. System-wide configuration

Example configuration file:

```toml
[cli]
# CLI settings
log_level = "info"
default_format = "table"
color = true

[connection]
# Connection settings
socket = "/var/run/rcp-service.sock"  # Unix
# socket = "\\\\.\\pipe\\rcp-service"  # Windows
api_url = "http://localhost:8080/api/v1"
timeout = 5  # seconds

[auth]
# Authentication settings (for API mode)
save_token = true
token_path = "~/.config/rcp/token"
```

## Authentication

When interacting with the RCP API, the CLI requires authentication:

```bash
# Login to RCP API
rcp-cli login [--username USERNAME]

# Logout (remove stored credentials)
rcp-cli logout
```

## Interactive Mode

RCP CLI provides an interactive shell mode:

```bash
# Start interactive mode
rcp-cli shell

# Inside the shell
rcp> server list
rcp> session list
rcp> exit
```

## Scripting Support

For scripting and automation, the CLI provides machine-readable output:

```bash
# JSON output
rcp-cli --json server list

# Exit status code
echo $?  # Non-zero indicates error

# Batch commands
rcp-cli batch commands.txt
```

## Completions

The CLI supports shell completions:

```bash
# Generate shell completions
rcp-cli completions bash > /etc/bash_completion.d/rcp
rcp-cli completions zsh > ~/.zshrc.d/_rcp
rcp-cli completions fish > ~/.config/fish/completions/rcp.fish
rcp-cli completions powershell > rcp.ps1
```

## Security Considerations

- Commands requiring elevated privileges will prompt for confirmation
- Authentication tokens are stored securely with appropriate permissions
- Sensitive output is masked by default (use `--show-sensitive` to reveal)
- Logs exclude sensitive information

## Development

For development:

```bash
# Build and run CLI directly
cargo run -p rcp-cli -- [arguments]

# Run tests
cargo test -p rcp-cli
```

## Implementation Plan

1. **Phase 1: Core Commands**
   - Basic service management
   - Status reporting
   - Configuration viewing

2. **Phase 2: Management Commands**
   - Server management
   - Session management
   - User management

3. **Phase 3: Advanced Features**
   - Interactive mode
   - Shell completions
   - Full diagnostics
   - Scripting support

## Windows-Specific Features

Windows environments have specific commands:

```powershell
# Register as Windows Service
rcp-cli service install --windows-service

# Configure firewall
rcp-cli windows firewall --allow
```

## Unix-Specific Features

Unix environments have specific commands:

```bash
# Generate systemd unit file
rcp-cli service systemd-unit > /etc/systemd/system/rcp-service.service

# Generate completion for current shell
rcp-cli completions auto > ~/.rcp-completion
```