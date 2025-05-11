use crate::error::ServiceError;
use crate::platform::Platform;
use anyhow::Result;

pub struct WindowsPlatform;

#[cfg(target_family = "windows")]
impl Platform for WindowsPlatform {
    fn get_socket_path() -> Result<String, ServiceError> {
        // TODO: Implement Windows-specific named pipe path
        Ok("\\\\.\\pipe\\rcp-service".to_string())
    }

    fn create_socket_dir() -> Result<(), ServiceError> {
        // Named pipes on Windows don't require directory creation
        Ok(())
    }

    fn cleanup_socket() -> Result<(), ServiceError> {
        // Named pipes on Windows are closed automatically when the process exits
        Ok(())
    }
}
