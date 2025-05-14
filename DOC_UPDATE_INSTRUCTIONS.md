# Documentation Updates for CLI Migration

This file contains instructions and replacements for updating documentation to reflect the migration from `rcp-cli` to `rcpd`.

## Common Replacements

When updating documentation, use these common replacements:

| Old Content | New Content |
|-------------|-------------|
| `rcp-cli service start` | `rcpd service start` |
| `rcp-cli server list` | `rcpd server list` |
| `rcp-cli config show` | `rcpd config show` |
| `rcp-cli diag run` | `rcpd diag run` |
| `cargo run -p rcp-cli -- [arguments]` | `cargo run -p rcpd --features cli -- [arguments]` |
| `cargo test -p rcp-cli` | `cargo test -p rcpd --features cli` |

## Files to Update

The following files need to be updated with the new command structure:

1. `/Volumes/EXT/repos/open-rcp/rcp/README.md`
2. `/Volumes/EXT/repos/open-rcp/rcp/QUICKSTART.md`
3. `/Volumes/EXT/repos/open-rcp/rcp/docs/rcp-cli.md` (add deprecation notice)
4. `/Volumes/EXT/repos/open-rcp/rcp/docs/rcpd.md` (add CLI information)
5. `/Volumes/EXT/repos/open-rcp/rcp/docs/rcpd-installation.md` (update CLI usage)

## Example Updates

### For README.md

```markdown
# Before:
cargo run -p rcp-cli -- status

# After:
cargo run -p rcpd --features cli -- service status
```

### For docs/rcp-cli.md

Add to top of file:

```markdown
> **DEPRECATED**: The functionality of rcp-cli is being migrated into the rcpd daemon.
> Please see [RCPD](rcpd.md) documentation for the new unified CLI.
```

### For docs/rcpd.md

Add a new section:

```markdown
## Command Line Interface

RCPD includes a comprehensive command-line interface when built with the `cli` feature:

```bash
# Build with CLI features
cargo build -p rcpd --features cli

# Use CLI functionality (examples)
rcpd service status       # Check service status
rcpd server list          # List configured servers
rcpd config show          # Show configuration
rcpd user list            # List users
```

The CLI functionality offers the same commands previously available in the standalone rcp-cli tool.
```

## Command Structure Reference

Create a new file `docs/rcpd-cli.md` that documents the command structure of the integrated CLI. This will serve as the reference for the new unified CLI.
