# RCP Project - Windows Build Instructions

This folder contains simplified build scripts for Windows that avoid the need for LLVM/libclang dependencies.

## Build Scripts

- `build.bat` - Main build script that builds the project without requiring libclang
  - Usage: `build.bat [--release/-r] [server|client|ws-bridge|all]`
  - Examples:
    - `build.bat` - Build the server component in debug mode
    - `build.bat --release client` - Build the client component in release mode
    - `build.bat -r all` - Build all components in release mode

## How Dependencies Are Managed

This project uses Rust's `.cargo/config.toml` configuration to avoid the need for libclang/LLVM dependencies that would otherwise be required by `bindgen`. The configuration forces all system crates to:

1. Use pre-compiled bindings
2. Use static linking with vendored dependencies
3. Disable bindgen completely

This approach allows you to build the project without installing any additional dependencies.