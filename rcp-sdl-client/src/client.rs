use rcp_client::{Client, ClientBuilder};
use rcp_core::AuthMethod;
use std::error::Error;
use tokio::time::Duration;

// Define AppInfo struct for application metadata
#[derive(Debug, Clone)]
pub struct AppInfo {
    pub id: String,
    pub name: String, 
    pub description: String,
    pub last_used: Option<String>,
}

// RCP client wrapper for async operations
pub struct RcpClient {
    pub(crate) client: Option<Client>,
    host: String,
    port: u16,
}

impl RcpClient {
    pub fn new() -> Self {
        Self {
            client: None,
            host: "127.0.0.1".to_string(),
            port: 8717,
        }
    }
    
    // Set connection parameters
    pub fn set_connection(&mut self, host: String, port: u16) {
        self.host = host;
        self.port = port;
    }
    
    // Get a reference to the client
    pub fn client(&self) -> Option<&Client> {
        self.client.as_ref()
    }
    
    // Get a mutable reference to the client
    pub fn client_mut(&mut self) -> Option<&mut Client> {
        self.client.as_mut()
    }
    
    // Check if we have a connected client
    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }
    
    // Set the client directly (for internal use)
    pub fn set_client(&mut self, client: Client) {
        self.client = Some(client);
    }
    
    // Connect to the RCP server
    pub async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Build client
        let client = ClientBuilder::new()
            .host(&self.host)
            .port(self.port)
            .client_name("RCP-SDL-Client")
            .build();
        
        // Connect to server
        match client.connect().await {
            Ok(_) => {
                // Store client
                self.client = Some(client);
                Ok(())
            },
            Err(e) => {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))))
            }
        }
    }
    
    // Authenticate with the RCP server
    pub async fn authenticate(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(client) = &mut self.client {
            // Set authentication method
            match client.set_auth_method(AuthMethod::Password(username.to_string(), password.to_string())).await {
                Ok(_) => {},
                Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))),
            }
            
            // Authenticate
            match client.authenticate().await {
                Ok(_) => {},
                Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))),
            }
            
            // Start client processing
            match client.start().await {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))),
            }
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to server")))
        }
    }
    
    // Get available applications
    pub async fn get_applications(&self) -> Result<Vec<AppInfo>, Box<dyn Error + Send + Sync>> {
        if let Some(_client) = &self.client {
            // In a real implementation, this would query the server for applications
            // For now, return some demo applications
            let apps = vec![
                AppInfo {
                    id: "notepad".to_string(),
                    name: "Notepad".to_string(),
                    description: "Simple text editor".to_string(),
                    last_used: Some("2025-05-08T14:30:00Z".to_string()),
                },
                AppInfo {
                    id: "calculator".to_string(),
                    name: "Calculator".to_string(),
                    description: "Basic calculator application".to_string(),
                    last_used: None,
                },
                AppInfo {
                    id: "browser".to_string(),
                    name: "Web Browser".to_string(),
                    description: "Secure web browser".to_string(),
                    last_used: Some("2025-05-07T09:15:00Z".to_string()),
                },
            ];
            
            Ok(apps)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to server")))
        }
    }
    
    // Launch an application
    pub async fn launch_application(&self, app_id: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(_client) = &self.client {
            // In a real implementation, this would launch the application via the RCP client
            // and handle streaming setup
            println!("Launching application: {}", app_id);
            
            // Simulate a small delay for the launch
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to server")))
        }
    }
    
    // Close an application
    pub async fn close_application(&self, app_id: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(_client) = &self.client {
            // In a real implementation, this would close the application via the RCP client
            println!("Closing application: {}", app_id);
            
            // Simulate a small delay for cleanup
            tokio::time::sleep(Duration::from_millis(300)).await;
            
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to server")))
        }
    }
    
    // Disconnect from the server
    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(client) = &self.client {
            match client.disconnect().await {
                Ok(_) => {
                    self.client = None;
                    Ok(())
                },
                Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))))
            }
        } else {
            Ok(()) // Already disconnected
        }
    }
}

// End of file
