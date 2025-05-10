use clap::{Parser, Subcommand, Command};
use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;
use clap_complete::Shell;

mod cli;
mod commands;
mod config;
mod error;
mod service;
mod utils;

use cli::{CliConfig, Cli};

#[derive(Parser)]
#[clap(name = "rcp-cli", about = "RCP command-line interface", version)]
struct Args {
    /// Path to configuration file
    #[clap(short, long, value_parser)]
    config: Option<PathBuf>,

    /// Log level (debug, info, warn, error)
    #[clap(short, long, default_value = "info")]
    log_level: String,

    /// Output in JSON format
    #[clap(short, long)]
    json: bool,

    /// Suppress non-error output
    #[clap(short, long)]
    quiet: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage RCP service
    Service {
        /// Service action to perform
        #[arg(value_enum)]
        action: ServiceAction,
        
        /// Auto-start service on boot (only for install action)
        #[arg(long, requires = "action", required_if_eq("action", "install"))]
        auto_start: Option<bool>,
        
        /// User to run service as (only for install action)
        #[arg(long, requires = "action")]
        user: Option<String>,
    },
    /// Manage remote servers
    Server {
        /// Server action to perform
        action: ServerAction,
    },
    /// Manage user sessions
    Session {
        /// Session action to perform
        action: SessionAction,
    },
    /// Manage RCP users
    User {
        /// User action to perform
        action: UserAction,
    },
    /// Manage configuration
    Config {
        /// Config action to perform
        action: ConfigAction,
    },
    /// Run diagnostics
    Diag {
        /// Diagnostic action to perform
        action: DiagAction,
    },
    /// View logs
    Logs {
        /// Log level filter
        #[arg(short, long, default_value = "info")]
        level: String,
        /// Maximum number of log entries
        #[arg(short, long, default_value = "100")]
        limit: usize,
        /// Show logs since timestamp
        #[arg(long)]
        since: Option<String>,
    },
    /// Authentication commands
    Auth {
        /// Auth action to perform
        action: AuthAction,
    },
    /// Start interactive shell
    Shell,
    /// Run commands from file
    Batch {
        /// Path to batch file
        file: String,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ServiceAction {
    Status,
    Start,
    Stop,
    Restart,
    Install,
    Uninstall,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ServerAction {
    List,
    Start,
    Stop,
    Restart,
    Delete,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SessionAction {
    List,
    Disconnect,
    Info,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum UserAction {
    List,
    Add,
    Remove,
    Update,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ConfigAction {
    Show,
    Set,
    Reset,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DiagAction {
    Network,
    System,
    All,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AuthAction {
    Login,
    Logout,
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&args.log_level))
        .format_timestamp_millis()
        .init();

    // Load configuration
    let config_path = match args.config {
        Some(path) => path,
        None => config::default_config_path()?,
    };

    let mut cli_config = if config_path.exists() {
        CliConfig::from_file(&config_path)?
    } else {
        CliConfig::default()
    };

    // Set output format
    cli_config.json_output = args.json;
    cli_config.quiet = args.quiet;

    // Create CLI instance
    let mut cli = Cli::new(cli_config);

    // Connect to service
    let connection_result = cli.connect().await;
    
    if let Err(ref e) = connection_result {
        if !args.quiet {
            eprintln!("{}: {}", "Error connecting to service".bright_red(), e);
            
            // Only warn and continue for commands that might work without a service connection
            match args.command {
                Commands::Auth { .. } | Commands::Completions { .. } => {
                    eprintln!("{}", "Continuing without service connection...".yellow());
                },
                _ => {
                    eprintln!("{}",
                        "Make sure the RCP service is running. You can start it with:".yellow());
                    eprintln!("  rcp-cli service start");
                    return Err(anyhow::anyhow!("Failed to connect to service"));
                }
            }
        }
    }

    // Handle commands
    let result = match &args.command {
        Commands::Service { action, auto_start, user } => {
            commands::service::handle_service_command(action.clone(), auto_start.unwrap_or(false), user.clone(), &mut cli).await
        }
        Commands::Server { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Session { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::User { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Config { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Diag { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Logs { level: _, limit: _, since: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Auth { action } => {
            commands::auth::handle_auth_command(&mut cli, action.clone()).await
        }
        Commands::Shell => {
            Ok(()) // Placeholder
        }
        Commands::Batch { file: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Completions { shell } => {
            commands::completions::handle_completions_command(&mut Command::new("rcp-cli"), *shell)
                .map_err(|e| anyhow::anyhow!("Completion error: {}", e))?;
            Ok(())
        }
    };

    // Disconnect from service
    if connection_result.is_ok() {
        if let Err(e) = cli.disconnect().await {
            if !args.quiet {
                eprintln!("{}: {}", "Warning".bright_yellow(), e);
            }
        }
    }

    // Handle command result
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if !args.quiet {
                eprintln!("{}: {}", "Error".bright_red(), e);
            }
            Err(anyhow::anyhow!("{}", e))
        }
    }
}