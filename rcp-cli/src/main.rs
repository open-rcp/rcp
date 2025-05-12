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
    Add {
        /// Username to add
        #[clap(long)]
        username: String,
        
        /// Password for the new user (will prompt if not provided)
        #[clap(long)]
        password: Option<String>,
        
        /// Role for the new user (admin, user, guest)
        #[clap(long, default_value = "user")]
        role: String,
    },
    Remove {
        /// Username to remove
        #[clap(long)]
        username: String,
    },
    UpdateRole {
        /// Username to update
        #[clap(long)]
        username: String,
        
        /// New role (admin, user, guest)
        #[clap(long)]
        role: String,
    },
    ResetPassword {
        /// Username to reset password for
        #[clap(long)]
        username: String,
        
        /// New password (will prompt if not provided)
        #[clap(long)]
        password: Option<String>,
    },
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
                auto_start.unwrap_or(false),
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
        Commands::User { action } => {
            match action {
                UserAction::List => {
                    let users = cli.list_users().await?;
                    
                    if users.is_empty() {
                        println!("No users found");
                    } else {
                        println!("{:<36} {:<20} {:<10}", "ID", "Username", "Role");
                        println!("{}", "-".repeat(70));
                        
                        for user in users {
                            println!("{:<36} {:<20} {:<10}", user.id, user.username, user.role);
                        }
                    }
                    
                    Ok(())
                }
                UserAction::Add { username, password, role } => {
                    let password = match password {
                        Some(p) => p.clone(),
                        None => {
                            // Prompt for password
                            utils::prompt("Password", None)?
                        }
                    };
                    
                    // Validate the password
                    if password.len() < 8 {
                        return Err(anyhow::anyhow!("Password must be at least 8 characters"));
                    }
                    
                    // Confirm the password
                    if password.is_none() {
                        let confirm = utils::prompt("Confirm password", None)?;
                        if confirm != password {
                            return Err(anyhow::anyhow!("Passwords do not match"));
                        }
                    }
                    
                    cli.add_user(username, &password, role).await?;
                    println!("User '{}' added successfully with role '{}'", username, role);
                    
                    Ok(())
                }
                UserAction::Remove { username } => {
                    // Ask for confirmation
                    let confirm = utils::prompt(
                        &format!("Are you sure you want to delete user '{}'? (y/N)", username),
                        Some("N"),
                    )?;
                    
                    if !confirm.eq_ignore_ascii_case("y") && !confirm.eq_ignore_ascii_case("yes") {
                        println!("Operation cancelled");
                        return Ok(());
                    }
                    
                    cli.delete_user(username).await?;
                    println!("User '{}' deleted successfully", username);
                    
                    Ok(())
                }
                UserAction::UpdateRole { username, role } => {
                    cli.update_user_role(username, role).await?;
                    println!("Updated role for user '{}' to '{}'", username, role);
                    
                    Ok(())
                }
                UserAction::ResetPassword { username, password } => {
                    let password = match password {
                        Some(p) => p.clone(),
                        None => {
                            // Prompt for password
                            utils::prompt("New password", None)?
                        }
                    };
                    
                    // Validate the password
                    if password.len() < 8 {
                        return Err(anyhow::anyhow!("Password must be at least 8 characters"));
                    }
                    
                    // Confirm the password
                    if password.is_none() {
                        let confirm = utils::prompt("Confirm new password", None)?;
                        if confirm != password {
                            return Err(anyhow::anyhow!("Passwords do not match"));
                        }
                    }
                    
                    cli.reset_user_password(username, &password).await?;
                    println!("Password for user '{}' reset successfully", username);
                    
                    Ok(())
                }
            }
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
