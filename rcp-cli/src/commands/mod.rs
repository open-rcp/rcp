pub mod service;
pub mod server;
pub mod session;
pub mod user;
pub mod config;
pub mod diag;
pub mod logs;

// Re-export handlers for easier importing
pub use service::handle_service_command;