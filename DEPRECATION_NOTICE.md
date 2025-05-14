# RCP-CLI DEPRECATION NOTICE

**Important: This is a standalone component that is being deprecated.**

The functionality of `rcp-cli` is being migrated into the `rcpd` daemon as an optional feature. This creates a unified binary for production environments and simplifies deployment.

## Migration Timeline

- **May 2025:** Initial migration of core commands (service, server, config)
- **June 2025:** Complete migration of all commands
- **July 2025:** Full deprecation and removal of standalone `rcp-cli`

## Using the new CLI functionality

The CLI functionality is now available in the `rcpd` binary when built with the `cli` feature:

```bash
# Build with CLI feature
cargo build -p rcpd --features cli

# Use CLI functionality
rcpd server list
rcpd config show
rcpd user list
```

For development, you can use:

```bash
# Run with CLI features
cargo run -p rcpd --features cli -- [arguments]

# Example: check service status
cargo run -p rcpd --features cli -- service status
```

## Benefits of the Unified Approach

1. **Simplified Deployment**: Single binary with all functionality
2. **Consistent Versioning**: CLI and daemon versions are always in sync
3. **Reduced Overhead**: Shared code and resources
4. **Improved Integration**: Direct access to daemon functionality

## Transition Assistance

If you have scripts or tools that depend on `rcp-cli`, please update them to use `rcpd` with the appropriate commands. The command structure and arguments remain largely the same to ensure backwards compatibility.
