use clap::Parser;
use log::{info, LevelFilter};
use rcp_ws_bridge::{start_bridge, BridgeConfig};

/// RCP WebSocket Bridge - Proxy for browser clients
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// WebSocket host to bind to
    #[clap(short, long, default_value = "0.0.0.0")]
    ws_host: String,

    /// WebSocket port to listen on
    #[clap(short, long, default_value = "8080")]
    ws_port: u16,

    /// RCP server host to connect to
    #[clap(short, long, default_value = "localhost")]
    rcp_host: String,

    /// RCP server port
    #[clap(short, long, default_value = "9277")]
    rcp_port: u16,

    /// Enable verbose logging
    #[clap(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    info!("RCP WebSocket Bridge starting...");

    // Configure bridge
    let config = BridgeConfig {
        ws_host: cli.ws_host,
        ws_port: cli.ws_port,
        rcp_host: cli.rcp_host,
        rcp_port: cli.rcp_port,
    };

    // Start the bridge
    start_bridge(config).await?;

    Ok(())
}
