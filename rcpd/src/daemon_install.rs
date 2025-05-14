//! Daemon installation module
//!
//! This module contains functionality for installing the daemon as a system service.

use anyhow::{anyhow, Result};
use std::path::PathBuf;

/// Install service
pub fn install(config: &str) -> Result<()> {
    #[cfg(target_os = "linux")]
    return install_linux(config);

    #[cfg(target_os = "macos")]
    return install_macos(config);

    #[cfg(target_os = "windows")]
    return install_windows(config);

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(anyhow!("Unsupported platform for service installation"))
    }
}

/// Uninstall service
pub fn uninstall() -> Result<()> {
    #[cfg(target_os = "linux")]
    return uninstall_linux();

    #[cfg(target_os = "macos")]
    return uninstall_macos();

    #[cfg(target_os = "windows")]
    return uninstall_windows();

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(anyhow!("Unsupported platform for service uninstallation"))
    }
}

/// Install service on Linux
#[cfg(target_os = "linux")]
fn install_linux(config: &str) -> Result<()> {
    let home_dir =
        std::env::var("HOME").map_err(|_| anyhow!("Could not determine home directory"))?;
    let home_dir = PathBuf::from(home_dir);
    let config_dir = home_dir.join(".config/systemd/user");

    // Create config directory if it doesn't exist
    std::fs::create_dir_all(&config_dir)?;

    // Create service file
    let service_file = config_dir.join("rcpd.service");
    let service_content = format!(
        r#"[Unit]
Description=RCP Daemon
After=network.target

[Service]
ExecStart={exec} --config {config}
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=default.target
"#,
        exec = std::env::current_exe()?.display(),
        config = config
    );

    std::fs::write(service_file, service_content)?;

    // Enable service
    let status = std::process::Command::new("systemctl")
        .args(&["--user", "enable", "rcpd"])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to enable service"));
    }

    Ok(())
}

/// Install service on macOS
#[cfg(target_os = "macos")]
fn install_macos(config: &str) -> Result<()> {
    let home_dir =
        std::env::var("HOME").map_err(|_| anyhow!("Could not determine home directory"))?;
    let home_dir = PathBuf::from(home_dir);
    let launch_agents_dir = home_dir.join("Library/LaunchAgents");

    // Create launch agents directory if it doesn't exist
    std::fs::create_dir_all(&launch_agents_dir)?;

    // Create plist file
    let plist_file = launch_agents_dir.join("io.rcp.daemon.plist");
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>io.rcp.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exec}</string>
        <string>--config</string>
        <string>{config}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardErrorPath</key>
    <string>/tmp/rcpd.err</string>
    <key>StandardOutPath</key>
    <string>/tmp/rcpd.out</string>
</dict>
</plist>
"#,
        exec = std::env::current_exe()?.display(),
        config = config
    );

    std::fs::write(&plist_file, plist_content)?;

    // Load service
    let status = std::process::Command::new("launchctl")
        .args(&["load", "-w", &plist_file.to_string_lossy()])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to load service"));
    }

    Ok(())
}

/// Install service on Windows
#[cfg(target_os = "windows")]
fn install_windows(config: &str) -> Result<()> {
    use std::process::Command;

    let exec = std::env::current_exe()?;
    let args = format!("--config {}", config);

    // Use sc.exe to create the service
    let output = Command::new("sc")
        .args(&[
            "create",
            "RCPD",
            "binPath=",
            &format!("\"{}\" {}", exec.display(), args),
            "start=",
            "auto",
            "DisplayName=",
            "RCP Daemon",
        ])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to create service: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Uninstall service on Linux
#[cfg(target_os = "linux")]
fn uninstall_linux() -> Result<()> {
    // Disable service
    let status = std::process::Command::new("systemctl")
        .args(&["--user", "disable", "rcpd"])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to disable service"));
    }

    // Remove service file
    let home_dir =
        std::env::var("HOME").map_err(|_| anyhow!("Could not determine home directory"))?;
    let home_dir = PathBuf::from(home_dir);
    let service_file = home_dir.join(".config/systemd/user/rcpd.service");

    if service_file.exists() {
        std::fs::remove_file(service_file)?;
    }

    Ok(())
}

/// Uninstall service on macOS
#[cfg(target_os = "macos")]
fn uninstall_macos() -> Result<()> {
    let home_dir =
        std::env::var("HOME").map_err(|_| anyhow!("Could not determine home directory"))?;
    let home_dir = PathBuf::from(home_dir);
    let plist_file = home_dir.join("Library/LaunchAgents/io.rcp.daemon.plist");

    // Unload service
    let status = std::process::Command::new("launchctl")
        .args(&["unload", &plist_file.to_string_lossy()])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to unload service"));
    }

    // Remove plist file
    if plist_file.exists() {
        std::fs::remove_file(plist_file)?;
    }

    Ok(())
}

/// Uninstall service on Windows
#[cfg(target_os = "windows")]
fn uninstall_windows() -> Result<()> {
    use std::process::Command;

    // Use sc.exe to delete the service
    let output = Command::new("sc").args(&["delete", "RCPD"]).output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to delete service: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
