use crate::{config::ServiceConfig, error::ServiceError, manager::ServiceManager};
use anyhow::Result;
use daemonize::Daemonize;
use log::{error, info};
use std::fs::File;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Service daemon that runs in the background
pub struct ServiceDaemon {
    /// Configuration
    config: ServiceConfig,

    /// Working directory
    work_dir: PathBuf,

    /// Shutdown channel receiver
    shutdown_rx: mpsc::Receiver<()>,
}

impl ServiceDaemon {
    /// Create a new service daemon
    pub fn new(config: ServiceConfig, work_dir: PathBuf, shutdown_rx: mpsc::Receiver<()>) -> Self {
        Self {
            config,
            work_dir,
            shutdown_rx,
        }
    }

    /// Start the daemon
    pub async fn start(&mut self) -> Result<(), ServiceError> {
        info!("Starting service daemon");

        // Create service manager with all required parameters
        let (shutdown_tx, _) = mpsc::channel::<()>(1);
        let mut service_manager =
            ServiceManager::new(self.work_dir.clone(), self.config.clone(), shutdown_tx);

        // Start the manager
        service_manager.start().await?;

        // Wait for shutdown signal
        self.wait_for_shutdown().await;

        Ok(())
    }

    /// Wait for shutdown signal
    async fn wait_for_shutdown(&mut self) {
        if let Some(_) = self.shutdown_rx.recv().await {
            info!("Shutdown signal received");
        }
    }
}

/// Daemonize the current process
pub fn daemonize(work_dir: &PathBuf) -> Result<()> {
    info!("Daemonizing process");

    // Create PID file for daemon
    #[cfg(feature = "cli")]
    let pid_file = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rcpd.pid");
    #[cfg(not(feature = "cli"))]
    let pid_file = std::env::temp_dir().join("rcpd.pid");

    // Create log file for daemon
    #[cfg(feature = "cli")]
    let log_file = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rcpd.log");
    #[cfg(not(feature = "cli"))]
    let log_file = std::env::temp_dir().join("rcpd.log");

    // Create daemonize config
    let daemonize = Daemonize::new()
        .pid_file(pid_file)
        .chown_pid_file(true)
        .working_directory(work_dir)
        .stdout(File::create(&log_file).unwrap())
        .stderr(File::create(&log_file).unwrap());

    // Start daemon
    daemonize.start().map_err(|e| {
        error!("Error starting daemon: {}", e);
        anyhow::anyhow!("Failed to start daemon: {}", e)
    })?;

    Ok(())
}

/// Start the daemon service
pub fn start(config: ServiceConfig, work_dir: PathBuf) -> Result<()> {
    info!("Starting daemon service");

    // Create runtime for daemon
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // Start daemon in runtime
    runtime.block_on(async {
        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);

        // Setup signal handlers
        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

        // Spawn signal handler tasks
        tokio::spawn(async move {
            tokio::select! {
                _ = sigterm.recv() => {
                    info!("Shutdown signal received");
                    let _ = shutdown_tx.send(()).await;
                }
                _ = sigint.recv() => {
                    info!("Ctrl+C received, shutting down");
                    let _ = shutdown_tx.send(()).await;
                }
            }
        });

        // Create and start daemon
        let mut daemon = ServiceDaemon::new(config, work_dir, shutdown_rx);
        daemon
            .start()
            .await
            .map_err(|e| anyhow::anyhow!("Daemon error: {}", e))
    })?;

    Ok(())
}

pub fn status() -> Result<String> {
    // Find PID file
    #[cfg(feature = "cli")]
    let pid_file = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rcpd.pid");
    #[cfg(not(feature = "cli"))]
    let pid_file = std::env::temp_dir().join("rcpd.pid");

    // Check if PID file exists
    if !pid_file.exists() {
        return Ok("Not running".to_string());
    }

    // Read PID from file
    let pid_data = std::fs::read_to_string(&pid_file)?;
    let pid: i32 = pid_data.trim().parse()?;

    // Check if process is running
    let status = unsafe { libc::kill(pid, 0) };

    if status == 0 {
        Ok(format!("Running (PID: {})", pid))
    } else {
        Ok("Not running (stale PID file)".to_string())
    }
}

pub fn stop() -> Result<()> {
    info!("Stopping daemon");

    // Find PID file
    #[cfg(feature = "cli")]
    let pid_file = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rcpd.pid");
    #[cfg(not(feature = "cli"))]
    let pid_file = std::env::temp_dir().join("rcpd.pid");

    // Read PID from file
    if !pid_file.exists() {
        return Err(anyhow::anyhow!("Daemon not running (no PID file)"));
    }

    // Read PID from file
    let pid_data = std::fs::read_to_string(&pid_file)?;
    let pid: i32 = pid_data.trim().parse()?;

    // Kill process
    unsafe {
        libc::kill(pid, libc::SIGTERM);
    }

    // Remove PID file
    std::fs::remove_file(&pid_file)?;

    info!("Daemon stopped");
    Ok(())
}
