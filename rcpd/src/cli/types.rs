//! CLI command types
//!
//! This module defines types for CLI commands.

#[cfg(feature = "cli")]
use clap::Parser;

/// Server commands
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum ServerCommand {
    /// Display server status
    Status,

    /// Restart the server
    Restart,

    /// Server configuration commands
    Config {
        #[clap(subcommand)]
        action: ServerConfigAction,
    },
}

/// Server configuration actions
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum ServerConfigAction {
    /// Display server configuration
    Display,

    /// Update server configuration
    Update {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },
}

/// Application commands
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum AppCommand {
    /// List available applications
    List,

    /// Display application information
    Info {
        /// Application ID
        app_id: String,
    },

    /// Launch an application
    Launch {
        /// Application ID
        app_id: String,

        /// User ID (optional)
        #[clap(long)]
        user_id: Option<String>,

        /// Additional arguments to pass to the application
        #[clap(multiple = true)]
        args: Vec<String>,
    },

    /// List running application instances
    Instances,

    /// Stop a running application instance
    Stop {
        /// Instance ID
        instance_id: String,
    },
}

/// Session commands
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum SessionCommand {
    /// List active sessions
    List,

    /// Display session information
    Info {
        /// Session ID
        session_id: String,
    },

    /// Close a session
    Close {
        /// Session ID
        session_id: String,
    },
}

/// Configuration commands
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum ConfigCommand {
    /// Display configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },

    /// Remove a configuration value
    Remove {
        /// Configuration key
        key: String,
    },
}

/// User commands
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum UserCommand {
    /// List users
    List,

    /// Display user information
    Info {
        /// User ID
        user: String,
    },

    /// Create a new user
    Create {
        /// Username
        username: String,

        /// Password
        password: String,

        /// Administrator privileges
        #[clap(long)]
        admin: bool,
    },

    /// Delete a user
    Delete {
        /// User ID
        user: String,
    },

    /// Set user password
    SetPassword {
        /// User ID
        user_id: String,

        /// New password
        password: String,
    },
}

/// Diagnostic commands
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum DiagCommand {
    /// Display system information
    System,

    /// Check network connectivity
    Network,

    /// Display logs
    Logs {
        /// Number of lines to display
        #[clap(default_value = "10")]
        lines: usize,

        /// Follow log output
        #[clap(short, long)]
        follow: bool,
    },
}
