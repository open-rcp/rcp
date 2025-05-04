# Build and Run Script for RCP Server with Management UI
# This script builds the RCP management UI and server components and runs the integrated system

Write-Host "Building RCP Management UI and API..." -ForegroundColor Cyan

# Navigate to project root
$projectRoot = Split-Path -Parent (Split-Path -Parent $PSCommandPath)
Set-Location $projectRoot

# Configure environment for Windows builds
Write-Host "Configuring build environment..." -ForegroundColor Yellow

# Disable zstd-sys mmap feature which can cause issues on Windows
$env:CARGO_FEATURE_DISABLE_ZSTD_SYS_MMAP = "1"
Write-Host "Disabled zstd-sys mmap feature" -ForegroundColor Green

# Additional environment variable that might help with zstd-sys build
$env:CARGO_ZSTD_SYS_USE_PKG_CONFIG = "0"
Write-Host "Set additional environment variables for zstd-sys build" -ForegroundColor Green

# Step 1: Build management UI
Write-Host "Building management UI..." -ForegroundColor Yellow
Set-Location "$projectRoot\rcp-management-ui"

# Check if npm is available
$npmExists = Get-Command npm -ErrorAction SilentlyContinue

if ($npmExists) {
    # Install dependencies
    Write-Host "Installing npm dependencies..." -ForegroundColor Yellow
    npm install

    # Build the Svelte app
    Write-Host "Building Svelte app..." -ForegroundColor Yellow
    npm run build
    
    # Create static directory in management API if it doesn't exist
    if (-not (Test-Path "$projectRoot\rcp-management-api\static")) {
        New-Item -Path "$projectRoot\rcp-management-api\static" -ItemType Directory
    }
    
    # Copy the built files to the management API static directory
    Write-Host "Copying built files to management API..." -ForegroundColor Yellow
    Copy-Item -Path "$projectRoot\rcp-management-ui\build\*" -Destination "$projectRoot\rcp-management-api\static" -Recurse -Force
} else {
    Write-Host "npm not found. Skipping UI build." -ForegroundColor Red
}

# Step 2: Build the server components
Write-Host "Building RCP server components..." -ForegroundColor Yellow
Set-Location $projectRoot

# Build with management-api feature enabled
Write-Host "Building RCP server with management-api feature..." -ForegroundColor Yellow
cargo build --bin rcp-server --features management-api

# Check if build was successful
if ($LASTEXITCODE -eq 0) {
    # Step 3: Run the RCP server with management API enabled
    Write-Host "Starting RCP server with management API enabled..." -ForegroundColor Green
    cargo run --bin rcp-server --features management-api -- -v --mgmt-port 8081
} else {
    Write-Host "Build failed. Please check the errors above." -ForegroundColor Red
}

Write-Host "Server has stopped." -ForegroundColor Cyan