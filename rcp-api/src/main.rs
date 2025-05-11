use anyhow::Result;
use clap::Parser;
use rcp_api::{init, ApiConfig};
use std::path::PathBuf;

/// Command line arguments for the RCP API server
#[derive(Parser)]
#[clap(name = "rcp-api", about = "RCP API Server", version)]
struct Args {
    /// Path to configuration file
    #[clap(short, long, value_parser)]
    config: Option<PathBuf>,

    /// Address to bind the API server to
    #[clap(short, long)]
    address: Option<String>,

    /// Port to bind the API server to
    #[clap(short = 'P', long)]
    port: Option<u16>,

    /// Socket path for RCP service connection
    #[clap(short, long)]
    socket: Option<String>,

    /// Log level (debug, info, warn, error)
    #[clap(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize logging
    let filter = format!("rcp_api={},tower_http=debug", args.log_level);
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    // Load configuration
    let mut config = if let Some(path) = args.config {
        ApiConfig::from_file(path)?
    } else {
        ApiConfig::default()
    };

    // Override config with command line arguments if provided
    if let Some(address) = args.address {
        config.bind_address = address;
    }

    if let Some(port) = args.port {
        config.port = port;
    }

    if let Some(socket) = args.socket {
        config.service_connection_string = socket;
    }

    // Initialize and start API server
    tracing::info!("Starting RCP API server");
    init(config).await?;

    Ok(())
}
