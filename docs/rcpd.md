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
- CLI interface for administration

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
│  │ Connection & Protocol   │  │ CLI Interface      │   │
│  │ Handling                │  │ & Commands         │   │
│  └─────────────────────────┘  └────────────────────┘   │
└────────────────────────────────────────────────────────┘
```

## Component Integration

RCPD integrates the following functionalities:

1. **Server Component**: Handles client connections, sessions, authentication, and protocol processing
2. **API Component**: Optional REST API endpoints for remote administration (enabled via feature flag)
3. **CLI Interface**: Command-line interface for service administration and management
4. **Service Core**: Handles lifecycle management, configuration, user management, etc.

## Dependencies

- **rcpp**: Uses the RCP Protocol library for message format, commands, and frame handling
- **tokio**: For asynchronous runtime and I/O
- **clap**: For command-line argument parsing
- **serde**: For configuration serialization/deserialization

## Feature Flags

RCPD uses feature flags to enable optional components:

- **api**: Enables the REST API component
- **cli**: Enables the command-line interface
- **all**: Enables all available features

## Command-Line Interface

RCPD includes a command-line interface for service management:

```
USAGE:
    rcpd [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -c, --config <FILE>    Path to configuration file
    -f, --foreground       Run in foreground (no daemon)
    -v, --verbose          Enable verbose logging
    -h, --help             Print help information
    -V, --version          Print version information

SUBCOMMANDS:
    service      Service management commands
    app          Application management
    config       Configuration management
    session      Session management
    user         User management
    completions  Generate shell completions
    diag         Diagnostic tools
    help         Print help information
```

## System Integration

RCPD provides integration with operating system service managers:

- **systemd** service units for Linux
- **launchd** plist for macOS
- **Windows Service** registration for Windows

## Configuration

RCPD's configuration includes settings for:

- Server listening address and port
- TLS certificate paths
- Authentication methods
- Application definitions
- User permissions
- Service behavior
- API endpoint configuration (when enabled)

Configuration can be loaded from:

- Default path (/etc/rcpd/config.toml)
- Custom path specified with --config option
- Environment variables

## API Endpoints (Feature-gated)

When built with the `api` feature, RCPD exposes RESTful endpoints:

- `/api/v1/status` - Service status information
- `/api/v1/apps` - Application management
- `/api/v1/users` - User management
- `/api/v1/sessions` - Active session management
- `/api/v1/config` - Configuration management

## Security

RCPD implements several security measures:

1. TLS encryption for all TCP connections
2. Authentication for client connections
3. Authorization for API access
4. Secure storage of credentials
5. Privilege separation when running as a system service
