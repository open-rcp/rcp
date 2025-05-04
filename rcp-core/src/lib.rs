// RCP Core - Rust Control Protocol Core Library
//
// Main library entry point that exports all components

mod auth;
mod command;
mod error;
mod frame;
mod header;
mod protocol;
pub mod utils;

pub use auth::{Auth, AuthChallenge, AuthMethod, AuthPayload, AuthResponse, SessionInfo};
pub use command::{Command, CommandId};
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
