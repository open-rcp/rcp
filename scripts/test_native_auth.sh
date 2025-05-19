#!/bin/bash
# filepath: /Volumes/EXT/repos/open-rcp/rcp/scripts/test_native_auth.sh
# 
# Integration test script for testing native authentication
#
# This script:
# 1. Creates a test configuration with native auth
# 2. Starts RCP daemon with this configuration
# 3. Tests authentication with various methods
# 4. Verifies permission mapping works
#

set -e

# Detect OS
OS="$(uname -s)"
echo "Detected OS: $OS"

# Set appropriate test group based on OS
case "$OS" in
  Darwin)
    TEST_GROUP="staff"  # macOS default group
    ;;
  Linux)
    TEST_GROUP="sudo"  # Linux common admin group
    # On some Linux distros, wheel may be used instead
    if getent group wheel > /dev/null 2>&1; then
      if id -Gn | grep -q wheel; then
        TEST_GROUP="wheel"
      fi
    fi
    ;;
  FreeBSD|OpenBSD|NetBSD|DragonFly)
    TEST_GROUP="wheel"  # BSD common admin group
    ;;
  MINGW*|MSYS*|CYGWIN*)
    TEST_GROUP="Administrators"  # Windows admin group
    ;;
  SunOS)
    TEST_GROUP="sys"   # Solaris common group
    ;;
  *)
    # Default to wheel for other Unix systems
    if [ -f /etc/passwd ]; then
      # Try to find a suitable admin group the current user belongs to
      for group in wheel operator staff admin sys adm; do
        if getent group $group > /dev/null 2>&1; then
          if id -Gn | grep -q $group; then
            TEST_GROUP="$group"
            break
          fi
        fi
      done
      
      # If no admin group found, just use the first non-system group the user is in
      if [ -z "$TEST_GROUP" ]; then
        TEST_GROUP=$(id -Gn | tr ' ' '\n' | grep -v "^[0-9]*$" | head -1)
      fi
    else
      echo "Unsupported OS: $OS"
      exit 1
    fi
    ;;
esac

# Get current user
CURRENT_USER="$(whoami)"
echo "Current user: $CURRENT_USER"

# Create temporary test configuration
TEST_CONFIG=$(mktemp)
cat > "$TEST_CONFIG" << EOL
# Test configuration for native auth
address = "127.0.0.1"
port = 8716
daemonize = false  # Run in foreground mode

# TLS configuration for the service
[tls]
enabled = false
cert_path = "cert.pem"
key_path = "key.pem"

[server]
address = "127.0.0.1"
port = 8717

[server.tls]
enabled = false
cert_path = "server-cert.pem"
key_path = "server-key.pem"

[server.auth]
required = true
provider = "native"
psk = "test_integration_key"
fallback_to_internal = true

[server.auth.native]
allow_all_users = false
require_group = "$TEST_GROUP"
permission_mapping = true
admin_groups = ["admin", "wheel", "Administrators", "sudo"]

[server.auth.native.permission_mappings]
"admin" = ["admin:*", "connect:*", "app:*"]
"staff" = ["connect:*", "app:safari"]
"sudo" = ["admin:*", "connect:*"]
"Administrators" = ["admin:*", "connect:*", "app:*"]

[api]
address = "127.0.0.1" 
port = 8718
EOL

echo "Created test configuration at $TEST_CONFIG"

# Start RCP daemon in background with test config
echo "Starting RCP daemon with native authentication..."
RCP_BIN="./target/debug/rcpd"

# Check if binary exists, build if needed
if [ ! -f "$RCP_BIN" ]; then
    echo "Building RCP daemon..."
    cargo build
fi

# Run the daemon in background without daemonizing
$RCP_BIN --config "$TEST_CONFIG" --foreground --verbose &
RCP_PID=$!

# Ensure we kill the daemon on exit
cleanup() {
    echo "Shutting down RCP daemon (PID: $RCP_PID)..."
    kill $RCP_PID 2>/dev/null || true
    rm -f "$TEST_CONFIG"
    echo "Test cleanup completed"
}
trap cleanup EXIT

# Wait for daemon to start
echo "Waiting for daemon to start..."
sleep 2

# Test authentication using the connect command
echo -e "\n=== Testing Authentication with Connect ==="
./target/debug/rcpc -H 127.0.0.1 -p 8717 connect --psk "test_integration_key"
if [ $? -eq 0 ]; then
    echo "✅ Authentication succeeded"
else
    echo "❌ Authentication failed"
    exit 1
fi

# Try to execute a command as a test of permissions
echo -e "\n=== Testing Permissions with Execute ==="
./target/debug/rcpc -H 127.0.0.1 -p 8717 execute --psk "test_integration_key" "status"
if [ $? -eq 0 ]; then
    echo "✅ Command execution succeeded"
else
    echo "❌ Command execution failed"
    exit 1
fi

echo -e "\n=== Native Authentication Test Completed Successfully ==="
exit 0
