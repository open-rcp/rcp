//! RCP Client Library
//!
//! This library provides a client implementation for the Rust/Remote Control Protocol (RCP).
//! It allows applications to connect to RCP servers and use their services like display
//! streaming, input control, clipboard sharing, and file transfers.

pub mod client;
pub mod connection_string;
pub mod error;
pub mod service;

pub use client::{Client, ClientBuilder, ClientConfig, ClientState};
pub use connection_string::ConnectionString;
pub use error::{Error, Result};
pub use service::{builtin, Service, ServiceClient, ServiceFactory, ServiceMessage, ServiceType};

/// Default port for RCP connections
pub const DEFAULT_PORT: u16 = rcpp::DEFAULT_PORT;

/// Default connection timeout in seconds
pub const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 10;

/// Default keep-alive interval in seconds
pub const DEFAULT_KEEP_ALIVE_SECS: u64 = 30;

/// Default reconnection delay in milliseconds
pub const DEFAULT_RECONNECT_DELAY_MS: u64 = 2000;

/// A simple example of using the RCP client:
///
/// ```rust,no_run
/// use rcpc::{Client, ServiceType};
/// use tokio::time::{sleep, Duration};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create the client
/// let client = Client::builder()
///     .host("192.168.1.100")
///     .port(8000)
///     .client_name("My RCP Client")
///     .auth_psk("my_secret_key")
///     .build();
///
/// // Connect and authenticate
/// client.connect_and_authenticate().await?;
///
/// // Start the client message processor
/// client.start().await?;
///
/// // Subscribe to the display service
/// let display_service = client.subscribe_service(ServiceType::Display).await?;
///
/// // Subscribe to the input service
/// let input_service = client.subscribe_service(ServiceType::Input).await?;
///
/// // Keep the client running
/// sleep(Duration::from_secs(60)).await;
///
/// // Disconnect when done
/// client.disconnect().await?;
/// # Ok(())
/// # }
/// ```
#[doc(hidden)]
pub struct Examples;
