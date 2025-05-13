mod config;
mod daemon;
mod error;
mod instance;
mod lifecycle;
mod manager;
mod platform;
mod server;
mod service;
mod user;

// API module is conditionally compiled when the "api" feature is enabled
#[cfg(feature = "api")]
mod api;

use crate::manager::ServiceManager;
use anyhow::Result;
use clap::Parser;
use log::{info, LevelFilter};

/// RCPD - Rust/Remote Control Protocol Daemon
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Config file path
    #[clap(short, long, default_value = "service.toml")]
    config: String,

    /// Run in foreground (no daemon)
    #[clap(short, long)]
    foreground: bool,

    /// Enable verbose logging
    #[clap(short, long)]
    verbose: bool,

    /// Service command to execute
    #[clap(subcommand)]
    command: Option<ServiceCommand>,
}

#[derive(Parser, Debug)]
enum ServiceCommand {
    /// Start the service
    Start,

    /// Stop the service
    Stop,

    /// Restart the service
    Restart,

    /// Get service status
    Status,

    /// Install service
    Install,

    /// Uninstall service
    Uninstall,
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

    info!("RCP Service v{} initializing...", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config_file = &cli.config;
    let config = match config::ServiceConfig::from_file(config_file) {
        Ok(cfg) => {
            info!("Configuration loaded from {}", config_file);
            cfg
        },
        Err(e) => {
            info!("Failed to load config from {}: {}. Using defaults.", config_file, e);
            config::ServiceConfig::default()
        }
    };

    // Process commands if provided
    if let Some(cmd) = cli.command {
        match cmd {
            ServiceCommand::Start => {
                #[cfg(feature = "api")]
                info!("Starting RCP Service (with integrated server and API)...");
                
                #[cfg(not(feature = "api"))]
                info!("Starting RCP Service (with integrated server)...");
                
                if cli.foreground {
                    // Run in foreground
                    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
                    let work_dir = std::path::PathBuf::from(&cli.config)
                        .parent()
                        .unwrap_or_else(|| std::path::Path::new("."))
                        .to_path_buf();
                    
                    // Initialize service manager with config
                    let mut manager = ServiceManager::new(work_dir, config.clone(), shutdown_tx);
                    
                    // Start the service (including integrated server)
                    manager.start().await?;
                    
                    // Wait for shutdown signal
                    tokio::select! {
                        _ = shutdown_rx.recv() => {
                            info!("Shutdown signal received");
                        },
                        _ = tokio::signal::ctrl_c() => {
                            info!("Ctrl+C received, shutting down");
                        }
                    }
                    
                    // Stop the service
                    manager.stop().await?;
                } else {
                    // Run as daemon
                    daemon::start(&cli.config)?;
                }
            }
            ServiceCommand::Stop => {
                info!("Stopping RCP Service...");
                daemon::stop()?;
            }
            ServiceCommand::Restart => {
                info!("Restarting RCP Service...");
                daemon::restart(&cli.config)?;
            }
            ServiceCommand::Status => {
                let status = daemon::status()?;
                println!("RCP Service status: {}", status);
            }
            ServiceCommand::Install => {
                info!("Installing RCP Service...");
                platform::install_service()?;
                println!("RCP Service installed successfully");
            }
            ServiceCommand::Uninstall => {
                info!("Uninstalling RCP Service...");
                platform::uninstall_service()?;
                println!("RCP Service uninstalled successfully");
            }
        }
    } else if cli.foreground {
        // No command, run in foreground
        info!("Starting RCP Service in foreground...");
        let (shutdown_tx, _shutdown_rx) = tokio::sync::mpsc::channel(1);
        let work_dir = std::path::PathBuf::from(&cli.config)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();
        let config = config::ServiceConfig::default();
        let mut manager = ServiceManager::new(work_dir, config, shutdown_tx);
        manager.start().await?;
    } else {
        // No command, run as daemon
        info!("Starting RCP Service as daemon...");
        daemon::start(&cli.config)?;
    }

    info!("RCP Service exiting");
    Ok(())
}
