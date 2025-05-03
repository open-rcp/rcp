mod config;
mod error;
mod server;
mod session;
mod service;

use clap::Parser;
use config::ServerConfig;
use error::Result;
use log::{info, LevelFilter};
use server::Server;

#[derive(Parser, Debug)]
#[clap(name = "rcp-server", version = env!("CARGO_PKG_VERSION"))]
/// RCP Server: Remote application control server using Rust Control Protocol
struct Cli {
    /// Path to the configuration file
    #[clap(short, long, default_value = "config.toml")]
    config: String,

    /// Server address to bind to
    #[clap(short, long)]
    address: Option<String>,

    /// Server port to listen on
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
    
    // Initialize and run server
    let server = Server::new(config);
    server.run().await?;
    
    info!("Server shutdown complete");
    Ok(())
}