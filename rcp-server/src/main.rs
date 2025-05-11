mod config;
mod error;
mod server;
mod service;
mod session;

use clap::Parser;
use config::ServerConfig;
use error::Result;
use log::{info, LevelFilter};
use server::Server;

/// RCP Server - Rust/Remote Control Protocol Server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Config file path
    #[clap(short, long, default_value = "config.toml")]
    config: String,

    /// Server address
    #[clap(short, long)]
    address: Option<String>,

    /// Server port
    #[clap(short, long)]
    port: Option<u16>,

    /// Enable verbose logging
    #[clap(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp_millis()
        .init();

    info!("RCP Server v{} starting...", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let mut config = ServerConfig::load(&cli.config)?;

    // Override with command line options if provided
    if let Some(address) = cli.address {
        config.address = address;
    }

    if let Some(port) = cli.port {
        config.port = port;
    }

    // Initialize server
    let server = Server::new(config);
    // Run RCP server
    server.run().await?;

    info!("Server shutdown complete");
    Ok(())
}
