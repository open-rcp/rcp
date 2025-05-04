@echo off
echo ====================================
echo RCP Project - Windows Setup Script
echo ====================================
echo.

echo Checking for Rust installation...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust is not installed. Installing Rust...
    echo Please follow the rustup installer prompts...
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | cmd
) else (
    echo Rust is already installed.
    echo Updating Rust...
    rustup update
)

echo Configuring Rust toolchain...
rustup default stable
rustup component add rustfmt
rustup component add clippy

echo Setting up environment variables for OpenSSL...
set OPENSSL_DIR=C:\Program Files\OpenSSL-Win64
set OPENSSL_INCLUDE_DIR=C:\Program Files\OpenSSL-Win64\include
set OPENSSL_LIB_DIR=C:\Program Files\OpenSSL-Win64\lib

echo Checking project dependencies...
echo Running cargo check to verify dependencies...
cargo check

echo.
echo ====================================
echo Setup completed successfully!
echo ====================================
echo.
echo You can now build the project using the build.bat script.
echo.