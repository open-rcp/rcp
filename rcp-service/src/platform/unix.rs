use crate::error::ServiceError;
use crate::platform::Platform;
use anyhow::Result;

pub struct UnixPlatform;

impl Platform for UnixPlatform {
    fn get_socket_path() -> Result<String, ServiceError> {
        // TODO: Implement Unix-specific socket path
        Ok("/tmp/rcp-service.sock".to_string())
    }

    fn create_socket_dir() -> Result<(), ServiceError> {
        // Unix typically doesn't need special directory creation for /tmp
        Ok(())
    }

    fn cleanup_socket() -> Result<(), ServiceError> {
        // TODO: Implement socket file cleanup
        let socket_path = Self::get_socket_path()?;
        if std::path::Path::new(&socket_path).exists() {
            std::fs::remove_file(socket_path).map_err(|e| {
                ServiceError::Service(format!("Failed to remove socket file: {}", e))
            })?;
        }
        Ok(())
    }
}
