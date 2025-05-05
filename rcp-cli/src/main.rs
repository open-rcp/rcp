use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

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
    /// Service management commands
    Service {
        #[clap(subcommand)]
        action: ServiceAction,
    },

    /// Server management commands
    Server {
        #[clap(subcommand)]
        action: ServerAction,
    },

    /// Session management commands
    Session {
        #[clap(subcommand)]
        action: SessionAction,
    },

    /// User management commands
    User {
        #[clap(subcommand)]
        action: UserAction,
    },

    /// Configuration management commands
    Config {
        #[clap(subcommand)]
        action: ConfigAction,
    },

    /// Diagnostic commands
    Diag {
        #[clap(subcommand)]
        action: DiagAction,
    },

    /// View system logs
    Logs {
        /// Log level filter
        #[clap(short, long)]
        level: Option<String>,

        /// Maximum number of logs to show
        #[clap(short, long, default_value = "100")]
        limit: usize,

        /// Show logs since this time (e.g. "10m", "1h", "1d")
        #[clap(short, long)]
        since: Option<String>,
    },

    /// Login to RCP API
    Login {
        /// Username for authentication
        #[clap(short, long)]
        username: Option<String>,
    },

    /// Logout from RCP API
    Logout,

    /// Start interactive shell mode
    Shell,

    /// Execute commands from a file
    Batch {
        /// Path to file with commands
        file: PathBuf,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
enum ServiceAction {
    /// Install the RCP service
    Install {
        /// Start automatically at boot
        #[clap(long)]
        auto_start: bool,

        /// Run service as specific user
        #[clap(long)]
        user: Option<String>,
    },

    /// Uninstall the RCP service
    Uninstall,

    /// Start the RCP service
    Start,

    /// Stop the RCP service
    Stop,

    /// Restart the RCP service
    Restart,

    /// Get RCP service status
    Status,
}

#[derive(Subcommand)]
enum ServerAction {
    /// List configured servers
    List,

    /// Get server status
    Status {
        /// Server name
        #[clap(default_value = "default")]
        name: String,
    },

    /// Start a server
    Start {
        /// Server name
        #[clap(default_value = "default")]
        name: String,
    },

    /// Stop a server
    Stop {
        /// Server name
        #[clap(default_value = "default")]
        name: String,
    },

    /// Restart a server
    Restart {
        /// Server name
        #[clap(default_value = "default")]
        name: String,
    },

    /// Create a new server configuration
    Create {
        /// Server name
        name: String,

        /// Server port
        #[clap(short, long, default_value = "8716")]
        port: u16,

        /// Maximum connections
        #[clap(long, default_value = "100")]
        max_conn: usize,

        /// Enable TLS
        #[clap(long)]
        tls: bool,
    },
}

#[derive(Subcommand)]
enum SessionAction {
    /// List active sessions
    List {
        /// Filter by server
        #[clap(short, long)]
        server: Option<String>,
    },

    /// Get session details
    Info {
        /// Session ID
        session_id: String,
    },

    /// Terminate a session
    Terminate {
        /// Session ID
        session_id: String,

        /// Reason for termination
        #[clap(short, long)]
        reason: Option<String>,
    },

    /// Send message to a session
    Message {
        /// Session ID
        session_id: String,

        /// Message to send
        message: String,
    },
}

#[derive(Subcommand)]
enum UserAction {
    /// List users
    List,

    /// Add a new user
    Add {
        /// Username
        username: String,

        /// Create as admin
        #[clap(long)]
        admin: bool,

        /// User password
        #[clap(short, long)]
        password: Option<String>,
    },

    /// Change user password
    Passwd {
        /// Username
        username: String,
    },

    /// Remove a user
    Remove {
        /// Username
        username: String,
    },

    /// Manage user roles
    Roles {
        /// Username
        username: String,

        /// Add role(s)
        #[clap(long)]
        add: Option<Vec<String>>,

        /// Remove role(s)
        #[clap(long)]
        remove: Option<Vec<String>>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show {
        /// Configuration section
        #[clap(short, long)]
        section: Option<String>,
    },

    /// Set configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// Reset configuration to defaults
    Reset {
        /// Configuration section
        #[clap(short, long)]
        section: Option<String>,
    },

    /// Validate configuration
    Validate,
}

#[derive(Subcommand)]
enum DiagAction {
    /// Run diagnostics
    Run {
        /// Tests to run
        #[clap(short, long)]
        tests: Option<Vec<String>>,
    },

    /// Check connectivity
    Connectivity {
        /// Server to check
        #[clap(short, long)]
        server: Option<String>,
    },
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
                Commands::Login { .. } | Commands::Completions { .. } => {
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
    let result = match args.command {
        Commands::Service { action } => {
            commands::service::handle_service_command(action, &mut cli).await
        }
        Commands::Server { action } => {
            commands::server::handle_server_command(action, &mut cli).await
        }
        Commands::Session { action } => {
            commands::session::handle_session_command(action, &mut cli).await
        }
        Commands::User { action } => {
            commands::user::handle_user_command(action, &mut cli).await
        }
        Commands::Config { action } => {
            commands::config::handle_config_command(action, &mut cli).await
        }
        Commands::Diag { action } => {
            commands::diag::handle_diag_command(action, &mut cli).await
        }
        Commands::Logs { level, limit, since } => {
            commands::logs::handle_logs_command(level, limit, since, &mut cli).await
        }
        Commands::Login { username } => {
            commands::auth::handle_login_command(username, &mut cli).await
        }
        Commands::Logout => {
            commands::auth::handle_logout_command(&mut cli).await
        }
        Commands::Shell => {
            commands::shell::handle_shell_command(&mut cli).await
        }
        Commands::Batch { file } => {
            commands::batch::handle_batch_command(file, &mut cli).await
        }
        Commands::Completions { shell } => {
            commands::completions::handle_completions_command(shell)
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