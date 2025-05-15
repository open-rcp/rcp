//! Command module for application management
//!
//! This module contains the command handlers for application-related operations.
//! Ported from rcp-cli component as part of CLI unification.

#[cfg(feature = "cli")]
use anyhow::Result;
#[cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "cli")]
use colored::Colorize;
#[cfg(feature = "cli")]
use std::fmt::Display;

#[cfg(feature = "cli")]
use crate::cli::{types::Cli, utils::OutputFormatter};

/// Application representation
#[cfg(feature = "cli")]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub path: String,
    pub arguments: Option<Vec<String>>,
    pub working_dir: Option<String>,
    pub enabled: bool,
}

#[cfg(feature = "cli")]
impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.enabled {
            "enabled".green()
        } else {
            "disabled".red()
        };

        write!(
            f,
            "{} - {} ({})\n  Path: {}\n  Args: {}\n  Working Dir: {}",
            self.id,
            self.name,
            status,
            self.path,
            self.arguments
                .as_ref()
                .map(|args| args.join(" "))
                .unwrap_or_else(|| "None".to_string()),
            self.working_dir
                .as_ref()
                .map(|dir| dir.to_string())
                .unwrap_or_else(|| "Default".to_string())
        )
    }
}

/// Handle the app command
#[cfg(feature = "cli")]
pub async fn handle_app_command(cli: &mut Cli, command: &AppCommand) -> Result<()> {
    match command {
        AppCommand::List { filter } => list_applications(cli, filter.as_deref()).await,
        AppCommand::Show { id } => show_application(cli, id).await,
        AppCommand::Create {
            name,
            path,
            arguments,
            working_dir,
            enabled,
        } => create_application(cli, name, path, arguments, working_dir.as_deref(), *enabled).await,
        AppCommand::Update {
            id,
            name,
            path,
            arguments,
            working_dir,
            enabled,
        } => {
            update_application(
                cli,
                id,
                name.as_deref(),
                path.as_deref(),
                arguments.as_ref(),
                working_dir.as_deref(),
                *enabled,
            )
            .await
        }
        AppCommand::Delete { id } => delete_application(cli, id).await,
        AppCommand::Enable { id } => set_application_status(cli, id, true).await,
        AppCommand::Disable { id } => set_application_status(cli, id, false).await,
    }
}

/// Application commands
#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
pub enum AppCommand {
    /// List available applications
    List {
        /// Filter applications by name
        filter: Option<String>,
    },

    /// Show application details
    Show {
        /// Application ID or name
        id: String,
    },

    /// Create a new application
    Create {
        /// Application name
        name: String,

        /// Application path (absolute path to executable)
        path: String,

        /// Application arguments
        #[clap(short, long)]
        arguments: Option<Vec<String>>,

        /// Working directory
        #[clap(short, long)]
        working_dir: Option<String>,

        /// Enable application (default: true)
        #[clap(short, long, default_value = "true")]
        enabled: bool,
    },

    /// Update an application
    Update {
        /// Application ID or name
        id: String,

        /// New application name
        #[clap(short, long)]
        name: Option<String>,

        /// New application path
        #[clap(short, long)]
        path: Option<String>,

        /// New application arguments
        #[clap(short, long)]
        arguments: Option<Vec<String>>,

        /// New working directory
        #[clap(short, long)]
        working_dir: Option<String>,

        /// Enable/disable application
        #[clap(short, long)]
        enabled: Option<bool>,
    },

    /// Delete an application
    Delete {
        /// Application ID or name
        id: String,
    },

    /// Enable an application
    Enable {
        /// Application ID or name
        id: String,
    },

    /// Disable an application
    Disable {
        /// Application ID or name
        id: String,
    },
}

// Application command implementations below

