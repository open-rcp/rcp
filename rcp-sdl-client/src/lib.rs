pub mod client;
pub mod ui;
pub mod sdl_egui;

// Re-export commonly used types
pub use client::{RcpClient, AppInfo};
pub use sdl_egui::EguiSDL2;

// Application state
pub struct AppState {
    pub client: RcpClient,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub remember_credentials: bool,
    pub connected: bool,
    pub authenticated: bool,
    pub error_message: Option<String>,
    pub apps: Vec<AppInfo>,
    pub active_app_id: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: RcpClient::new(),
            host: "localhost".to_string(),
            port: 8717, // Default RCP server port
            username: String::new(),
            password: String::new(),
            remember_credentials: false,
            connected: false, 
            authenticated: false,
            error_message: None,
            apps: Vec::new(),
            active_app_id: None,
        }
    }
    
    // Connect to the RCP server
    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Set connection parameters
        self.client.set_connection(self.host.clone(), self.port);
        
        // Connect to server
        match self.client.connect().await {
            Ok(_) => {
                println!("Connected to server!");
                self.connected = true;
                self.error_message = None;
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Failed to connect: {}", e);
                println!("{}", error_msg);
                self.error_message = Some(error_msg);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))))
            }
        }
    }
    
    // Login to the RCP server
    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.client.authenticate(&self.username, &self.password).await {
            Ok(_) => {
                println!("Authentication successful!");
                self.authenticated = true;
                self.error_message = None;
                
                // Load available applications
                self.load_apps().await?;
                
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Authentication failed: {}", e);
                println!("{}", error_msg);
                self.error_message = Some(error_msg);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))))
            }
        }
    }
    
    // Load available applications
    pub async fn load_apps(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.client.get_applications().await {
            Ok(apps) => {
                self.apps = apps;
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Failed to load applications: {}", e);
                println!("{}", error_msg);
                self.error_message = Some(error_msg);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))))
            }
        }
    }
    
    // Launch an application
    pub async fn launch_app(&mut self, app_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.client.launch_application(app_id).await {
            Ok(_) => {
                println!("Launching application: {}", app_id);
                self.active_app_id = Some(app_id.to_string());
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Failed to launch application: {}", e);
                println!("{}", error_msg);
                self.error_message = Some(error_msg);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))))
            }
        }
    }
    
    // Close the active application
    pub async fn close_app(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(app_id) = &self.active_app_id {
            match self.client.close_application(app_id).await {
                Ok(_) => {
                    println!("Closed application: {}", app_id);
                    self.active_app_id = None;
                    Ok(())
                },
                Err(e) => {
                    let error_msg = format!("Failed to close application: {}", e);
                    println!("{}", error_msg);
                    self.error_message = None; // Don't show error to user
                    self.active_app_id = None; // Force close anyway
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))))
                }
            }
        } else {
            Ok(())
        }
    }
}
