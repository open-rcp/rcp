// Module for integrated server functionality
// This module contains the server components migrated from the separate rcp-server crate

pub mod config;
pub mod error;
pub mod server;
pub mod session;
pub mod user;

// Re-export important items
pub use self::server::Server;
