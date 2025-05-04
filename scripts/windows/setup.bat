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

echo Installing additional dependencies...
echo Installing OpenSSL (required for crypto operations)...
echo NOTE: If you don't have Chocolatey installed, you may need to install OpenSSL manually.
echo       Visit https://slproweb.com/products/Win32OpenSSL.html to download the installer.
where choco >nul 2>&1
if %errorlevel% equ 0 (
    choco install openssl -y
) else (
    echo Chocolatey not found. Please install OpenSSL manually.
)

echo Checking for additional build dependencies...
where cmake >nul 2>&1
if %errorlevel% neq 0 (
    echo CMake not found. Installing CMake...
    where choco >nul 2>&1
    if %errorlevel% equ 0 (
        choco install cmake -y
    ) else (
        echo Chocolatey not found. Please install CMake manually from https://cmake.org/download/
    )
) else (
    echo CMake is already installed.
)

echo Setting up environment variables...
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