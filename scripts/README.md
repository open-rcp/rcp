# RCP Build and Development Scripts

This directory contains scripts to help with building, configuring, and running the RCP project components across different operating systems.

## Directory Structure

```
scripts/
├── README.md           # This file
├── linux/              # Linux-specific scripts
│   ├── build.sh        # Build script for Linux
│   ├── build_and_run.sh # Build and run script 
│   └── setup.sh        # Environment setup script
├── macos/              # macOS-specific scripts
│   ├── build.sh        # Build script for macOS
│   └── setup.sh        # Environment setup script
└── windows/            # Windows-specific scripts
    ├── build.bat       # Build script for Windows
    ├── README.md       # Windows-specific instructions
    └── setup.bat       # Environment setup script
```

## Common Workflow

### Initial Setup

Before building the project, run the setup script for your platform:

**macOS:**
```bash
chmod +x scripts/macos/setup.sh
./scripts/macos/setup.sh
```

**Linux:**
```bash
chmod +x scripts/linux/setup.sh
sudo ./scripts/linux/setup.sh
```

**Windows:**
```
scripts\windows\setup.bat
```

### Building Components

The build scripts support building any of the RCP components:

- `rcpp`: RCP Protocol library
- `rcpc`: RCP Client library and CLI
- `rcpd`: RCP Daemon with integrated server
- `examples`: Example applications

**macOS/Linux:**
```bash
# Build the RCP daemon in release mode with API feature
./scripts/macos/build.sh --release --rcpd --api

# Build all components in debug mode
./scripts/linux/build.sh --all

# Build and run the RCP client
./scripts/macos/build.sh --rcpc --run-rcpc
```

**Windows:**
```
# Build the RCP daemon in release mode with API feature
scripts\windows\build.bat --release rcpd --api

# Build all components in debug mode
scripts\windows\build.bat all

# Build and run the RCP client
scripts\windows\build.bat rcpc --run
```

### Available Options

All build scripts support similar options:

| Option | Description |
|--------|-------------|
| `--release` | Build in release mode (optimized) |
| `--debug` | Build in debug mode (default) |
| `--rcpp` | Build the RCP protocol library |
| `--rcpc` | Build the RCP client library/CLI |
| `--rcpd` | Build the RCP daemon |
| `--examples` | Build the example applications |
| `--all` | Build all components |
| `--run-rcpd`/`--run` | Run the RCP daemon after building |
| `--run-rcpc` | Run the RCP client after building |
| `--api` | Enable the API feature when building rcpd |

## Feature Flags

The build scripts support the following feature flags:

- `--api`: Enable the API component in rcpd (RESTful API endpoints)

## Adding New Scripts

When adding new scripts:

1. Place them in the appropriate OS-specific directory
2. Make sure they follow consistent naming and parameter conventions
3. Update this README.md to document the new script
4. On Unix systems (macOS/Linux), set executable permissions:
   ```bash
   chmod +x scripts/{os}/new_script.sh
   ```
