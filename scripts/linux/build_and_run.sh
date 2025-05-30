#!/bin/bash
# Build and Run Script for RCP Server
# This script builds the RCP server components and runs the system

echo -e "\033[0;36mBuilding RCP Components...\033[0m"

# Navigate to project root
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
PROJECT_ROOT=$(dirname "$(dirname "$SCRIPT_DIR")")
cd "$PROJECT_ROOT"

# Build the service components
echo -e "\033[0;33mBuilding RCP service components...\033[0m"
cd "$PROJECT_ROOT"
cargo build

# Run the RCP daemon
echo -e "\033[0;32mStarting RCP daemon...\033[0m"
cargo run --bin rcpdaemon -- -v

echo -e "\033[0;36mDaemon has stopped.\033[0m"

# NOTE: Future management UI and API components will be added here once they're created