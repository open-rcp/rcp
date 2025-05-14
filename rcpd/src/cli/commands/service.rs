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
    formatter: &OutputFormatter,
) -> Result<(), CliError> {
    match client.get_status().await {
        Ok(status) => {
            if formatter.json_output {
                formatter.json(&status)?;
                return Ok(());
            }

            formatter.info("RCPD Service Status:");
            formatter.info(&format!(
                "Status: {}",
                if status.running { "Running" } else { "Stopped" }
            ));

            if let Some(pid) = status.pid {
                formatter.info(&format!("Process ID: {}", pid));
            }

            if let Some(uptime) = status.uptime {
                formatter.info(&format!("Uptime: {}", uptime));
            }

            formatter.info(&format!("Version: {}", status.version));
            Ok(())
        }
        Err(e) => {
            if let CliError::CommunicationError(_) = e {
                formatter.warning("Could not connect to RCPD service");
                Ok(())
            } else {
                formatter.error(&format!("Failed to get service status: {}", e));
                Err(e)
            }
        }
    }
}
