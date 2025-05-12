use crate::cli::Cli;
use crate::utils;
use crate::ServiceAction;
use anyhow::Result;
use colored::Colorize;

/// Helper function for tests to check if this module is properly loaded
#[cfg(test)]
pub fn is_module_loaded() -> bool {
    true
}

/// Handle service-related commands
pub async fn handle_service_command(
    action: ServiceAction,
    auto_start: bool,
    user: Option<String>,
    cli: &mut Cli,
) -> Result<()> {
    match action {
        ServiceAction::Install => install_service(cli, auto_start, user).await,
        ServiceAction::Uninstall => uninstall_service(cli).await,
        ServiceAction::Start => start_service(cli).await,
        ServiceAction::Stop => stop_service(cli).await,
        ServiceAction::Restart => restart_service(cli).await,
        ServiceAction::Status => print_service_status(cli).await,
    }
}

/// Install RCP service
async fn install_service(cli: &mut Cli, auto_start: bool, user: Option<String>) -> Result<()> {
    // Try to check if already installed, but proceed even if service isn't running
    let result = cli.get_service_client_mut();

    let already_installed = match result {
        Ok(client) => {
            (client.is_service_installed().await).unwrap_or(false) // Assume not installed if we can't check
        }
        Err(_) => {
            // Service not running, but that's okay for installation
            false
        }
    };

    if already_installed {
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

    // Try to send installation command to running service
    let client_result = cli.get_service_client_mut();

    let installed = if let Ok(client) = client_result {
        (client.install_service(&install_options).await).is_ok()
    } else {
        // Service isn't running, perform direct installation
        println!(
            "{}",
            "RCP service is not running. Attempting direct installation...".yellow()
        );

        // Platform-specific installation
        #[cfg(target_os = "macos")]
        {
            use std::fs::{self, File};
            use std::io::Write;
            use std::path::PathBuf;
            use std::process::Command;

            // Create LaunchAgent directory if it doesn't exist
            let launch_agents_dir = dirs::home_dir()
                .map(|h| h.join("Library/LaunchAgents"))
                .unwrap_or_else(|| PathBuf::from("/Library/LaunchAgents"));

            fs::create_dir_all(&launch_agents_dir)?;

            // Get path to the rcp-service binary
            let service_binary = std::env::current_exe()?
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?
                .join("rcp-service");

            // Create plist file content
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.rcp.service</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <{}/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/rcp-service.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/rcp-service.err</string>
</dict>
</plist>"#,
                service_binary.to_string_lossy(),
                if auto_start { "true" } else { "false" }
            );

            // Write plist file
            let plist_path = launch_agents_dir.join("com.rcp.service.plist");
            let mut file = File::create(&plist_path)?;
            file.write_all(plist_content.as_bytes())?;

            // Load the service if auto_start is true
            if auto_start {
                Command::new("launchctl")
                    .args(["load", "-w"])
                    .arg(&plist_path)
                    .status()?;
            }

            true
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs::{self, File};
            use std::io::Write;
            use std::process::Command;

            // Get path to the rcp-service binary
            let service_binary = std::env::current_exe()?
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?
                .join("rcp-service");

            // Create systemd unit file content
            let unit_content = format!(
                r#"[Unit]
Description=RCP Service
After=network.target

[Service]
Type=simple
ExecStart={}
Restart=on-failure

[Install]
WantedBy=multi-user.target
"#,
                service_binary.to_string_lossy()
            );

            // Create directory if it doesn't exist
            let systemd_dir = PathBuf::from("/etc/systemd/system");
            fs::create_dir_all(&systemd_dir)?;

            // Write unit file
            let unit_path = systemd_dir.join("rcp-service.service");
            let mut file = File::create(&unit_path)?;
            file.write_all(unit_content.as_bytes())?;

            // Enable and start service if auto_start is true
            if auto_start {
                Command::new("systemctl").args(["daemon-reload"]).status()?;

                Command::new("systemctl")
                    .args(["enable", "rcp-service"])
                    .status()?;

                Command::new("systemctl")
                    .args(["start", "rcp-service"])
                    .status()?;
            }

            true
        }

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // Get path to the rcp-service binary
            let service_binary = std::env::current_exe()?
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?
                .join("rcp-service.exe");

            // Install service using sc.exe
            let status = Command::new("sc.exe")
                .args(["create", "RCPService"])
                .arg(format!("binPath= {}", service_binary.to_string_lossy()))
                .arg("start= auto")
                .arg("DisplayName= RCP Service")
                .status()?;

            if status.success() {
                // Start the service if auto_start is true
                if auto_start {
                    Command::new("sc.exe")
                        .args(["start", "RCPService"])
                        .status()?;
                }
                true
            } else {
                false
            }
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            println!(
                "{}",
                "Direct installation not supported on this platform".red()
            );
            false
        }
    };

    if installed {
        println!("\n{}", "RCP service installed successfully!".green());

        if auto_start {
            println!("The service will start automatically on boot.");
        } else {
            println!(
                "You can start the service with: {}",
                "rcp-cli service start".cyan()
            );
        }
    } else {
        println!("\n{}", "Failed to install RCP service".red());
        println!("Please check permissions and try again.");
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

    // Try to send start command to running service
    let client_result = cli.get_service_client_mut();

    let started = if let Ok(client) = client_result {
        (client.start_service().await).is_ok()
    } else {
        // Service isn't running, perform direct start
        println!(
            "{}",
            "RCP service control socket not available. Attempting direct start...".yellow()
        );

        // Platform-specific start
        #[cfg(target_os = "macos")]
        {
            use std::path::PathBuf;
            use std::process::Command;

            let launch_agents_dir = dirs::home_dir()
                .map(|h| h.join("Library/LaunchAgents"))
                .unwrap_or_else(|| PathBuf::from("/Library/LaunchAgents"));

            let plist_path = launch_agents_dir.join("com.rcp.service.plist");

            if plist_path.exists() {
                let status = Command::new("launchctl")
                    .args(["load", "-w"])
                    .arg(&plist_path)
                    .status()?;

                status.success()
            } else {
                // Try to start the service binary directly
                let service_binary = std::env::current_exe()?
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?
                    .join("rcp-service");

                if service_binary.exists() {
                    let _child = Command::new(&service_binary).spawn()?;

                    // Give it a moment to start
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    true
                } else {
                    println!(
                        "{}",
                        "Service binary not found. Please install the service first.".red()
                    );
                    false
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;

            let systemd_unit = "/etc/systemd/system/rcp-service.service";

            if std::path::Path::new(systemd_unit).exists() {
                let status = Command::new("systemctl")
                    .args(["start", "rcp-service"])
                    .status()?;

                status.success()
            } else {
                // Try to start the service binary directly
                let service_binary = std::env::current_exe()?
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?
                    .join("rcp-service");

                if service_binary.exists() {
                    let _child = Command::new(&service_binary).spawn()?;

                    // Give it a moment to start
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    true
                } else {
                    println!(
                        "{}",
                        "Service binary not found. Please install the service first.".red()
                    );
                    false
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // Try to start the Windows service
            let status = Command::new("sc.exe")
                .args(["start", "RCPService"])
                .status()?;

            if status.success() {
                true
            } else {
                // Try to start the service binary directly
                let service_binary = std::env::current_exe()?
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?
                    .join("rcp-service.exe");

                if service_binary.exists() {
                    let _child = Command::new(&service_binary).spawn()?;

                    // Give it a moment to start
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    true
                } else {
                    println!(
                        "{}",
                        "Service binary not found. Please install the service first.".red()
                    );
                    false
                }
            }
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            println!(
                "{}",
                "Direct service start not supported on this platform".red()
            );
            false
        }
    };

    if started {
        println!("\n{}", "RCP service started successfully!".green());
    } else {
        println!("\n{}", "Failed to start RCP service".red());
        println!("Please check permissions and try again.");
    }

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
    println!(
        "Status:             {}",
        utils::format_status(&status.service_status)
    );
    println!(
        "Uptime:             {}",
        utils::format_duration(status.uptime)
    );
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

            utils::print_table_row(&[&server.name, &status_str, &port_str, &conn_str], &widths);
        }
    }

    Ok(())
}
