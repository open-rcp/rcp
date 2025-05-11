use crate::service::ServiceClient;
use crate::{ApiError, AppState};
use axum::{extract::State, Json};
use serde::Serialize;
use std::sync::Arc;

/// Health status response
#[derive(Debug, Serialize)]
pub struct HealthStatus {
    /// API status
    api_status: String,

    /// Database status
    database_status: String,

    /// RCP service status
    service_status: String,

    /// System info
    system_info: SystemInfo,

    /// Version info
    version_info: VersionInfo,
}

/// System information
#[derive(Debug, Serialize)]
pub struct SystemInfo {
    /// Operating system
    os: String,

    /// CPU architecture
    arch: String,

    /// Number of CPUs
    cpus: usize,

    /// System uptime in seconds
    uptime: u64,

    /// Total memory in bytes
    total_memory: u64,

    /// Free memory in bytes
    free_memory: u64,
}

/// Version information
#[derive(Debug, Serialize)]
pub struct VersionInfo {
    /// API version
    api_version: String,

    /// Service version
    service_version: Option<String>,
}

/// Basic health check handler
pub async fn health_check() -> &'static str {
    "OK"
}

/// Detailed health status handler
pub async fn health_status(State(state): State<AppState>) -> Result<Json<HealthStatus>, ApiError> {
    // Check database connection
    let db_status = sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .map(|_| "healthy")
        .unwrap_or("unhealthy");

    // Check service connection
    let service_client = ServiceClient::new(Arc::clone(&state.config));
    let (service_status, service_version) = match service_client.ping().await {
        Ok(version) => ("healthy", Some(version)),
        Err(_) => ("unhealthy", None),
    };

    // Get system info
    let sys_info = SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpus: num_cpus::get(),
        uptime: get_system_uptime(),
        total_memory: get_total_memory(),
        free_memory: get_free_memory(),
    };

    // Build version info
    let version_info = VersionInfo {
        api_version: env!("CARGO_PKG_VERSION").to_string(),
        service_version,
    };

    // Build health status response
    let status = HealthStatus {
        api_status: "healthy".to_string(),
        database_status: db_status.to_string(),
        service_status: service_status.to_string(),
        system_info: sys_info,
        version_info,
    };

    Ok(Json(status))
}

// Helper function to get system uptime
#[cfg(target_os = "linux")]
fn get_system_uptime() -> u64 {
    use std::fs::read_to_string;

    read_to_string("/proc/uptime")
        .map(|uptime| {
            uptime
                .split_whitespace()
                .next()
                .unwrap_or("0")
                .parse::<f64>()
                .unwrap_or(0.0) as u64
        })
        .unwrap_or(0)
}

#[cfg(not(target_os = "linux"))]
fn get_system_uptime() -> u64 {
    // Default implementation for non-Linux systems
    // In a real app, use platform-specific APIs
    0
}

// Helper function to get total memory
#[cfg(target_os = "linux")]
fn get_total_memory() -> u64 {
    use std::fs::read_to_string;

    read_to_string("/proc/meminfo")
        .map(|meminfo| {
            let line = meminfo
                .lines()
                .find(|line| line.starts_with("MemTotal:"))
                .unwrap_or("MemTotal: 0 kB");

            line.split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or(0)
                * 1024
        })
        .unwrap_or(0)
}

#[cfg(not(target_os = "linux"))]
fn get_total_memory() -> u64 {
    // Default implementation for non-Linux systems
    // In a real app, use platform-specific APIs
    0
}

// Helper function to get free memory
#[cfg(target_os = "linux")]
fn get_free_memory() -> u64 {
    use std::fs::read_to_string;

    read_to_string("/proc/meminfo")
        .map(|meminfo| {
            let line = meminfo
                .lines()
                .find(|line| line.starts_with("MemFree:"))
                .unwrap_or("MemFree: 0 kB");

            line.split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or(0)
                * 1024
        })
        .unwrap_or(0)
}

#[cfg(not(target_os = "linux"))]
fn get_free_memory() -> u64 {
    // Default implementation for non-Linux systems
    // In a real app, use platform-specific APIs
    0
}
