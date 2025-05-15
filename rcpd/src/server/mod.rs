// Module for integrated server functionality
// This module contains the server components migrated from the separate rcp-server crate

pub mod config;
pub mod error;
// Apply clippy allow to avoid module inception warning
#[allow(clippy::module_inception)]
pub mod server;
pub mod session;
pub mod user;

// Re-export important items
pub use self::server::Server;
