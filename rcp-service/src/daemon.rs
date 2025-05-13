use std::path::PathBuf;
use crate::{
    config::ServiceConfig,
    error::ServiceError,
    manager::ServiceManager
};
use anyhow::Result;
use daemonize::Daemonize;
use log::{info, error};
use std::fs::File;
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
            shutdown_rx 
        }
    }

    /// Run the daemon service
    pub async fn run(&mut self) -> Result<(), ServiceError> {
        // Create service manager with shutdown channel
        let (shutdown_tx, _) = mpsc::channel(1);
        let mut manager = ServiceManager::new(
            self.work_dir.clone(),
            self.config.clone(),
            shutdown_tx
        );
        
        // Start the service (with integrated server)
        manager.start().await?;
        
        // Wait for shutdown signal
        tokio::select! {
            _ = self.shutdown_rx.recv() => {
                info!("Received shutdown signal, stopping service...");
                manager.stop().await?;
                info!("Service stopped");
            }
        }
        
        Ok(())
    }
}

pub fn start(config_path: &str) -> Result<()> {
    info!("Starting daemon with config: {}", config_path);
    
    // Load configuration
    let config = match ServiceConfig::from_file(config_path) {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        },
        Err(e) => {
            info!("Failed to load config from {}: {}. Using defaults.", config_path, e);
            ServiceConfig::default()
        }
    };
    
    // Get working directory from config path
    let work_dir = PathBuf::from(config_path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    
    // Create PID file for daemon
    let pid_file = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rcp-service.pid");
    
    // Create log file for daemon
    let log_file = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rcp-service.log");
    
    // Create daemonize struct
    let daemon = Daemonize::new()
        .pid_file(pid_file)
        .working_directory(work_dir.clone())
        .stdout(File::create(&log_file).unwrap())
        .stderr(File::create(&log_file).unwrap());
    
    // Start daemon
    match daemon.start() {
        Ok(_) => {
            // This code runs in the daemon process
            
            // Create runtime for async functions in a separate thread
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    // Create shutdown channel
                    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
                    
                    // Create daemon service
                    let mut daemon_service = ServiceDaemon::new(config, work_dir, shutdown_rx);
                    
                    // Run the service
                    if let Err(e) = daemon_service.run().await {
                        error!("Daemon service error: {}", e);
                    }
                    
                    // Ensure we drop the sender to close the channel
                    drop(shutdown_tx);
                });
            });
            
            Ok(())
        },
        Err(e) => {
            error!("Failed to start daemon: {}", e);
            Err(anyhow::anyhow!("Failed to start daemon: {}", e))
        }
    }
}

pub fn stop() -> Result<()> {
    info!("Stopping daemon");
    
    // Find PID file
    let pid_file = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rcp-service.pid");
    
    // Read PID from file
    if !pid_file.exists() {
        return Err(anyhow::anyhow!("Service not running (no PID file found)"));
    }
    
    let pid_str = std::fs::read_to_string(&pid_file)?;
    let pid: i32 = pid_str.trim().parse()?;
    
    // Send SIGTERM
    #[cfg(unix)]
    unsafe {
        libc::kill(pid, libc::SIGTERM);
    }
    
    #[cfg(not(unix))]
    return Err(anyhow::anyhow!("Stop not implemented for this platform"));
    
    // Remove PID file
    std::fs::remove_file(&pid_file)?;
    
    info!("Daemon stopped");
    Ok(())
}

pub fn restart(config_path: &str) -> Result<()> {
    // Attempt to stop, but continue even if failed
    let _ = stop();
    start(config_path)?;
    Ok(())
}

pub fn status() -> Result<String> {
    // TODO: Check daemon status
    Ok("Unknown".to_string())
}
