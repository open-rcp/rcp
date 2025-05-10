use anyhow::Result;
use colored::Colorize;
use crate::cli::Cli;
use crate::utils;
use crate::ServiceAction;

/// Handle service-related commands
pub async fn handle_service_command(action: ServiceAction, auto_start: bool, user: Option<String>, cli: &mut Cli) -> Result<()> {
    match action {
        ServiceAction::Install => {
            install_service(cli, auto_start, user).await
        }
        ServiceAction::Uninstall => {
            uninstall_service(cli).await
        }
        ServiceAction::Start => {
            start_service(cli).await
        }
        ServiceAction::Stop => {
            stop_service(cli).await
        }
        ServiceAction::Restart => {
            restart_service(cli).await
        }
        ServiceAction::Status => {
            print_service_status(cli).await
        }
    }
}

/// Install RCP service
async fn install_service(cli: &mut Cli, auto_start: bool, user: Option<String>) -> Result<()> {
    // Check if already installed
    let result = cli.get_service_client_mut()?.is_service_installed().await;
    
    if result.is_ok() && result.unwrap() {
        println!("{}", "RCP service is already installed".yellow());
        
        if !utils::confirm("Do you want to reinstall?", false) {
            return Ok(());
        }
    }
    
    println!("Installing RCP service...");
    
    // Create installation options
    let install_options = serde_json::json!({
        "auto_start": auto_start,
        "user": user
    });
    
    // Send installation command
    cli.get_service_client_mut()?.install_service(&install_options).await?;
    
    println!("\n{}", "RCP service installed successfully!".green());
    
    if auto_start {
        println!("The service will start automatically on boot.");
    } else {
        println!("You can start the service with: {}", "rcp-cli service start".cyan());
    }
    
    Ok(())
}

/// Uninstall RCP service
async fn uninstall_service(cli: &mut Cli) -> Result<()> {
    // Check if installed
    let result = cli.get_service_client_mut()?.is_service_installed().await;
    
    if result.is_err() || !result.unwrap() {
        println!("{}", "RCP service is not installed".yellow());
        return Ok(());
    }
    
    // Confirm uninstallation
    if !utils::confirm("This will remove the RCP service. Are you sure?", false) {
        println!("Uninstallation cancelled.");
        return Ok(());
    }
    
    println!("Uninstalling RCP service...");
    
    // Send uninstallation command
    cli.get_service_client_mut()?.uninstall_service().await?;
    
    println!("\n{}", "RCP service uninstalled successfully!".green());
    
    Ok(())
}

/// Start RCP service
async fn start_service(cli: &mut Cli) -> Result<()> {
    println!("Starting RCP service...");
    
    // Send start command
    cli.get_service_client_mut()?.start_service().await?;
    
    println!("\n{}", "RCP service started successfully!".green());
    
    Ok(())
}

/// Stop RCP service
async fn stop_service(cli: &mut Cli) -> Result<()> {
    println!("Stopping RCP service...");
    
    // Send stop command
    cli.get_service_client_mut()?.stop_service().await?;
    
    println!("\n{}", "RCP service stopped successfully!".green());
    
    Ok(())
}

/// Restart RCP service
async fn restart_service(cli: &mut Cli) -> Result<()> {
    println!("Restarting RCP service...");
    
    // Send restart command
    cli.get_service_client_mut()?.restart_service().await?;
    
    println!("\n{}", "RCP service restarted successfully!".green());
    
    Ok(())
}

/// Print service status
async fn print_service_status(cli: &mut Cli) -> Result<()> {
    // Get service status
    let status = cli.get_status().await?;
    
    // Print status information
    utils::print_section("RCP Service Status");
    println!("Status:             {}", utils::format_status(&status.service_status));
    println!("Uptime:             {}", utils::format_duration(status.uptime));
    println!("Active Servers:     {}", status.active_servers.len());
    println!("Active Connections: {}", status.active_connections);
    
    // Print server status if there are any
    if !status.active_servers.is_empty() {
        utils::print_section("Servers");
        
        // Define column widths
        let widths = [20, 10, 8, 15];
        
        // Print header
        utils::print_table_row(&["NAME", "STATUS", "PORT", "CONNECTIONS"], &widths);
        println!("{}", "-".repeat(60));
        
        // Print each server
        for server in status.active_servers {
            let status_str = utils::format_status(&server.status).to_string();
            let port_str = server.port.to_string();
            let conn_str = server.active_connections.to_string();
            
            utils::print_table_row(&[
                &server.name, 
                &status_str, 
                &port_str, 
                &conn_str
            ], &widths);
        }
    }
    
    Ok(())
}