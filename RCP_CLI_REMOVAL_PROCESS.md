# RCP-CLI Removal Process

This document outlines the complete process for removing the standalone `rcp-cli` component and transitioning to the integrated CLI functionality in `rcpd`.

## Prerequisites

Before removing `rcp-cli`, ensure that:

1. ✅ All essential command modules have been migrated to `rcpd`
   - All command modules have been implemented in `rcpd/src/cli/commands/`
   - Feature flags (`#[cfg(feature = "cli")]`) are applied correctly
2. ⏳ All CLI functionality works correctly with `rcpd --features cli`
   - Need to fix compilation errors in the migrated modules
   - Need to run tests for the CLI functionality
3. ⏳ Documentation has been updated to reference the new command structure

## Step-by-Step Removal Process

### 1. Complete Migration of Command Modules

Use the provided `port_command_module.sh` script to migrate remaining command modules:

```bash
# Make the script executable (if not already)
chmod +x port_command_module.sh

# Port each remaining module
./port_command_module.sh app
./port_command_module.sh user
./port_command_module.sh session
./port_command_module.sh completions
./port_command_module.sh diag
```

### 2. Update Module References

After porting a module, check for and fix:
- Import paths (change `rcp_cli::*` to `crate::cli::*`)
- Feature flags (ensure `#[cfg(feature = "cli")]` is used appropriately)
- Service client integration

### 3. Test CLI Functionality

Test each ported command to ensure it works correctly:

```bash
# Build rcpd with CLI feature
cargo build -p rcpd --features cli

# Test various commands
./target/debug/rcpd service status
./target/debug/rcpd server list
./target/debug/rcpd config show
./target/debug/rcpd app list
```

### 4. Update Documentation

Update all documentation files to reference `rcpd` instead of `rcp-cli`:

1. Add the deprecation notice to `docs/rcp-cli.md`
2. Update examples in README.md and QUICKSTART.md
3. Add CLI information to `docs/rcpd.md`
4. Create a new CLI reference document if needed

See `DOC_UPDATE_INSTRUCTIONS.md` for specific changes.

### 5. Run the Removal Script

Once migration is complete and tested, run the removal script:

```bash
# Make the script executable (if not already)
chmod +x remove_rcp_cli.sh

# Run the script
./remove_rcp_cli.sh
```

This script will:
1. Create a backup of the `rcp-cli` directory
2. Remove `rcp-cli` from the workspace in Cargo.toml
3. Delete the `rcp-cli` directory
4. Update references in README.md

### 6. Final Verification

After removal:

1. Build the project to ensure nothing breaks:
   ```bash
   cargo build --all-targets
   ```

2. Run tests to ensure functionality works:
   ```bash
   cargo test --all
   ```

3. Verify the CLI functionality still works:
   ```bash
   cargo run -p rcpd --features cli -- service status
   ```

## Reverting (If Needed)

If issues are discovered after removal, the backup can be restored:

```bash
# Find the backup directory
ls -la backup/

# Restore from backup
cp -r backup/rcp-cli-backup-YYYYMMDD/rcp-cli ./
```

Then, restore the workspace entry in Cargo.toml:

```toml
[workspace]
members = [
    "rcp-core",
    "rcp-client",
    "rcp-ws-bridge",
    "rcpd",
    "examples",
    "rcp-cli",  # Re-add this line
    "flutter_rcp_client/rust_bridge",
]
```
