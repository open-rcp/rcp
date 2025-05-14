# RCP-CLI Migration Completion Plan

This document outlines the remaining steps to complete the migration of functionality from `rcp-cli` into `rcpd` and safely remove the standalone `rcp-cli` component.

## Remaining Command Modules to Implement

The following command modules need to be ported from `rcp-cli` to `rcpd`:

| Module | Status | Priority | Description |
|--------|--------|----------|-------------|
| app.rs | **Completed** | High | Application management commands |
| user.rs | **Completed** | High | User management commands |
| session.rs | **Completed** | High | Session management commands |
| completions.rs | **Completed** | Medium | Shell completion generation |
| diag.rs | **Completed** | Medium | Diagnostic commands |
| logs.rs | Commented out in mod.rs | Medium | Log viewing and management |
| auth.rs | Commented out in mod.rs | Low | Authentication management |
| batch.rs | Commented out in mod.rs | Low | Batch command execution |
| shell.rs | Commented out in mod.rs | Low | Interactive shell |

## Implementation Strategy

For each module:

1. Port the command implementation from `rcp-cli` to `rcpd/src/cli/commands/`
2. Update imports and dependencies as needed
3. Ensure proper feature-gating with `#[cfg(feature = "cli")]`
4. Test the command functionality
5. Add to the integration test suite

## Documentation Updates

After implementation is complete:

1. Update all documentation to reflect the new unified binary approach
2. Replace examples using `rcp-cli` with `rcpd` equivalents
3. Add deprecation notices to existing `rcp-cli` documentation

## Removal Process

Once migration is complete:

1. Update workspace Cargo.toml to remove `rcp-cli` as a member
2. Update any build scripts or workflows that reference `rcp-cli`
3. Create a final migration report
4. Remove the `rcp-cli` directory

## Timeline

- Phase 1 (High priority modules): Complete by May 30, 2025
- Phase 2 (Medium priority modules): Complete by June 15, 2025
- Phase 3 (Low priority modules): Complete by June 30, 2025
- Documentation updates: Complete by July 7, 2025
- Final removal: Target July 15, 2025

## Testing Strategy

For each ported command:

1. Unit tests for command functionality
2. Integration tests for command-line interface
3. End-to-end tests with actual daemon
4. Feature flag testing (with and without `api` feature)

## Related Issues

- Update issue tracking system with tasks for each module
- Track progress using milestones
