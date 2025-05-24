# GitHub Copilot Instructions for RCP Project

## Environment Context
- **Operating System**: Windows 11
- **Primary Shell**: PowerShell 7.x
- **Development Environment**: VS Code with Rust toolchain
- **Git Repository**: C:\Users\Administrator\Documents\GitHub\rcp

## Command Execution Guidelines

### Always Use PowerShell Commands
When suggesting terminal commands, always use PowerShell syntax instead of Unix/Linux commands:

**File Operations:**
- Use `Get-ChildItem` or `ls` instead of `ls` (when PowerShell aliases don't work)
- Use `Copy-Item` instead of `cp`
- Use `Move-Item` instead of `mv` 
- Use `Remove-Item` instead of `rm`
- Use `New-Item` instead of `touch` or `mkdir`

**Network Operations:**
- Use `Invoke-WebRequest` instead of `curl` (PowerShell's curl alias has different parameters)
- Use `Invoke-RestMethod` for API calls

**Text Processing:**
- Use `Select-String` instead of `grep`
- Use `Get-Content` and `Set-Content` instead of `cat` and redirection when aliases fail

**Archive Operations:**
- Use `Expand-Archive` for .zip files
- Use external tools like 7-Zip for .7z files

### Rust Development Specifics
- **Toolchain**: stable-x86_64-pc-windows-gnu (GNU toolchain to avoid MSVC linker issues)
- **MinGW-w64 Location**: C:\Users\Administrator\mingw64\bin (added to PATH)
- **Build Command**: `cargo build` (works with GNU toolchain)

### Project Structure
This is a Rust workspace with multiple crates:
- `rcpcore`: Core protocol implementation
- `rcpcli`: Command-line client
- `rcpdaemon`: Background daemon service

### Git Best Practices
- Clean up temporary files before committing
- Use meaningful commit messages
- Stage changes carefully to avoid including build artifacts

### Common Issues to Avoid
1. **Don't suggest Unix commands** - Always use PowerShell equivalents
2. **Don't retry failed commands** - Analyze the error and suggest the correct PowerShell approach
3. **Check for required tools** - Verify tools like 7-Zip are available before suggesting extraction commands
4. **PATH considerations** - Remember that MinGW-w64 tools are available in the current session PATH

### Development Workflow
1. Use `cargo check` for quick syntax validation
2. Use `cargo build` for full compilation
3. Use `cargo test` for running tests
4. Use `cargo clippy` for linting (if installed)

## Notes
- This project was recently refactored from `rcpc`/`rcpd`/`rcpp` to `rcpcli`/`rcpcore`/`rcpdaemon`
- The GNU toolchain is preferred over MSVC to avoid Visual Studio dependencies
- MinGW-w64 tools are available system-wide for native Windows development