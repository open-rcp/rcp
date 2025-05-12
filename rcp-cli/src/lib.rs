//! RCP CLI Library
//!
//! This library provides the core functionality for the RCP command-line interface.
//! It's structured to be testable and modular, with separate components for different
//! aspects of CLI functionality.

// Re-export all modules for use in tests
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod service;
pub mod utils;

// Define main types used in commands
pub use cli::{Cli, CliConfig, ConnectionConfig, AuthConfig, UserInfo};
pub use error::CliError;
pub use service::{ServiceClient, ServiceStatus, ServerInfo};

// Re-export enums from main
pub use clap::Parser;
pub use clap::Subcommand;

#[derive(Subcommand)]
pub enum ServiceAction {
    /// Install RCP service
    Install,
    /// Uninstall RCP service
    Uninstall,
    /// Start RCP service
    Start,
    /// Stop RCP service
    Stop,
    /// Restart RCP service
    Restart,
    /// Display service status
    Status,
}

#[derive(Subcommand)]
pub enum AuthAction {
    /// Log in to the RCP service
    Login,
    /// Log out from the RCP service
    Logout,
    /// Show current authentication status
    Status,
}
