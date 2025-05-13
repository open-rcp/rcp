# RCP Developer Quick Start Guide

This guide provides a fast path to get started developing for the RCP project. Follow these steps to set up your development environment and begin working with the codebase.

## Prerequisites

- **Rust** (1.75+) - Install using [rustup](https://rustup.rs/)
- **Git** - For repository management
- **VS Code** (recommended) with Rust Analyzer extension
- **Platform-specific dependencies**:
  - **Windows**: Visual Studio Build Tools with C++ toolchain
  - **Linux**: build-essential, pkg-config, libssl-dev
  - **macOS**: Xcode Command Line Tools

## Quick Setup

### 1. Clone the Repository

```bash
git clone https://github.com/open-rcp/rcp.git
cd rcp
```

### 2. Make Scripts Executable (Linux/macOS only)

```bash
chmod +x scripts/linux/*.sh scripts/macos/*.sh
```

### 3. Run Setup Script

#### Windows:
```
scripts\windows\setup.bat
```

#### Linux:
```bash
sudo ./scripts/linux/setup.sh
```

#### macOS:
```bash
./scripts/macos/setup.sh
```

### 4. Build All Components

#### Windows:
```
scripts\windows\build.bat --all
```

#### Linux/macOS:
```bash
./scripts/linux/build.sh --all
# or
./scripts/macos/build.sh --all
```

## Development Workflow

### 1. Build and Run Specific Components

To build and run the daemon (which includes server functionality):

```bash
# macOS
./scripts/macos/build.sh --daemon --run-daemon
```

To build and run the client:

```bash
# macOS
./scripts/macos/build.sh --client --run-client
```

### 2. Testing

Run specific tests:

```bash
cargo test -p rcp-core
cargo test -p rcpd
cargo test -p rcp-client
```

Run all tests:

```bash
cargo test --all
```

### 3. Development Strategies

#### Component-by-Component Development

For initial development, focus on one component at a time:

1. Start with core protocol changes in `rcp-core`
2. Update the daemon implementation in `rcpd` 
3. Modify the client library in `rcp-client`
4. Update CLI commands in `rcp-cli`
5. Adapt the admin interface and end-user applications last

#### Full Stack Development

For feature development that spans multiple components:

1. Create a branch for your feature
2. Define interfaces in `rcp-core` first
3. Implement server-side functionality in `rcpd`
4. Add client-side support in `rcp-client`
5. Update CLI/admin/desk components as needed
6. Write tests for each component
7. Submit a PR with changes to all affected components

## Useful Cargo Commands

```bash
# Check for errors without building
cargo check --all

# Format code
cargo fmt --all

# Check for linting issues
cargo clippy --all -- -D warnings

# Build documentation
cargo doc --no-deps --open

# Clean build artifacts
cargo clean
```

## Project Structure Overview

See [DIRECTORY_STRUCTURE.md](DIRECTORY_STRUCTURE.md) for a detailed breakdown of the project's organization.

## Key Documentation

- [Architecture Overview](docs/architecture.md)
- [Protocol Specification](docs/protocol-specification.md)
- [Development Guidelines](docs/development-guidelines.md)
- [Project Outline](docs/project-outline.md)

## Getting Help

If you need assistance, please:

1. Check the documentation first
2. Look for similar issues in the issue tracker
3. Ask questions in the project discussions
4. Reach out to the project maintainers

Happy coding!