/// List available applications
#[cfg(feature = "cli")]
async fn list_applications(cli: &mut Cli, filter: Option<&str>) -> Result<()> {
    let formatter = OutputFormatter::new(cli.json, true, false);

    // TODO: Implement service client request to get applications
    let applications = vec![
        // This is just sample data - replace with actual service client call
        Application {
            id: "app1".to_string(),
            name: "Sample App 1".to_string(),
            path: "/usr/bin/sample1".to_string(),
            arguments: Some(vec!["-v".to_string()]),
            working_dir: None,
            enabled: true,
        },
        Application {
            id: "app2".to_string(),
            name: "Sample App 2".to_string(),
            path: "/usr/bin/sample2".to_string(),
            arguments: None,
            working_dir: Some("/tmp".to_string()),
            enabled: false,
        },
    ];

    let filtered = if let Some(filter_text) = filter {
        applications
            .into_iter()
            .filter(|app| app.name.contains(filter_text) || app.id.contains(filter_text))
            .collect::<Vec<_>>()
    } else {
        applications
    };

    let _ = formatter.output_list(&filtered, "Applications", "No applications found");
    Ok(())
}

/// Show application details
#[cfg(feature = "cli")]
async fn show_application(cli: &mut Cli, id: &str) -> Result<()> {
    let formatter = OutputFormatter::new(cli.json, true, false);

    // TODO: Implement service client request to get application by ID
    // This is just sample data - replace with actual service client call
    let application = Application {
        id: id.to_string(),
        name: "Sample App".to_string(),
        path: "/usr/bin/sample".to_string(),
        arguments: Some(vec!["-v".to_string()]),
        working_dir: None,
        enabled: true,
    };

    let _ = formatter.output_item(&application, "Application Details");
    Ok(())
}

/// Create a new application
#[cfg(feature = "cli")]
async fn create_application(
    cli: &mut Cli,
    name: &str,
    path: &str,
    arguments: &Option<Vec<String>>,
    working_dir: Option<&str>,
    enabled: bool,
) -> Result<()> {
    let formatter = OutputFormatter::new(cli.json, true, false);

    // TODO: Implement service client request to create application
    // This is just sample code - replace with actual service client call
    let application = Application {
        id: "new-app-id".to_string(), // This would be generated by the service
        name: name.to_string(),
        path: path.to_string(),
        arguments: arguments.clone(),
        working_dir: working_dir.map(|s| s.to_string()),
        enabled,
    };

    formatter.output_success(&format!("Application '{}' created successfully", name));
    let _ = formatter.output_item(&application, "Application Details");
    Ok(())
}

/// Update an application
#[cfg(feature = "cli")]
async fn update_application(
    cli: &mut Cli,
    id: &str,
    name: Option<&str>,
    path: Option<&str>,
    arguments: Option<&Vec<String>>,
    working_dir: Option<&str>,
    enabled: Option<bool>,
) -> Result<()> {
    let formatter = OutputFormatter::new(cli.json, true, false);

    // TODO: Implement service client request to update application
    // This is just sample code - replace with actual service client call

    // Building a change description for output
    let mut changes = Vec::new();
    if let Some(name) = name {
        changes.push(format!("name: {}", name));
    }
    if let Some(path) = path {
        changes.push(format!("path: {}", path));
    }
    if let Some(args) = arguments {
        changes.push(format!("arguments: {}", args.join(" ")));
    }
    if let Some(dir) = working_dir {
        changes.push(format!("working directory: {}", dir));
    }
    if let Some(status) = enabled {
        changes.push(format!(
            "status: {}",
            if status { "enabled" } else { "disabled" }
        ));
    }

    formatter.output_success(&format!(
        "Application '{}' updated successfully: {}",
        id,
        changes.join(", ")
    ));
    Ok(())
}

/// Delete an application
#[cfg(feature = "cli")]
async fn delete_application(cli: &mut Cli, id: &str) -> Result<()> {
    let formatter = OutputFormatter::new(cli.json, true, false);

    // TODO: Implement service client request to delete application
    // This is just sample code - replace with actual service client call

    formatter.output_success(&format!("Application '{}' deleted successfully", id));
    Ok(())
}

/// Enable or disable an application
#[cfg(feature = "cli")]
async fn set_application_status(cli: &mut Cli, id: &str, enabled: bool) -> Result<()> {
    let formatter = OutputFormatter::new(cli.json, true, false);

    // TODO: Implement service client request to enable/disable application
    // This is just sample code - replace with actual service client call

    let status = if enabled { "enabled" } else { "disabled" };
    formatter.output_success(&format!("Application '{}' {} successfully", id, status));
    Ok(())
}
