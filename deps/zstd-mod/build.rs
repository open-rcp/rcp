fn main() {
    // Force zstd-sys to build from source
    println!("cargo:rustc-env=ZSTD_SYS_FORCE_FROM_SOURCE=1");
    println!("cargo:rustc-env=ZSTD_SYS_USE_PKG_CONFIG=0");
    println!("cargo:rustc-env=ZSTD_SYS_USE_BINDGEN=0");
    
    // Tell cargo to invalidate the built crate whenever this script changes
    println!("cargo:rerun-if-changed=build.rs");
}