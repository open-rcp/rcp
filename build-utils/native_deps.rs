// Native dependency build utilities for RCP project
// This file provides helper functions for managing native libraries

use std::env;
use std::path::PathBuf;

/// Configure zstd-sys to build from source rather than relying on pkg-config
pub fn configure_zstd() {
    // Force build from source
    println!("cargo:rustc-env=ZSTD_SYS_FORCE_FROM_SOURCE=1");
    println!("cargo:rustc-env=ZSTD_SYS_USE_PKG_CONFIG=0");
    println!("cargo:rustc-env=ZSTD_SYS_USE_BINDGEN=0");
    
    // Tell cargo to invalidate the built crate whenever this script changes
    println!("cargo:rerun-if-changed=build.rs");
}

/// Configure AWS-LC-sys to avoid CMake requirements
pub fn configure_aws_lc() {
    println!("cargo:rustc-env=AWS_LC_SYS_STATIC=1");
    println!("cargo:rustc-env=AWS_LC_SYS_USE_SHARED=0");
    println!("cargo:rustc-env=AWS_LC_SYS_VENDORED=1");
}

/// Setup environment for a project that uses native libraries
pub fn setup_native_build_environment() {
    configure_zstd();
    configure_aws_lc();
    
    println!("cargo:rerun-if-env-changed=ZSTD_SYS_USE_PKG_CONFIG");
    println!("cargo:rerun-if-env-changed=ZSTD_SYS_FORCE_FROM_SOURCE");
    println!("cargo:rerun-if-env-changed=AWS_LC_SYS_STATIC");
}