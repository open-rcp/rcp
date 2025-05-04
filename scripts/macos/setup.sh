#!/bin/bash

echo "===================================="
echo "RCP Project - macOS Setup Script"
echo "===================================="
echo

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo "Homebrew is not installed. Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
else
    echo "Homebrew is already installed. Updating..."
    brew update
fi

# Install build dependencies
echo "Installing build dependencies..."
brew install cmake pkg-config

# Install OpenSSL
echo "Installing OpenSSL..."
brew install openssl@3

# Set OpenSSL environment variables
echo "Setting up OpenSSL environment variables..."
OPENSSL_PATH=$(brew --prefix openssl@3)
echo "export OPENSSL_DIR=$OPENSSL_PATH" >> ~/.zshrc
echo "export OPENSSL_INCLUDE_DIR=$OPENSSL_PATH/include" >> ~/.zshrc
echo "export OPENSSL_LIB_DIR=$OPENSSL_PATH/lib" >> ~/.zshrc
echo "export PATH=$OPENSSL_PATH/bin:\$PATH" >> ~/.zshrc

# Export variables for current session
export OPENSSL_DIR=$OPENSSL_PATH
export OPENSSL_INCLUDE_DIR=$OPENSSL_PATH/include
export OPENSSL_LIB_DIR=$OPENSSL_PATH/lib
export PATH=$OPENSSL_PATH/bin:$PATH

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed. Updating..."
    rustup update
fi

# Configure Rust toolchain
echo "Configuring Rust toolchain..."
rustup default stable
rustup component add rustfmt
rustup component add clippy

# Install LLVM for additional build dependencies
echo "Installing LLVM..."
brew install llvm
echo "export PATH=$(brew --prefix llvm)/bin:\$PATH" >> ~/.zshrc

# Verify project dependencies
echo "Checking project dependencies..."
cargo check

echo
echo "===================================="
echo "Setup completed successfully!"
echo "===================================="
echo
echo "You can now build the project using the build.sh script."
echo "NOTE: You may need to restart your terminal or run 'source ~/.zshrc' to apply environment variables."
echo