//! # RCP Client
//! 
//! Client library for the Rust Control Protocol (RCP).
//! 
//! This crate provides the client-side implementation of the RCP protocol,
//! allowing applications to connect to RCP servers and use their services.

pub mod client;
pub mod error;
pub mod service;

pub use client::{Client, ClientConfig, ClientEvent, ClientState, ReconnectConfig};
pub use error::{Error, Result};
pub use service::{ClipboardService, DisplayService, InputService, Service, ServiceEventHandler};

/// Re-export core types from rcp-core
pub use rcp_core::{AuthMethod, CommandId, Frame, SessionInfo};