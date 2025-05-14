//! Service commands module
//!
//! This module provides CLI commands for RCPD service management.

#[cfg(feature = "cli")]
use crate::cli::error::CliError;
#[cfg(feature = "cli")]
use crate::cli::service::ServiceClient;
#[cfg(feature = "cli")]
use crate::cli::utils::OutputFormatter;
#[cfg(feature = "cli")]
use anyhow::Result;

/// Handle service status command
#[cfg(feature = "cli")]
pub async fn handle_status(
    client: &ServiceClient,
    formatter: &OutputFormatter
) -> Result<(), CliError> {
    match client.get_status().await {
        Ok(status) => {
            if formatter.json_output {
                formatter.json(&status)?;
                return Ok(());
            }
            
            formatter.info("RCPD Service Status:");
            formatter.info(&format!("Status: {}", if status.running { "Running" } else { "Stopped" }));
            
            if let Some(pid) = status.pid {
                formatter.info(&format!("Process ID: {}", pid));
            }
            
            if let Some(uptime) = status.uptime {
                formatter.info(&format!("Uptime: {}", uptime));
            }
            
            formatter.info(&format!("Version: {}", status.version));
            Ok(())
        },
        Err(e) => {
            formatter.error("Failed to connect to RCPD service");
            formatter.error(&format!("Error: {}", e));
            
            // Check for common connection errors and provide helpful messages
            if let CliError::ConnectionError(_) = e {
                formatter.info("The service may be stopped or not installed.");
                formatter.info("You can install and start it with:");
                formatter.info("  rcpd service install");
                formatter.info("  rcpd service start");
            }
            
            Err(e)
        }
    }
}

/// Handle service start command
#[cfg(feature = "cli")]
pub async fn handle_start(
    formatter: &OutputFormatter
) -> Result<(), CliError> {
    // This operation would typically use platform-specific service management
    // For now, we'll implement a placeholder
    use std::process::Command;
    
    formatter.info("Starting RCPD service...");
    
    #[cfg(target_os = "windows")]
    let result = Command::new("sc")
        .args(&["start", "rcpd"])
        .status();
        
    #[cfg(target_os = "macos")]
    let result = Command::new("launchctl")
        .args(&["load", "-w", "/Library/LaunchDaemons/com.rcp.daemon.plist"])
        .status();
        
    #[cfg(target_os = "linux")]
    let result = Command::new("systemctl")
        .args(&["start", "rcpd"])
        .status();
        
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let result = Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "Unsupported platform"
    ));
    
    match result {
        Ok(status) if status.success() => {
            formatter.success("RCPD service started successfully");
            Ok(())
        },
        Ok(_) => {
            formatter.error("Failed to start RCPD service");
            Err(CliError::CommandError("Service start command failed".to_string()))
        },
        Err(e) => {
            formatter.error(&format!("Error starting service: {}", e));
            Err(CliError::IoError(e))
        }
    }
}

/// Handle service stop command
#[cfg(feature = "cli")]
pub async fn handle_stop(
    formatter: &OutputFormatter
) -> Result<(), CliError> {
    // This operation would typically use platform-specific service management
    // For now, we'll implement a placeholder
    use std::process::Command;
    
    formatter.info("Stopping RCPD service...");
    
    #[cfg(target_os = "windows")]
    let result = Command::new("sc")
        .args(&["stop", "rcpd"])
        .status();
        
    #[cfg(target_os = "macos")]
    let result = Command::new("launchctl")
        .args(&["unload", "/Library/LaunchDaemons/com.rcp.daemon.plist"])
        .status();
        
    #[cfg(target_os = "linux")]
    let result = Command::new("systemctl")
        .args(&["stop", "rcpd"])
        .status();
        
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let result = Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "Unsupported platform"
    ));
    
    match result {
        Ok(status) if status.success() => {
            formatter.success("RCPD service stopped successfully");
            Ok(())
        },
        Ok(_) => {
            formatter.error("Failed to stop RCPD service");
            Err(CliError::CommandError("Service stop command failed".to_string()))
        },
        Err(e) => {
            formatter.error(&format!("Error stopping service: {}", e));
            Err(CliError::IoError(e))
        }
    }
}

