# RCP Service (DEPRECATED)

> **DEPRECATED**: This document is deprecated. Please refer to [rcpdaemon (RCP Daemon)](rcpdaemon.md) for current documentation.

This document previously outlined the RCP Service, which has been renamed to rcpdaemon (RCP Daemon).

## Overview

RCP Service is a long-running daemon/system service that manages the lifecycle of applications and provides integrated server and API capabilities. It acts as a unified component that handles configuration persistence, monitoring, control, and communication within the RCP ecosystem.

Key responsibilities of the RCP Service include:

- Integrated server functionality for handling client connections
- Optional API endpoints for remote management (feature-gated)
- Application lifecycle management
- Runtime configuration
- System integration (startup, permissions, etc.)
- Monitoring and metrics collection
- Communication with RCP CLI and RCP Admin

## Architecture

The RCP Service follows a modular design with integrated server and API components:

```
┌────────────────────────────────────────────────────────┐
│                     RCP Service                        │
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

## Core Features

### Integrated Server Functionality

The service includes integrated server functionality:

- Direct handling of RCP protocol connections
- Session management and authentication
- Service subscription handling
- Protocol versioning and compatibility
- Connection health monitoring
- Multiple simultaneous client support
- Automatic recovery from failures

### Application Lifecycle Management

Complete control over the applications accessible through RCP:

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

```powershell
# Install as a Windows service
rcp-cli service install

# Start the service
rcp-cli service start
```

### Linux

```bash
# Install as a systemd service
sudo rcp-cli service install

# Start the service
sudo systemctl start rcp-service
```

### macOS

```bash
# Install as a launchd service
sudo rcp-cli service install

# Start the service
sudo launchctl start com.devstroop.rcp-service
```

## Configuration

The service configuration is stored in a central location that varies by platform:

- Windows: `%ProgramData%\RCP\config.toml`
- Linux: `/etc/rcp/config.toml`
- macOS: `/Library/Application Support/RCP/config.toml`

Example configuration:

```toml
[service]
# Service settings
name = "RCP Service"
log_level = "info"
data_dir = "/var/lib/rcp"

[servers]
# Server instances configuration
default = { port = 8716, max_connections = 100, name = "Default Server" }
secure = { port = 8717, max_connections = 50, tls = true, name = "Secure Server" }

[security]
# Security settings
auth_required = true
allowed_auth_methods = ["psk", "public_key"]
session_timeout = 3600

[applications]
# Application settings
whitelist_enabled = true
allowed_apps = [
  { path = "/usr/bin/firefox", args = ["--private-window"], display_name = "Firefox" },
  { path = "C:\\Windows\\System32\\notepad.exe", display_name = "Notepad" }
]
```

## Communication Interfaces

The RCP Service exposes multiple interfaces for management:

### Local Socket

A Unix socket or named pipe for local communication with the CLI:

- Windows: `\\.\pipe\rcp-service`
- Unix: `/var/run/rcp-service.sock`

### RCP API Integration

The service provides the backend for the RCP API component:

- RESTful API for remote management
- Authentication and authorization
- Configuration endpoints
- Monitoring endpoints

### Event System

The service implements an event system for:

- Real-time notifications
- Status changes
- Error reporting
- Audit events

## Service Commands

The service responds to these commands:

| Command | Description |
|---------|-------------|
| status | Reports the current status of the service |
| start-server | Starts a configured server instance |
| stop-server | Stops a running server instance |
| restart-server | Restarts a server instance |
| reload-config | Reloads configuration without restart |
| shutdown | Gracefully shuts down the service |

## Logging

The service maintains detailed logs:

- Configurable log levels
- Log rotation
- Structured logging format
- Multiple outputs (file, syslog, etc.)

Default log locations:
- Windows: `%ProgramData%\RCP\logs\`
- Linux: `/var/log/rcp/`
- macOS: `/Library/Logs/RCP/`

## Development

For development purposes, the service can be run in the foreground:

```bash
cargo run -p rcp-service -- --foreground
```

## Security Considerations

The RCP Service is designed with security in mind:

- Runs with minimal required privileges
- Secures communication channels
- Validates all configuration changes
- Implements defense-in-depth principles
- Provides detailed audit logging

## Implementation Plan

1. **Phase 1: Core Service**
   - Basic service infrastructure
   - Configuration management
   - Single server management
   - Process supervision

2. **Phase 2: Enhanced Management**
   - User management
   - Application whitelisting
   - Comprehensive logging
   - CLI communication

3. **Phase 3: Advanced Features**
   - Multiple server support
   - High availability features
   - Advanced metrics
   - API integration