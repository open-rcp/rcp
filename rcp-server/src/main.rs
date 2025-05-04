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
use std::sync::Arc;
use tokio::sync::Mutex;

/// RCP Server - Remote Control Protocol Server
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

    /// Management API port
    #[clap(long, default_value = "8081")]
    mgmt_port: u16,
    
    /// Disable management API
    #[clap(long)]
    no_mgmt: bool,
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
    let server_handle = Arc::new(Mutex::new(server));
    
    // Start management API server if enabled
    if !cli.no_mgmt {
        #[cfg(feature = "management-api")]
        {
            let api_server_handle = server_handle.clone();
            tokio::spawn(async move {
                info!("Starting management API server on port {}", cli.mgmt_port);
                if let Err(e) = run_management_api_server(api_server_handle, cli.mgmt_port).await {
                    log::error!("Management API server error: {}", e);
                }
            });
        }
        
        #[cfg(not(feature = "management-api"))]
        {
            info!("Management API requested but the feature is not enabled. Build with --features management-api to enable it.");
        }
    }

    // Run RCP server
    {
        // Get a lock on the server
        let server_guard = server_handle.lock().await;
        // Clone the server to avoid ownership issues
        let server_instance = server_guard.clone();
        // Drop the guard to release the lock
        drop(server_guard);
        // Run the server
        server_instance.run().await?;
    }

    info!("Server shutdown complete");
    Ok(())
}

#[cfg(feature = "management-api")]
async fn run_management_api_server(
    server_handle: Arc<Mutex<Server>>, 
    port: u16
) -> Result<()> {
    use log::error;
    
    // Create management API configuration
    let mgmt_config = rcp_management_api::Config::new()
        .with_port(port)
        .with_server_handle(server_handle);
    
    // Run the management API server
    rcp_management_api::run_server(mgmt_config).await.map_err(|e| {
        error::Error::Other(format!("Management API server error: {}", e))
    })
}
