# rcpdaemon (RCP Daemon) Build and Installation Guide

This guide provides comprehensive instructions for building and installing the RCP Daemon (rcpdaemon), which combines server and API functionality into a single integrated component.

## Building rcpdaemon

### Prerequisites

Before building rcpdaemon, ensure you have the following dependencies installed:

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

You can also build rcpdaemon directly using Cargo:

```bash
# Debug build
cargo build -p rcpdaemon

# Release build (recommended for production)
cargo build --release -p rcpdaemon
```

## Installation

### Windows

1. Build rcpdaemon in release mode:
   ```powershell
   scripts\windows\build.bat --release --daemon
   ```

2. Copy the executable to a permanent location:
   ```powershell
   mkdir -p "C:\Program Files\RCP"
   copy "target\release\rcpdaemon.exe" "C:\Program Files\RCP\"
   copy "config.toml" "C:\Program Files\RCP\"
   ```

3. Install as a Windows service:
   ```powershell
   cd "C:\Program Files\RCP"
   .\rcpdaemon.exe --install-service
   ```

4. Start the service:
   ```powershell
   sc start rcpdaemon
   ```

5. Configure Windows Firewall (if necessary):
   ```powershell
   netsh advfirewall firewall add rule name="RCP Daemon" dir=in action=allow program="C:\Program Files\RCP\rcpdaemon.exe" enable=yes
   ```

### Linux

1. Build rcpdaemon in release mode:
   ```bash
   ./scripts/linux/build.sh --release --daemon
   ```

2. Create required directories:
   ```bash
   sudo mkdir -p /usr/local/bin /etc/rcpdaemon /var/lib/rcpdaemon /var/log/rcpdaemon
   ```

3. Copy the executable and configuration:
   ```bash
   sudo cp target/release/rcpdaemon /usr/local/bin/
   sudo cp config.toml /etc/rcpdaemon/
   sudo cp rcpdaemon/systemd/rcpdaemon.service /etc/systemd/system/
   ```

4. Set proper permissions:
   ```bash
   sudo chown -R root:root /etc/rcpdaemon /usr/local/bin/rcpdaemon
   sudo chmod 755 /usr/local/bin/rcpdaemon
   sudo chmod 644 /etc/rcpdaemon/config.toml /etc/systemd/system/rcpdaemon.service
   ```

5. Reload systemd, enable and start the service:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable rcpdaemon
   sudo systemctl start rcpdaemon
   ```

6. Check the service status:
   ```bash
   sudo systemctl status rcpdaemon
   ```

### macOS

1. Build rcpdaemon in release mode:
   ```bash
   ./scripts/macos/build.sh --release --daemon
   ```

2. Create required directories:
   ```bash
   sudo mkdir -p /usr/local/bin /usr/local/etc/rcpdaemon /var/lib/rcpdaemon /var/log/rcpdaemon
   ```

3. Copy the executable and configuration:
   ```bash
   sudo cp target/release/rcpdaemon /usr/local/bin/
   sudo cp config.toml /usr/local/etc/rcpdaemon/
   ```

4. Copy the launchd service file:
   ```bash
   # For system-wide installation
   sudo cp rcpdaemon/launchd/com.devstroop.rcpdaemon.plist /Library/LaunchDaemons/
   
   # OR for user-specific installation
   cp rcpdaemon/launchd/com.devstroop.rcpdaemon.plist ~/Library/LaunchAgents/
   ```

5. Load and start the service:
   ```bash
   # For system-wide installation
   sudo launchctl load -w /Library/LaunchDaemons/com.devstroop.rcpdaemon.plist
   
   # OR for user-specific installation
   launchctl load -w ~/Library/LaunchAgents/com.devstroop.rcpdaemon.plist
   ```

6. Verify the daemon is running:
   ```bash
   launchctl list | grep rcpdaemon
   ```

## Configuration

rcpdaemon uses a TOML configuration file, typically located at:

- Windows: `C:\Program Files\RCP\config.toml`
- Linux: `/etc/rcpdaemon/config.toml`
- macOS: `/usr/local/etc/rcpdaemon/config.toml`

### Example Configuration

```toml
# rcpdaemon Configuration

# Basic configuration
address = "0.0.0.0"
port = 8716
work_dir = "/var/lib/rcpdaemon"
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
cert_path = "/etc/rcpdaemon/tls/cert.pem"
key_path = "/etc/rcpdaemon/tls/key.pem"
```

## Running rcpdaemon Manually

For development or testing purposes, you can run rcpdaemon directly:

```bash
# Run with default configuration
rcpdaemon

# Run with a specific configuration file
rcpdaemon -c /path/to/config.toml

# Run in the foreground with verbose output
rcpdaemon -f -v

# Show all available options
rcpdaemon --help
```

## Uninstalling

### Windows
```powershell
# Stop and remove service
sc stop rcpdaemon
sc delete rcpdaemon

# Remove files
Remove-Item -Path "C:\Program Files\RCP" -Recurse -Force
```

### Linux
```bash
# Stop and disable service
sudo systemctl stop rcpdaemon
sudo systemctl disable rcpdaemon
sudo rm /etc/systemd/system/rcpdaemon.service
sudo systemctl daemon-reload

# Remove files
sudo rm /usr/local/bin/rcpdaemon
sudo rm -rf /etc/rcpdaemon /var/lib/rcpdaemon /var/log/rcpdaemon
```

### macOS
```bash
# Unload service
sudo launchctl unload /Library/LaunchDaemons/com.devstroop.rcpdaemon.plist
# OR
launchctl unload ~/Library/LaunchAgents/com.devstroop.rcpdaemon.plist

# Remove files
sudo rm /usr/local/bin/rcpdaemon
sudo rm /Library/LaunchDaemons/com.devstroop.rcpdaemon.plist
# OR
rm ~/Library/LaunchAgents/com.devstroop.rcpdaemon.plist
sudo rm -rf /usr/local/etc/rcpdaemon /var/lib/rcpdaemon /var/log/rcpdaemon
```

## Troubleshooting

### Common Issues

1. **Permission Denied**:
   - Ensure you have proper permissions for the configuration directories
   - Check file ownership and permissions

2. **Service Won't Start**:
   - Check logs for errors: `/var/log/rcpdaemon/rcpdaemon.log` (Linux/macOS) or Event Viewer (Windows)
   - Verify configuration file syntax
   - Ensure ports are not in use by other applications

3. **Cannot Connect to Daemon**:
   - Check firewall settings
   - Verify the daemon is running (`systemctl status rcpdaemon`, `launchctl list | grep rcpdaemon`, or `sc query rcpdaemon`)
   - Ensure the configuration has the correct bind address and port

### Viewing Logs

#### Windows
Check the Windows Event Viewer for logs from the rcpdaemon service.

#### Linux
```bash
sudo journalctl -u rcpdaemon
# Or check the log file
sudo cat /var/log/rcpdaemon/rcpdaemon.log
```

#### macOS
```bash
sudo cat /var/log/rcpdaemon/rcpdaemon.log
# Or check system logs
log show --predicate 'processImagePath CONTAINS "rcpdaemon"'
```
