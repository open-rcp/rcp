#!/bin/bash
# Build and Run Script for RCP Server with Management UI
# This script builds the RCP management UI and server components and runs the integrated system

echo -e "\033[0;36mBuilding RCP Management UI and API...\033[0m"

# Navigate to project root
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
PROJECT_ROOT=$(dirname "$(dirname "$SCRIPT_DIR")")
cd "$PROJECT_ROOT"

# Step 1: Build management UI
echo -e "\033[0;33mBuilding management UI...\033[0m"
cd "$PROJECT_ROOT/rcp-management-ui"

# Check if npm is available
if command -v npm &> /dev/null; then
    # Install dependencies
    echo -e "\033[0;33mInstalling npm dependencies...\033[0m"
    npm install

    # Build the Svelte app
    echo -e "\033[0;33mBuilding Svelte app...\033[0m"
    npm run build
    
    # Create static directory in management API if it doesn't exist
    mkdir -p "$PROJECT_ROOT/rcp-management-api/static"
    
    # Copy the built files to the management API static directory
    echo -e "\033[0;33mCopying built files to management API...\033[0m"
    cp -r "$PROJECT_ROOT/rcp-management-ui/build/"* "$PROJECT_ROOT/rcp-management-api/static/"
else
    echo -e "\033[0;31mnpm not found. Skipping UI build.\033[0m"
fi

# Step 2: Build the server components
echo -e "\033[0;33mBuilding RCP server components...\033[0m"
cd "$PROJECT_ROOT"
cargo build

# Step 3: Run the RCP server with management API enabled
echo -e "\033[0;32mStarting RCP server with management API enabled...\033[0m"
cargo run --bin rcp-server -- -v --mgmt-port 8081

echo -e "\033[0;36mServer has stopped.\033[0m"