//! CLI commands module
//!
//! This module contains implementations of CLI commands for RCPD.

#[cfg(feature = "cli")]
pub mod app;

#[cfg(feature = "cli")]
pub mod server;

#[cfg(feature = "cli")]
pub mod service;

#[cfg(feature = "cli")]
pub mod session;

#[cfg(feature = "cli")]
pub mod user;

#[cfg(feature = "cli")]
pub mod config;

#[cfg(feature = "cli")]
pub mod completions;

#[cfg(feature = "cli")]
pub mod diag;

// Future modules to implement:
// #[cfg(feature = "cli")]
// pub mod logs;
//
// #[cfg(feature = "cli")]
// pub mod auth;
//
// #[cfg(feature = "cli")]
// pub mod batch;
//
// #[cfg(feature = "cli")]
// pub mod shell;
