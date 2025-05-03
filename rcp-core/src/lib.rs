// RCP Core - Rust Control Protocol Core Library
//
// Main library entry point that exports all components

mod error;
mod frame;
mod header;
mod protocol;
mod auth;
mod command;
pub mod utils;

pub use error::{Error, Result};
pub use frame::Frame;
pub use header::Header;
pub use protocol::{Protocol, ConnectionState};
pub use auth::{Auth, AuthMethod, AuthChallenge, AuthPayload, AuthResponse, SessionInfo};
pub use command::{Command, CommandId};

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