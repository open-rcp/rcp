# Migration Plan: Integrating RCP-CLI into RCPD

This document outlines the plan to migrate the RCP-CLI functionality into RCPD, creating a single unified binary for production environments.

## Motivation

The current architecture separates the client tool (rcp-cli) from the daemon (rcpd). While this separation provides some benefits in terms of code organization and separation of concerns, it creates multiple binaries to maintain and distribute. For production environments, a single binary approach simplifies deployment, versioning, and management.

## Migration Strategy

### 1. Feature Flag Structure

We'll implement CLI functionality in RCPD behind an optional feature flag:

```toml
[features]
default = []
api = ["axum", "tower-http", ...]
cli = ["clap", "clap_complete", ...] 
```

### 2. Code Organization

The new structure will be:
- `rcpd/src/cli/` - New directory for CLI-specific code
- `rcpd/src/cli/commands/` - Command implementations
- `rcpd/src/main.rs` - Updated to include CLI command routing

### 3. Command Structure

The new integrated command structure will be:
```
rcpd [daemon options] [COMMAND]

COMMANDS:
  daemon      Manage the RCP daemon (start, stop, status)
  server      Server management commands
  app         Application management
  session     Manage active sessions
  user        User management
  config      Configuration commands
  ...
```

### 4. Implementation Steps

1. Add CLI feature flag to rcpd's Cargo.toml
2. Create cli module structure in rcpd
3. Port commands from rcp-cli to rcpd/src/cli/commands/
4. Update command structure in main.rs
5. Implement shared configuration
6. Update documentation
7. Add tests for new functionality
8. Deprecation notice for standalone rcp-cli

### 5. Migration Timeline

- Phase 1: Basic CLI command structure and framework
- Phase 2: Port core commands (service, server, app)
- Phase 3: Port secondary commands (user, config, etc.)
- Phase 4: Testing and documentation
- Phase 5: Deprecation of standalone rcp-cli

## API and Client Compatibility

We will ensure that the CLI functionality:
- Functions correctly whether the API feature is enabled or not
- Maintains backward compatibility with existing API clients
- Uses the same API endpoints when available to avoid duplication

## Documentation Updates

- Update RCPD documentation to include CLI functionality
- Create new usage guides for the combined binary
- Add deprecation notices to rcp-cli docs
