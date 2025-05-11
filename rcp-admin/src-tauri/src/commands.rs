// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcp-admin/src-tauri/src/commands.rs
// Minimal implementation to avoid SIGBUS errors

// Allow dead code to suppress warnings since this is a stub implementation
#[allow(dead_code, unused_variables)]

// Basic greeting command that the frontend tries to invoke
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub struct ServerStatus {
    pub running: bool,
    pub uptime: String,
    pub version: String,
    pub connections: usize,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

pub struct VirtualAppInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub file_associations: Vec<String>,
}

// Mock data helper
pub fn mock_virtual_apps() -> Vec<VirtualAppInfo> {
    vec![
        VirtualAppInfo {
            id: "app1".to_string(),
            name: "Notepad".to_string(),
            path: "/usr/bin/notepad".to_string(),
            file_associations: vec![".txt".to_string(), ".md".to_string()],
        },
        VirtualAppInfo {
            id: "app2".to_string(),
            name: "Calculator".to_string(),
            path: "/usr/bin/calc".to_string(),
            file_associations: vec![],
        }
    ]
}

// Sample command to check connection status
pub fn get_connection_status(_state: &super::RcpState) -> bool {
    // For simplicity, return a fixed value instead of accessing a mutex that may cause issues
    true
}

// Sample command to set connection status
pub fn set_connection_status(_state: &super::RcpState, status: bool) {
    // No-op implementation to avoid mutex access issues
    println!("Setting connection status to: {}", status);
}

// Command to get virtual apps
pub fn get_virtual_apps() -> Vec<VirtualAppInfo> {
    mock_virtual_apps()
}

// Command to get server status
pub fn get_server_status() -> ServerStatus {
    ServerStatus {
        running: true,
        uptime: "2h 15m".to_string(),
        version: "1.0.0".to_string(),
        connections: 5,
        cpu_usage: 12.5,
        memory_usage: 256.4,
    }
}
