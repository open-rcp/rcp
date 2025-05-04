// A thin wrapper around zstd-sys that ensures it builds from source
// without pkg-config or bindgen requirements

// Re-export everything from zstd-sys
pub use zstd_sys::*;

// Export version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");