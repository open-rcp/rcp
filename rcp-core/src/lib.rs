//! # RCP Core - Rust/Remote Control Protocol Core Library
//!
//! This crate provides the core functionality for the Rust/Remote Control Protocol (RCP),
//! a versatile protocol designed for managing applications across various deployment scenarios,
//! from individual use to enterprise data centers.
//! 
//! ## Architecture
//!
//! The RCP Core establishes the foundation for all RCP components:
//!
//! - Protocol specification and versioning
//! - Frame format for binary communication
//! - Authentication and security mechanisms
//! - Command definition and handling
//! - Error types and handling
//!
//! ## Deployment Flexibility
//!
//! The protocol is designed to support multiple deployment models:
//!
//! ### Individual Use
//! - Self-contained operation for personal application management
//! - Simplified configuration for single-user environments
//! - Reduced resource requirements
//!
//! ### SaaS/Data Center Model
//! - Centralized application management (admin/server side)
//! - User isolation (users only access their reserved apps/directories)
//! - Server clustering with single administration point
//! - Resource allocation controlled by administrators
//!
//! ## Core Implementation Philosophy
//!
//! The core library is designed to be fully open source and standard,
//! with no division between different editions or tiers.
//! All core modules (rcp-core, rcp-server, rcp-service, rcp-client, rcp-ws-bridge)
//! provide the same complete functionality regardless of deployment scenario.
//! Any edition-specific features would only exist in the interface projects
//! (rcp-admin, rcp-desk, rcp-api, rcp-cli).

mod auth;
mod command;
mod error;
mod frame;
mod header;
mod protocol;
pub mod utils;

pub use auth::{Auth, AuthChallenge, AuthMethod, AuthPayload, AuthResponse, SessionInfo};
pub use command::{Command, CommandId, LaunchAppCommand};
pub use error::{Error, Result};
pub use frame::Frame;
pub use header::Header;
pub use protocol::{ConnectionState, Protocol};

/// Current protocol version
pub const PROTOCOL_VERSION: u8 = 0x01;

/// Default port for RCP connections
pub const DEFAULT_PORT: u16 = 9277;

/// RCP Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
