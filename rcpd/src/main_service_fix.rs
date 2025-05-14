#[cfg(feature = "cli")]
async fn handle_server_command(command: ServerCommand, _config: &config::ServiceConfig, json_output: bool) -> Result<()> {
    use crate::cli::utils::OutputFormatter;
    use crate::cli::service::ServiceClient;
    
    let formatter = OutputFormatter::new(json_output, true, false);
    let client = ServiceClient::new("localhost".to_string(), 8080);
    
    match command {
        ServerCommand::Status => {
            crate::cli::commands::server::handle_status(&client, &formatter).await?;
        },
        ServerCommand::Restart => {
            crate::cli::commands::server::handle_restart(&client, &formatter).await?;
        },
        ServerCommand::Config { action } => {
            match action {
                ServerConfigAction::Display => {
                    crate::cli::commands::server::handle_config_display(&client, &formatter).await?;
                },
                ServerConfigAction::Update { key, value } => {
                    crate::cli::commands::server::handle_config_update(&client, &key, &value, &formatter).await?;
                }
            }
        }
    }
    
    Ok(())
}

async fn handle_service_command(command: ServiceCommand, _config: &config::ServiceConfig, json_output: bool) -> Result<()> {
    use crate::cli::utils::OutputFormatter;
    
    let formatter = OutputFormatter::new(json_output, true, false);
    
    match command {
        ServiceCommand::Start => {
            println!("Starting RCP service...");
            daemon::start("service.toml")?;
        },
        ServiceCommand::Stop => {
            println!("Stopping RCP service...");
            daemon::stop()?;
        },
        ServiceCommand::Restart => {
            println!("Restarting RCP service...");
            daemon::stop()?;
            daemon::start("service.toml")?;
        },
        ServiceCommand::Status => {
            let status = daemon::status()?;
            println!("RCP Service Status: {}", status);
        },
        ServiceCommand::Install => {
            println!("Installing RCP service...");
            daemon_install::install("service.toml")?;
        },
        ServiceCommand::Uninstall => {
            println!("Uninstalling RCP service...");
            daemon_install::uninstall()?;
        }
    }
    
    Ok(())
}
