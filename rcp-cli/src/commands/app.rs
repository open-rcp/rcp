use crate::cli::Cli;
use crate::utils;
use anyhow::Result;
use colored::Colorize;

/// Application action enum (only unit variants for clap's ValueEnum)
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

/// Options for application commands
pub struct AppCommandOptions<'a> {
    pub id: Option<&'a str>,
    pub name: Option<&'a str>,
    pub path: Option<&'a str>,
    pub args: Option<&'a str>,
    pub description: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub instance_id: Option<&'a str>,
}

/// Handle application commands - updated to work with options struct
pub async fn handle_app_command(
    cli: &mut Cli,
    action: AppAction,
    options: AppCommandOptions<'_>,
) -> Result<()> {
    match action {
        AppAction::List => {
            let apps = cli.list_apps().await?;

            if apps.is_empty() {
                println!("No applications found");
            } else {
                println!(
                    "{:<36} {:<25} {:<12} {:<30}",
                    "ID", "Name", "Status", "Path"
                );
                println!("{}", "-".repeat(100));

                for app in apps {
                    let status = if app.enabled {
                        "Enabled".green()
                    } else {
                        "Disabled".red()
                    };
                    println!(
                        "{:<36} {:<25} {:<12} {:<30}",
                        app.id, app.name, status, app.path
                    );
                }
            }
        }
        AppAction::Get => {
            // Check if ID is provided
            let id = options.id.ok_or(anyhow::anyhow!("Application ID is required for get action"))?;
            let app = cli.get_app(id).await?;

            utils::print_section("Application Details");
            println!("ID:          {}", app.id);
            println!("Name:        {}", app.name);
            println!("Path:        {}", app.path);
            println!(
                "Status:      {}",
                if app.enabled {
                    "Enabled".green()
                } else {
                    "Disabled".red()
                }
            );
            if let Some(args) = app.args {
                println!("Arguments:   {}", args);
            }
            if let Some(desc) = app.description {
                println!("Description: {}", desc);
            }
            if let Some(created) = app.created_at {
                println!("Created:     {}", created);
            }
            if let Some(updated) = app.updated_at {
                println!("Updated:     {}", updated);
            }
        }
        AppAction::Create => {
            // Check if required arguments are available
            let name = options.name.ok_or(anyhow::anyhow!(
                "Application name is required for create action"
            ))?;
            let path = options.path.ok_or(anyhow::anyhow!(
                "Application path is required for create action"
            ))?;

            // Validate path
            if !std::path::Path::new(path).exists() {
                println!("{}", "Warning: Application path does not exist".yellow());
                if !utils::confirm("Do you want to continue?", false) {
                    return Ok(());
                }
            }

            let app = cli.create_app(name, path, options.args, options.description).await?;

            println!(
                "Application '{}' created successfully with ID: {}",
                app.name, app.id
            );
        }
        AppAction::Update => {
            // Check if ID is provided
            let id = options.id.ok_or(anyhow::anyhow!(
                "Application ID is required for update action"
            ))?;

            // Validate path if provided
            if let Some(path_val) = options.path {
                if !std::path::Path::new(path_val).exists() {
                    println!("{}", "Warning: Application path does not exist".yellow());
                    if !utils::confirm("Do you want to continue?", false) {
                        return Ok(());
                    }
                }
            }

            let app = cli
                .update_app(id, options.name, options.path, options.args, options.description, None)
                .await?;

            println!("Application '{}' updated successfully", app.name);
        }
        AppAction::Delete => {
            // Check if ID is provided
            let id = options.id.ok_or(anyhow::anyhow!(
                "Application ID is required for delete action"
            ))?;

            // Ask for confirmation
            if !utils::confirm(
                &format!(
                    "Are you sure you want to delete application with ID '{}'? (y/N)",
                    id
                ),
                false,
            ) {
                println!("Operation cancelled");
                return Ok(());
            }

            cli.delete_app(id).await?;
            println!("Application deleted successfully");
        }
        AppAction::Enable => {
            // Check if ID is provided
            let id = options.id.ok_or(anyhow::anyhow!(
                "Application ID is required for enable action"
            ))?;
            cli.enable_app(id).await?;
            println!("Application enabled successfully");
        }
        AppAction::Disable => {
            // Check if ID is provided
            let id = options.id.ok_or(anyhow::anyhow!(
                "Application ID is required for disable action"
            ))?;
            cli.disable_app(id).await?;
            println!("Application disabled successfully");
        }
        AppAction::Launch => {
            // Check if ID is provided
            let id = options.id.ok_or(anyhow::anyhow!(
                "Application ID is required for launch action"
            ))?;
            let result = cli.launch_app(id, options.user_id).await?;
            println!(
                "Application launched successfully: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
        AppAction::ListInstances => {
            let instances = cli.list_app_instances().await?;

            if instances.is_empty() {
                println!("No running application instances found");
            } else {
                println!(
                    "{:<36} {:<25} {:<15} {:<15}",
                    "Instance ID", "Application", "Status", "Started"
                );
                println!("{}", "-".repeat(95));

                for instance in instances {
                    println!(
                        "{:<36} {:<25} {:<15} {:<15}",
                        instance.instance_id,
                        instance.app_name,
                        utils::format_status(&instance.status),
                        instance.start_time
                    );
                }
            }
        }
        AppAction::Terminate => {
            // Check if instance ID is provided
            let instance_id = options.instance_id.ok_or(anyhow::anyhow!(
                "Instance ID is required for terminate action"
            ))?;

            // Confirm termination
            if !utils::confirm(
                &format!("Terminate application instance '{}'? (y/N)", instance_id),
                false,
            ) {
                println!("Operation cancelled");
                return Ok(());
            }

            cli.terminate_app_instance(instance_id).await?;
            println!("Application instance terminated successfully");
        }
    }

    Ok(())
}
