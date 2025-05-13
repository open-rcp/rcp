# RCP Daemon (RCPD)

This document outlines the RCP Daemon (RCPD), which provides runtime management for RCP applications with integrated server and API functionality.

## Overview

RCPD is a long-running daemon/system service that manages the lifecycle of applications and provides integrated server and API capabilities. It acts as a unified component that handles configuration persistence, monitoring, control, and communication within the RCP ecosystem.

Key responsibilities of RCPD include:

- Integrated server functionality for handling client connections
- Optional API endpoints for remote management (feature-gated)
- Application lifecycle management
- Runtime configuration
- System integration (startup, permissions, etc.)
- Monitoring and metrics collection
- Communication with RCP CLI and RCP Admin

## Architecture

RCPD follows a modular design with integrated server and API components:

```
┌────────────────────────────────────────────────────────┐
│                        RCPD                            │
│                                                        │
│  ┌─────────────┐  ┌────────────┐  ┌────────────────┐   │
│  │ Config      │  │ Server     │  │ App Lifecycle  │   │
│  │ Management  │  │ Component  │  │ Management     │   │
│  └─────────────┘  └────────────┘  └────────────────┘   │
│                                                        │
│  ┌─────────────┐  ┌────────────┐  ┌────────────────┐   │
│  │ User        │  │ Session    │  │ API Component  │   │
│  │ Management  │  │ Management │  │ (feature-gated)│   │
│  └─────────────┘  └────────────┘  └────────────────┘   │
│                                                        │
│  ┌─────────────────────────┐  ┌────────────────────┐   │
│  │ Connection & Protocol   │  │ Logging &          │   │
│  │ Handling               │  │ Monitoring         │   │
│  └─────────────────────────┘  └────────────────────┘   │
└────────────────────────────────────────────────────────┘
```

### Component Integration

RCPD integrates the following functionalities:

1. **Server Component**: Handles client connections, sessions, authentication, and protocol processing
2. **API Component**: Optional REST API endpoints for remote administration (enabled via feature flag)
3. **Service Core**: Handles lifecycle management, configuration, user management, etc.

## Functionality

### Server Management

The server component within RCPD is responsible for:

- Accepting and authenticating client connections
- Managing client sessions and subscriptions
- Dispatching client requests to appropriate services
- Managing application instances
- Handling disconnections and reconnections

### Application Management

RCPD provides comprehensive application lifecycle management:

- Application launching and termination
- Runtime environment configuration
- Process monitoring
- Resource allocation and control
- Application whitelist management
- Launch parameter configuration
- Resource monitoring and limits
- Process supervision
- Exit status handling

### Configuration Management

Centralized configuration handling:

- Storage of configuration in persistent storage
- Configuration validation
- Change tracking and versioning
- Environment-specific configurations
- Default values and templates

### User Management

User and permission handling:

- User database management
- Authentication provider integration
- Permission assignment
- Session tracking
- Activity logging

### Monitoring and Metrics

Comprehensive system monitoring:

- Resource usage tracking
- Performance metrics collection
- Threshold-based alerts
- Historical data retention
- Health check mechanisms

## Installation

### Windows

1. Build the daemon:
   ```bash
   cargo build --release -p rcpd
   ```

2. Install as a Windows service:
   ```bash
   rcpd install-service
   ```

3. Start the service:
   ```bash
   sc start rcpd
   ```

### Linux

1. Build the daemon:
   ```bash
   cargo build --release -p rcpd
   ```

2. Install the systemd service:
   ```bash
   sudo cp target/release/rcpd /usr/local/bin/
   sudo cp deployment/rcpd.service /etc/systemd/system/
   sudo systemctl daemon-reload
   ```

3. Start the daemon:
   ```bash
   sudo systemctl start rcpd
   ```

### macOS

1. Build the daemon:
   ```bash
   cargo build --release -p rcpd
   ```

2. Install the launchd service:
   ```bash
   cp target/release/rcpd /usr/local/bin/
   cp deployment/com.devstroop.rcpd.plist ~/Library/LaunchAgents/
   ```

3. Start the daemon:
   ```bash
   sudo launchctl start com.devstroop.rcpd
   ```

## Configuration

RCPD uses a TOML configuration file, typically located at:

- Windows: `C:\Program Files\RCP\config.toml`
- Linux: `/etc/rcpd/config.toml`
- macOS: `/usr/local/etc/rcpd/config.toml`

Example configuration:

```toml
# RCPD Configuration

# Basic configuration
address = "0.0.0.0"
port = 8716
work_dir = "/var/lib/rcpd"
log_level = "info"

# Server-specific configuration
[server]
address = "0.0.0.0"
port = 8717
max_connections = 100
connection_timeout = 30
idle_timeout = 300

# API configuration (only used when built with the 'api' feature)
[api]
enabled = true
address = "127.0.0.1"
port = 8080
auth_token_expiry = 3600

# TLS configuration
[tls]
enabled = false
cert_path = "/etc/rcpd/tls/cert.pem"
key_path = "/etc/rcpd/tls/key.pem"
```

## IPC Interface

RCPD provides an IPC interface for local control via a Unix socket or Windows named pipe:

- Windows: `\\.\pipe\rcpd`
- Unix: `/var/run/rcpd.sock`

This interface is used by the RCP CLI to communicate with the daemon for management operations.

## Command Line Usage

RCPD supports several command-line options:

```
USAGE:
    rcpd [OPTIONS]

OPTIONS:
    -c, --config <FILE>     Path to config file [default: config.toml]
    -d, --daemon            Run as a background daemon
    -f, --foreground        Run in the foreground
    -h, --help              Print help information
    -v, --verbose           Verbose output
    --version               Print version information
```

## Development Usage

For development purposes, you can run RCPD directly from Cargo:

```bash
cargo run -p rcpd -- --foreground
```

## API Reference

When built with the `api` feature, RCPD provides a RESTful API. See the [RCP API](rcp-api.md) documentation for details.

## See Also

- [RCP CLI](rcp-cli.md) - Command-line interface for administration
- [RCP API](rcp-api.md) - API reference for the RCPD API component
- [Server-Service Integration](server-service-integration.md) - Details on the integration of server and service components