/// Handle service restart command
#[cfg(feature = "cli")]
pub async fn handle_restart(
    formatter: &OutputFormatter
) -> Result<(), CliError> {
    // This operation would typically use platform-specific service management
    // For now, we'll implement a placeholder
    use std::process::Command;
    
    formatter.info("Restarting RCPD service...");
    
    #[cfg(target_os = "windows")]
    let result = Command::new("sc")
        .args(&["restart", "rcpd"])
        .status();
        
    #[cfg(target_os = "macos")]
    {
        let stop = Command::new("launchctl")
            .args(&["unload", "/Library/LaunchDaemons/com.rcp.daemon.plist"])
            .status();
            
        if let Err(e) = stop {
            formatter.error(&format!("Error stopping service: {}", e));
            return Err(CliError::IoError(e));
        }
        
        let result = Command::new("launchctl")
            .args(&["load", "-w", "/Library/LaunchDaemons/com.rcp.daemon.plist"])
            .status();
    }
        
    #[cfg(target_os = "linux")]
    let result = Command::new("systemctl")
        .args(&["restart", "rcpd"])
        .status();
        
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let result = Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "Unsupported platform"
    ));
    
    match result {
        Ok(status) if status.success() => {
            formatter.success("RCPD service restarted successfully");
            Ok(())
        },
        Ok(_) => {
            formatter.error("Failed to restart RCPD service");
            Err(CliError::CommandError("Service restart command failed".to_string()))
        },
        Err(e) => {
            formatter.error(&format!("Error restarting service: {}", e));
            Err(CliError::IoError(e))
        }
    }
}

