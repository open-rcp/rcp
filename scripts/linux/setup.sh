#!/bin/bash

echo "===================================="
echo "RCP Project - Linux Setup Script"
echo "===================================="
echo

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
  echo "Please run this script with sudo or as root to install system dependencies."
  exit 1
fi

# Update package lists
echo "Updating package lists..."
apt-get update

# Install essential build tools
echo "Installing essential build tools..."
apt-get install -y build-essential pkg-config

# Install SSL development libraries
echo "Installing SSL development libraries..."
apt-get install -y libssl-dev

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed. Updating..."
    rustup update
fi

# Configure Rust toolchain
echo "Configuring Rust toolchain..."
rustup default stable
rustup component add rustfmt
rustup component add clippy

# Verify project dependencies
echo "Checking project dependencies..."
cargo check

echo
echo "===================================="
echo "Setup completed successfully!"
echo "===================================="
echo
echo "You can now build the project using the build.sh script."
echo