use anyhow::Result;
use clap::{Command, Parser, Subcommand};
use clap_complete::Shell;
use colored::Colorize;
use std::path::PathBuf;

// Re-import modules locally for main.rs
mod cli;
mod commands;
mod config;
mod error;
mod service;
mod utils;

// We're using these directly now
use cli::{Cli, CliConfig};

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
        #[arg(value_enum)]
        action: UserAction,

        /// Username
        #[arg(long, required_if_eq("action", "add"))]
        #[arg(long, required_if_eq("action", "remove"))]
        #[arg(long, required_if_eq("action", "update_role"))]
        #[arg(long, required_if_eq("action", "reset_password"))]
        username: Option<String>,

        /// Password for user
        #[arg(long)]
        password: Option<String>,

        /// Role for user
        #[arg(long, required_if_eq("action", "update_role"))]
        role: Option<String>,
    },
    /// Manage applications
    App {
        /// Application action to perform
        #[arg(value_enum)]
        action: AppAction,

        /// Application ID (for get, update, delete, enable, disable, launch)
        #[arg(long, required_if_eq("action", "get"))]
        #[arg(long, required_if_eq("action", "update"))]
        #[arg(long, required_if_eq("action", "delete"))]
        #[arg(long, required_if_eq("action", "enable"))]
        #[arg(long, required_if_eq("action", "disable"))]
        #[arg(long, required_if_eq("action", "launch"))]
        id: Option<String>,

        /// Application name (for create, update)
        #[arg(long, required_if_eq("action", "create"))]
        name: Option<String>,

        /// Application path (for create, update)
        #[arg(long, required_if_eq("action", "create"))]
        path: Option<String>,

        /// Command-line arguments (for create, update)
        #[arg(long)]
        args: Option<String>,

        /// Application description (for create, update)
        #[arg(long)]
        description: Option<String>,

        /// User ID to run the application as (for launch)
        #[arg(long)]
        user_id: Option<String>,

        /// Instance ID (for terminate)
        #[arg(long, required_if_eq("action", "terminate"))]
        instance_id: Option<String>,
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
        #[arg(short = 'e', long, default_value = "info")]
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
    UpdateRole,
    ResetPassword,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AppAction {
    List,
    Get,
    Create,
    Update,
    Delete,
    Enable,
    Disable,
    Launch,
    ListInstances,
    Terminate,
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
                }
                _ => {
                    eprintln!(
                        "{}",
                        "Make sure the RCP service is running. You can start it with:".yellow()
                    );
                    eprintln!("  rcp-cli service start");
                    return Err(anyhow::anyhow!("Failed to connect to service"));
                }
            }
        }
    }

    // Handle commands
    let result = match &args.command {
        Commands::Service {
            action,
            auto_start,
            user,
        } => {
            commands::service::handle_service_command(
                action.clone(),
                *auto_start.as_ref().unwrap_or(&false),
                user.clone(),
                &mut cli,
            )
            .await
        }
        Commands::Server { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Session { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::User {
            action,
            username,
            password,
            role,
        } => {
            // Convert the UserAction enum from main to the one defined in the user module
            let user_action = match action {
                UserAction::List => commands::user::UserAction::List,
                UserAction::Add => commands::user::UserAction::Add,
                UserAction::Remove => commands::user::UserAction::Remove,
                UserAction::UpdateRole => commands::user::UserAction::UpdateRole,
                UserAction::ResetPassword => commands::user::UserAction::ResetPassword,
            };

            commands::user::handle_user_command(
                &mut cli,
                user_action,
                username.as_deref(),
                password.as_deref(),
                role.as_deref(),
            )
            .await
        }
        Commands::App {
            action,
            id,
            name,
            path,
            args: app_args,
            description,
            user_id,
            instance_id,
        } => {
            // Convert the AppAction enum from main to the one defined in the app module
            let app_action = match action {
                AppAction::List => commands::app::AppAction::List,
                AppAction::Get => commands::app::AppAction::Get,
                AppAction::Create => commands::app::AppAction::Create,
                AppAction::Update => commands::app::AppAction::Update,
                AppAction::Delete => commands::app::AppAction::Delete,
                AppAction::Enable => commands::app::AppAction::Enable,
                AppAction::Disable => commands::app::AppAction::Disable,
                AppAction::Launch => commands::app::AppAction::Launch,
                AppAction::ListInstances => commands::app::AppAction::ListInstances,
                AppAction::Terminate => commands::app::AppAction::Terminate,
            };

            // Create options struct
            let options = commands::app::AppCommandOptions {
                id: id.as_deref(),
                name: name.as_deref(),
                path: path.as_deref(),
                args: app_args.as_deref(),
                description: description.as_deref(),
                user_id: user_id.as_deref(),
                instance_id: instance_id.as_deref(),
            };

            commands::app::handle_app_command(&mut cli, app_action, options).await
        }
        Commands::Config { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Diag { action: _ } => {
            Ok(()) // Placeholder
        }
        Commands::Logs {
            level: _,
            limit: _,
            since: _,
        } => {
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
