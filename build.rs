// Root build script for RCP project
// This file configures the build process for native dependencies

use std::env;

fn main() {
    // Tell cargo to invalidate the built crate whenever this script changes
    println!("cargo:rerun-if-changed=build.rs");
    
    // Force zstd-sys to build from source rather than using pkg-config
    env::set_var("ZSTD_SYS_FORCE_FROM_SOURCE", "1");
    env::set_var("ZSTD_SYS_USE_PKG_CONFIG", "0");
    env::set_var("ZSTD_SYS_USE_BINDGEN", "0");
    
    // Print these variables so cargo knows to use them for all builds
    println!("cargo:rustc-env=ZSTD_SYS_FORCE_FROM_SOURCE=1");
    println!("cargo:rustc-env=ZSTD_SYS_USE_PKG_CONFIG=0");
    println!("cargo:rustc-env=ZSTD_SYS_USE_BINDGEN=0");
    
    // Enable tracking of environment variable changes
    println!("cargo:rerun-if-env-changed=ZSTD_SYS_FORCE_FROM_SOURCE");
    println!("cargo:rerun-if-env-changed=ZSTD_SYS_USE_PKG_CONFIG");
    println!("cargo:rerun-if-env-changed=ZSTD_SYS_USE_BINDGEN");
}