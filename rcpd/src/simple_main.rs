// Main entry point for RCPD
mod config;
mod daemon;
mod daemon_install;
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

// CLI module is conditionally compiled when the "cli" feature is enabled
#[cfg(feature = "cli")]
mod cli;

use anyhow::Result;
use clap::Parser;
use log::{info, LevelFilter};
use std::path::PathBuf;

/// RCPD - RCP Daemon
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

    /// Output in JSON format
    #[clap(long)]
    json: bool,

    /// Command to execute
    #[clap(subcommand)]
    command: Option<ServiceCommand>,
}

#[derive(Parser, Debug, Clone)]
pub enum ServiceCommand {
    /// Start the daemon
    Start,

    /// Stop the daemon
    Stop,

    /// Restart the daemon
    Restart,

    /// Show daemon status
    Status,

    /// Install as system service
    Install,

    /// Uninstall system service
    Uninstall,
}

/// Main entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Set log level
    let log_level = if cli.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp_millis()
        .init();

    info!("RCPD v{} initializing...", env!("CARGO_PKG_VERSION"));

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

    // Handle command or run daemon by default
    match cli.command {
        Some(ServiceCommand::Start) => {
            run_daemon(&cli, config).await?;
        },
        Some(ServiceCommand::Stop) => {
            info!("Stopping RCP service...");
            daemon::stop()?;
        },
        Some(ServiceCommand::Restart) => {
            info!("Restarting RCP service...");
            daemon::stop()?;
            run_daemon(&cli, config).await?;
        },
        Some(ServiceCommand::Status) => {
            let status = daemon::status()?;
            println!("RCP Service Status: {}", status);
        },
        Some(ServiceCommand::Install) => {
            info!("Installing RCP service...");
            daemon_install::install(&cli.config)?;
        },
        Some(ServiceCommand::Uninstall) => {
            info!("Uninstalling RCP service...");
            daemon_install::uninstall()?;
        },
        None => {
            // No command specified, run daemon
            run_daemon(&cli, config).await?;
        }
    }

    Ok(())
}

/// Run RCPD daemon
async fn run_daemon(cli: &Cli, config: config::ServiceConfig) -> Result<()> {
    #[cfg(feature = "api")]
    info!("Starting RCPD (with API)...");
    
    #[cfg(not(feature = "api"))]
    info!("Starting RCPD...");
    
    // Check if we should daemonize
    if !cli.foreground {
        let work_dir = std::env::current_dir()?;
        info!("Daemonizing process in {}", work_dir.display());
        daemon::daemonize(&work_dir)?;
    }
    
    // Start the daemon
    daemon::start(config, std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}
