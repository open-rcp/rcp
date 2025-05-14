// RCP Daemon Library (rcpd)
// Exposes the core functionality of the RCP daemon

// Public modules
pub mod config;
pub mod error;
pub mod instance;
pub mod lifecycle;
pub mod manager;
pub mod server;
pub mod service;
pub mod user;

// Feature-gated modules
#[cfg(feature = "api")]
pub mod api;

#[cfg(feature = "cli")]
pub mod cli;

// Platform-specific modules (private)
mod platform;

// Re-export common types for external usage
pub use config::ServiceConfig;
pub use error::{Result, ServiceError};
pub use manager::ServiceManager;
pub use service::Service;
