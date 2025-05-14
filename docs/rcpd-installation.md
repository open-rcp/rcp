# RCPD (RCP Daemon) Build and Installation Guide

This guide provides comprehensive instructions for building and installing the RCP Daemon (RCPD), which combines server and API functionality into a single integrated component.

## Building RCPD

### Prerequisites

Before building RCPD, ensure you have the following dependencies installed:

#### Common Dependencies
- Rust toolchain (install via [rustup](https://rustup.rs/))
- Git

#### Windows-Specific Dependencies
- Visual Studio Build Tools with C++ toolchain
- pkg-config and CMake (can be installed via Chocolatey)

#### Linux-Specific Dependencies
- build-essential package
- pkg-config
- libssl-dev
- cmake

#### macOS-Specific Dependencies
- Xcode Command Line Tools
- pkg-config and cmake (via Homebrew)

### Building from Source

#### Using Build Scripts (Recommended)

The project provides platform-specific build scripts that handle all necessary steps:

##### Windows
```powershell
# Build the daemon in debug mode
scripts\windows\build.bat --daemon

# Build the daemon in release mode
scripts\windows\build.bat --release --daemon

# Build and immediately run the daemon
scripts\windows\build.bat --daemon --run-daemon
```

##### Linux
```bash
# Make the script executable (if needed)
chmod +x scripts/linux/build.sh

# Build the daemon in debug mode
./scripts/linux/build.sh --daemon

# Build the daemon in release mode
./scripts/linux/build.sh --release --daemon

# Build and immediately run the daemon
./scripts/linux/build.sh --daemon --run-daemon
```

##### macOS
```bash
# Make the script executable (if needed)
chmod +x scripts/macos/build.sh

# Build the daemon in debug mode
./scripts/macos/build.sh --daemon

# Build the daemon in release mode
./scripts/macos/build.sh --release --daemon

# Build and immediately run the daemon
./scripts/macos/build.sh --daemon --run-daemon
```

#### Using Cargo Directly

You can also build RCPD directly using Cargo:

```bash
# Debug build
cargo build -p rcpd

# Release build (recommended for production)
cargo build --release -p rcpd
```

## Installation

### Windows

1. Build RCPD in release mode:
   ```powershell
   scripts\windows\build.bat --release --daemon
   ```

2. Copy the executable to a permanent location:
   ```powershell
   mkdir -p "C:\Program Files\RCP"
   copy "target\release\rcpd.exe" "C:\Program Files\RCP\"
   copy "config.toml" "C:\Program Files\RCP\"
   ```

3. Install as a Windows service:
   ```powershell
   cd "C:\Program Files\RCP"
   .\rcpd.exe --install-service
   ```

4. Start the service:
   ```powershell
   sc start rcpd
   ```

5. Configure Windows Firewall (if necessary):
   ```powershell
   netsh advfirewall firewall add rule name="RCP Daemon" dir=in action=allow program="C:\Program Files\RCP\rcpd.exe" enable=yes
   ```

### Linux

1. Build RCPD in release mode:
   ```bash
   ./scripts/linux/build.sh --release --daemon
   ```

2. Create required directories:
   ```bash
   sudo mkdir -p /usr/local/bin /etc/rcpd /var/lib/rcpd /var/log/rcpd
   ```

3. Copy the executable and configuration:
   ```bash
   sudo cp target/release/rcpd /usr/local/bin/
   sudo cp config.toml /etc/rcpd/
   sudo cp rcpd/systemd/rcpd.service /etc/systemd/system/
   ```

4. Set proper permissions:
   ```bash
   sudo chown -R root:root /etc/rcpd /usr/local/bin/rcpd
   sudo chmod 755 /usr/local/bin/rcpd
   sudo chmod 644 /etc/rcpd/config.toml /etc/systemd/system/rcpd.service
   ```

5. Reload systemd, enable and start the service:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable rcpd
   sudo systemctl start rcpd
   ```

6. Check the service status:
   ```bash
   sudo systemctl status rcpd
   ```

### macOS

1. Build RCPD in release mode:
   ```bash
   ./scripts/macos/build.sh --release --daemon
   ```

2. Create required directories:
   ```bash
   sudo mkdir -p /usr/local/bin /usr/local/etc/rcpd /var/lib/rcpd /var/log/rcpd
   ```

3. Copy the executable and configuration:
   ```bash
   sudo cp target/release/rcpd /usr/local/bin/
   sudo cp config.toml /usr/local/etc/rcpd/
   ```

4. Copy the launchd service file:
   ```bash
   # For system-wide installation
   sudo cp rcpd/launchd/com.devstroop.rcpd.plist /Library/LaunchDaemons/
   
   # OR for user-specific installation
   cp rcpd/launchd/com.devstroop.rcpd.plist ~/Library/LaunchAgents/
   ```

5. Load and start the service:
   ```bash
   # For system-wide installation
   sudo launchctl load -w /Library/LaunchDaemons/com.devstroop.rcpd.plist
   
   # OR for user-specific installation
   launchctl load -w ~/Library/LaunchAgents/com.devstroop.rcpd.plist
   ```

6. Verify the daemon is running:
   ```bash
   launchctl list | grep rcpd
   ```

## Configuration

RCPD uses a TOML configuration file, typically located at:

- Windows: `C:\Program Files\RCP\config.toml`
- Linux: `/etc/rcpd/config.toml`
- macOS: `/usr/local/etc/rcpd/config.toml`

### Example Configuration

```toml
# RCPD Configuration

# Basic configuration
address = "0.0.0.0"
port = 8716
work_dir = "/var/lib/rcpd"
log_level = "info"

# Server-specific configuration
[server]
address = "0.0.0.0"
port = 8717
max_connections = 100
connection_timeout = 30
idle_timeout = 300

# API configuration (only used when built with the 'api' feature)
[api]
enabled = true
address = "127.0.0.1"
port = 8080
auth_token_expiry = 3600

# TLS configuration
[tls]
enabled = false
cert_path = "/etc/rcpd/tls/cert.pem"
key_path = "/etc/rcpd/tls/key.pem"
```

## Running RCPD Manually

For development or testing purposes, you can run RCPD directly:

```bash
# Run with default configuration
rcpd

# Run with a specific configuration file
rcpd -c /path/to/config.toml

# Run in the foreground with verbose output
rcpd -f -v

# Show all available options
rcpd --help
```

## Uninstalling

### Windows
```powershell
# Stop and remove service
sc stop rcpd
sc delete rcpd

# Remove files
Remove-Item -Path "C:\Program Files\RCP" -Recurse -Force
```

### Linux
```bash
# Stop and disable service
sudo systemctl stop rcpd
sudo systemctl disable rcpd
sudo rm /etc/systemd/system/rcpd.service
sudo systemctl daemon-reload

# Remove files
sudo rm /usr/local/bin/rcpd
sudo rm -rf /etc/rcpd /var/lib/rcpd /var/log/rcpd
```

### macOS
```bash
# Unload service
sudo launchctl unload /Library/LaunchDaemons/com.devstroop.rcpd.plist
# OR
launchctl unload ~/Library/LaunchAgents/com.devstroop.rcpd.plist

# Remove files
sudo rm /usr/local/bin/rcpd
sudo rm /Library/LaunchDaemons/com.devstroop.rcpd.plist
# OR
rm ~/Library/LaunchAgents/com.devstroop.rcpd.plist
sudo rm -rf /usr/local/etc/rcpd /var/lib/rcpd /var/log/rcpd
```

## Troubleshooting

### Common Issues

1. **Permission Denied**:
   - Ensure you have proper permissions for the configuration directories
   - Check file ownership and permissions

2. **Service Won't Start**:
   - Check logs for errors: `/var/log/rcpd/rcpd.log` (Linux/macOS) or Event Viewer (Windows)
   - Verify configuration file syntax
   - Ensure ports are not in use by other applications

3. **Cannot Connect to Daemon**:
   - Check firewall settings
   - Verify the daemon is running (`systemctl status rcpd`, `launchctl list | grep rcpd`, or `sc query rcpd`)
   - Ensure the configuration has the correct bind address and port

### Viewing Logs

#### Windows
Check the Windows Event Viewer for logs from the rcpd service.

#### Linux
```bash
sudo journalctl -u rcpd
# Or check the log file
sudo cat /var/log/rcpd/rcpd.log
```

#### macOS
```bash
sudo cat /var/log/rcpd/rcpd.log
# Or check system logs
log show --predicate 'processImagePath CONTAINS "rcpd"'
```
