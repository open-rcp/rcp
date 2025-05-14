// Implementation of daemon::install and daemon::uninstall functions
use anyhow::Result;

/// Install the service to run as a daemon
pub fn install(config_file: &str) -> Result<()> {
    // Add your installation code here
    #[cfg(target_os = "macos")]
    crate::platform::macos::install_service(config_file)?;

    #[cfg(target_os = "linux")]
    crate::platform::linux::install_service(config_file)?;

    #[cfg(target_os = "windows")]
    crate::platform::windows::install_service(config_file)?;

    Ok(())
}

/// Uninstall the service daemon
pub fn uninstall() -> Result<()> {
    // Add your uninstallation code here
    #[cfg(target_os = "macos")]
    crate::platform::macos::uninstall_service()?;

    #[cfg(target_os = "linux")]
    crate::platform::linux::uninstall_service()?;

    #[cfg(target_os = "windows")]
    crate::platform::windows::uninstall_service()?;

    Ok(())
}
