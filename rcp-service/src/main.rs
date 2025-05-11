mod config;
mod daemon;
mod error;
mod instance;
mod lifecycle;
mod manager;
mod platform;
mod service;
mod user;

use clap::Parser;
use anyhow::Result;
use log::{info, LevelFilter};
use crate::manager::ServiceManager;

/// RCP Service - Rust/Remote Control Protocol Runtime Service
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

    // Process commands if provided
    if let Some(cmd) = cli.command {
        match cmd {
            ServiceCommand::Start => {
                info!("Starting RCP Service...");
                if cli.foreground {
                    // Run in foreground
                    let (shutdown_tx, _shutdown_rx) = tokio::sync::mpsc::channel(1);
                    let work_dir = std::path::PathBuf::from(&cli.config).parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
                    let manager = ServiceManager::new(work_dir, shutdown_tx);
                    manager.start().await?;
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
        let work_dir = std::path::PathBuf::from(&cli.config).parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
        let manager = ServiceManager::new(work_dir, shutdown_tx);
        manager.start().await?;
    } else {
        // No command, run as daemon
        info!("Starting RCP Service as daemon...");
        daemon::start(&cli.config)?;
    }

    info!("RCP Service exiting");
    Ok(())
}
