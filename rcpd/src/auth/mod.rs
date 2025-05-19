pub mod factory;
pub mod improved_native;
pub mod manager;
pub mod mock_provider;
pub mod native_macos;
pub mod provider;

#[cfg(target_os = "windows")]
pub mod native_windows;

#[cfg(target_os = "linux")]
pub mod native_linux;

#[cfg(all(unix, not(any(target_os = "macos", target_os = "linux"))))]
pub mod native_unix;

// Re-export key components
pub use factory::{AuthConfig, AuthProviderFactory, AuthProviderType, NativeAuthConfig};
pub use improved_native::EnhancedGroupManagement;
pub use manager::AuthManager;
pub use provider::AuthProvider;