/// Handle service install command
#[cfg(feature = "cli")]
pub async fn handle_install(
    formatter: &OutputFormatter
) -> Result<(), CliError> {
    // This operation would typically use platform-specific service installation
    // For now, we'll implement a placeholder
    formatter.info("Installing RCPD service...");
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        // For Windows, we'd typically register as a service
        let result = Command::new("sc")
            .args(&["create", "rcpd", "binPath=", "\"C:\\Program Files\\RCP\\rcpd.exe daemon\"", "start=", "auto"])
            .status();
            
        match result {
            Ok(status) if status.success() => {
                formatter.success("RCPD service installed successfully");
                formatter.info("You can start the service with: rcpd service start");
                Ok(())
            },
            Ok(_) => {
                formatter.error("Failed to install RCPD service");
                Err(CliError::CommandError("Service installation failed".to_string()))
            },
            Err(e) => {
                formatter.error(&format!("Error installing service: {}", e));
                Err(CliError::IoError(e))
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::fs;
        use std::io::Write;
        use std::process::Command;
        
        // For macOS, create and load a launchd plist
        let plist_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.rcp.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/rcpd</string>
        <string>daemon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardErrorPath</key>
    <string>/var/log/rcpd.log</string>
    <key>StandardOutPath</key>
    <string>/var/log/rcpd.log</string>
</dict>
</plist>"#;

        // Create the plist file
        let plist_path = "/Library/LaunchDaemons/com.rcp.daemon.plist";
        let mut file = match fs::File::create(plist_path) {
            Ok(file) => file,
            Err(e) => {
                formatter.error(&format!("Failed to create plist file: {}", e));
                return Err(CliError::IoError(e));
            }
        };
        
        if let Err(e) = file.write_all(plist_content.as_bytes()) {
            formatter.error(&format!("Failed to write plist file: {}", e));
            return Err(CliError::IoError(e));
        }
        
        // Set correct permissions
        let chmod_result = Command::new("chmod")
            .args(&["644", plist_path])
            .status();
            
        if let Err(e) = chmod_result {
            formatter.error(&format!("Failed to set permissions on plist: {}", e));
            return Err(CliError::IoError(e));
        }
        
        formatter.success("RCPD service installed successfully");
        formatter.info("You can start the service with: rcpd service start");
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        use std::io::Write;
        use std::process::Command;
        
        // For Linux, create and enable a systemd service
        let service_content = r#"[Unit]
Description=RCP Daemon Service
After=network.target

[Service]
ExecStart=/usr/local/bin/rcpd daemon
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
"#;

        // Create the service file
        let service_path = "/etc/systemd/system/rcpd.service";
        let mut file = match fs::File::create(service_path) {
            Ok(file) => file,
            Err(e) => {
                formatter.error(&format!("Failed to create service file: {}", e));
                return Err(CliError::IoError(e));
            }
        };
        
        if let Err(e) = file.write_all(service_content.as_bytes()) {
            formatter.error(&format!("Failed to write service file: {}", e));
            return Err(CliError::IoError(e));
        }
        
        // Reload systemd
        let reload_result = Command::new("systemctl")
            .args(&["daemon-reload"])
            .status();
            
        if let Err(e) = reload_result {
            formatter.error(&format!("Failed to reload systemd: {}", e));
            return Err(CliError::IoError(e));
        }
        
        // Enable the service
        let enable_result = Command::new("systemctl")
            .args(&["enable", "rcpd"])
            .status();
            
        if let Err(e) = enable_result {
            formatter.error(&format!("Failed to enable service: {}", e));
            return Err(CliError::IoError(e));
        }
        
        formatter.success("RCPD service installed successfully");
        formatter.info("You can start the service with: rcpd service start");
        Ok(())
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        formatter.error("Service installation not supported on this platform");
        Err(CliError::CommandError("Unsupported platform".to_string()))
    }
}

/// Handle service uninstall command
#[cfg(feature = "cli")]
pub async fn handle_uninstall(
    formatter: &OutputFormatter
) -> Result<(), CliError> {
    // This operation would typically use platform-specific service uninstallation
    // For now, we'll implement a placeholder
    formatter.info("Uninstalling RCPD service...");
    
    // First, stop the service if it's running
    let _ = handle_stop(formatter).await;
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let result = Command::new("sc")
            .args(&["delete", "rcpd"])
            .status();
            
        match result {
            Ok(status) if status.success() => {
                formatter.success("RCPD service uninstalled successfully");
                Ok(())
            },
            Ok(_) => {
                formatter.error("Failed to uninstall RCPD service");
                Err(CliError::CommandError("Service uninstallation failed".to_string()))
            },
            Err(e) => {
                formatter.error(&format!("Error uninstalling service: {}", e));
                Err(CliError::IoError(e))
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::fs;
        use std::process::Command;
        
        // Unload from launchd
        let _ = Command::new("launchctl")
            .args(&["unload", "/Library/LaunchDaemons/com.rcp.daemon.plist"])
            .status();
            
        // Remove the plist file
        let plist_path = "/Library/LaunchDaemons/com.rcp.daemon.plist";
        if let Err(e) = fs::remove_file(plist_path) {
            formatter.error(&format!("Failed to remove plist file: {}", e));
            return Err(CliError::IoError(e));
        }
        
        formatter.success("RCPD service uninstalled successfully");
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        use std::process::Command;
        
        // Disable and stop the service
        let _ = Command::new("systemctl")
            .args(&["disable", "rcpd"])
            .status();
            
        // Remove the service file
        let service_path = "/etc/systemd/system/rcpd.service";
        if let Err(e) = fs::remove_file(service_path) {
            formatter.error(&format!("Failed to remove service file: {}", e));
            return Err(CliError::IoError(e));
        }
        
        // Reload systemd
        let reload_result = Command::new("systemctl")
            .args(&["daemon-reload"])
            .status();
            
        if let Err(e) = reload_result {
            formatter.error(&format!("Failed to reload systemd: {}", e));
            return Err(CliError::IoError(e));
        }
        
        formatter.success("RCPD service uninstalled successfully");
        Ok(())
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        formatter.error("Service uninstallation not supported on this platform");
        Err(CliError::CommandError("Unsupported platform".to_string()))
    }
}
