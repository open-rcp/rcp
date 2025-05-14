//! CLI module for RCPD
//!
//! This module provides CLI functionality for the RCPD daemon.

#[cfg(feature = "cli")]
pub mod commands;

#[cfg(feature = "cli")]
pub mod utils;

#[cfg(feature = "cli")]
pub mod config;

#[cfg(feature = "cli")]
pub mod error;

#[cfg(feature = "cli")]
pub mod types;
