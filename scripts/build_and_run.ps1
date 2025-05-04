# Build and Run Script for RCP Server with Management UI
# This script builds the RCP management UI and server components and runs the integrated system

Write-Host "Building RCP Management UI and API..." -ForegroundColor Cyan

# Navigate to project root
$projectRoot = Split-Path -Parent (Split-Path -Parent $PSCommandPath)
Set-Location $projectRoot

# Configure environment for Windows builds
Write-Host "Configuring build environment..." -ForegroundColor Yellow

# Create a temporary directory for downloading LLVM/Clang if needed
$tempDir = "$projectRoot\temp"
if (-not (Test-Path $tempDir)) {
    New-Item -Path $tempDir -ItemType Directory | Out-Null
}

# Function to download and extract LLVM/Clang
function Setup-Libclang {
    $llvmVersion = "16.0.0"
    $llvmUrl = "https://github.com/llvm/llvm-project/releases/download/llvmorg-$llvmVersion/LLVM-$llvmVersion-win64.exe"
    $llvmInstaller = "$tempDir\LLVM-$llvmVersion-win64.exe"
    $llvmDir = "$projectRoot\llvm"

    if (-not (Test-Path "$llvmDir\bin\libclang.dll")) {
        Write-Host "libclang.dll not found. Setting up LLVM/Clang..." -ForegroundColor Yellow
        
        # Download LLVM installer if needed
        if (-not (Test-Path $llvmInstaller)) {
            Write-Host "Downloading LLVM/Clang $llvmVersion..." -ForegroundColor Yellow
            
            try {
                [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
                Invoke-WebRequest -Uri $llvmUrl -OutFile $llvmInstaller
            }
            catch {
                Write-Host "Failed to download LLVM/Clang. Please install it manually." -ForegroundColor Red
                Write-Host "Download from: $llvmUrl" -ForegroundColor Red
                Write-Host "Then set LIBCLANG_PATH environment variable to the path containing libclang.dll" -ForegroundColor Red
                exit 1
            }
        }
        
        # Create LLVM directory if needed
        if (-not (Test-Path $llvmDir)) {
            New-Item -Path $llvmDir -ItemType Directory | Out-Null
        }
        
        # Extract libclang.dll from the installer using 7zip or similar
        # Note: This is a simplified approach; you may need a proper installer
        Write-Host "Extracting libclang.dll from LLVM installer..." -ForegroundColor Yellow
        
        # Try to find 7zip
        $sevenZip = $null
        $potentialPaths = @(
            "C:\Program Files\7-Zip\7z.exe",
            "C:\Program Files (x86)\7-Zip\7z.exe"
        )
        
        foreach ($path in $potentialPaths) {
            if (Test-Path $path) {
                $sevenZip = $path
                break
            }
        }
        
        if ($sevenZip) {
            # Extract using 7zip
            & $sevenZip x $llvmInstaller "-o$llvmDir" "bin\libclang.dll" -r -y | Out-Null
        }
        else {
            # Alternative: Use Start-Process to run the installer silently
            Write-Host "7-Zip not found. Attempting to install LLVM..." -ForegroundColor Yellow
            Start-Process -FilePath $llvmInstaller -ArgumentList "/S" -Wait
            
            # Copy libclang.dll from the installed location
            $defaultInstallPath = "C:\Program Files\LLVM"
            if (Test-Path "$defaultInstallPath\bin\libclang.dll") {
                Copy-Item "$defaultInstallPath\bin\libclang.dll" -Destination "$llvmDir\bin\"
            }
            else {
                Write-Host "Could not locate libclang.dll. Please install LLVM manually." -ForegroundColor Red
                exit 1
            }
        }
    }
    
    # Set environment variables for the build process
    if (Test-Path "$llvmDir\bin\libclang.dll") {
        $env:LIBCLANG_PATH = "$llvmDir\bin"
        Write-Host "LIBCLANG_PATH set to: $env:LIBCLANG_PATH" -ForegroundColor Green
    }
    else {
        Write-Host "libclang.dll still not found. Build may fail." -ForegroundColor Red
    }
}

# Setup libclang for bindgen
Setup-Libclang

# Disable zstd-sys mmap feature which can cause issues on Windows
$env:CARGO_FEATURE_DISABLE_ZSTD_SYS_MMAP = "1"
Write-Host "Disabled zstd-sys mmap feature" -ForegroundColor Green

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

# Step 3: Run the RCP server with management API enabled
Write-Host "Starting RCP server with management API enabled..." -ForegroundColor Green
cargo run --bin rcp-server --features management-api -- -v --mgmt-port 8081

Write-Host "Server has stopped." -ForegroundColor Cyan