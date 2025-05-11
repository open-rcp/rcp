use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::sync::Arc;
use tauri::{generate_handler, command, State};
use tokio::sync::Mutex;
use tauri_plugin_dialog;
use tauri_plugin_shell;
use rcp_client::{ClientBuilder, Client};
use rcp_core::AuthMethod;
use uuid::Uuid;
use thiserror::Error;

#[serde_as]
#[derive(Debug, Error, Serialize, Clone)]
pub enum ClientError {
    #[error("Client error: {0}")]
    ClientError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Application error: {0}")]
    AppError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<rcp_client::error::Error> for ClientError {
    fn from(err: rcp_client::error::Error) -> Self {
        ClientError::ClientError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, ClientError>;

#[derive(Default)]
pub struct ClientState {
    client: Arc<Mutex<Option<Client>>>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResult {
    success: bool,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionConfig {
    host: String,
    port: u16,
    remember_credentials: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AppInfo {
    id: String,
    name: String,
    icon: String,
    description: String,
    last_used: Option<String>,
}

mod commands {
    use super::*;
    
    #[command]
    pub async fn connect(
        state: State<'_, ClientState>,
        host: String,
        port: u16
    ) -> Result<bool> {
        let mut client_lock = state.client.lock().await;
        
        if client_lock.is_some() {
            return Ok(false);
        }
        
        // Build client
        let client = ClientBuilder::new()
            .host(&host)
            .port(port)
            .client_name("RCP-Desk")
            .client_id(Uuid::new_v4())
            .build();
        
        // Connect to server
        client.connect().await?;
        
        // Store client
        *client_lock = Some(client);
        
        Ok(true)
    }
    
    #[command]
    pub async fn login(
        state: State<'_, ClientState>,
        username: String,
        password: String,
        _remember_credentials: bool
    ) -> Result<AuthResult> {
        let mut client_lock = state.client.lock().await;
        
        if let Some(client) = &mut *client_lock {
            // Set auth method
            client.set_auth_method(AuthMethod::Password(username, password)).await?;
            
            // Authenticate
            match client.authenticate().await {
                Ok(_) => {
                    // Start command processing
                    client.start().await?;
                    
                    Ok(AuthResult {
                        success: true,
                        message: None,
                    })
                }
                Err(e) => {
                    Ok(AuthResult {
                        success: false,
                        message: Some(e.to_string()),
                    })
                }
            }
        } else {
            Ok(AuthResult {
                success: false,
                message: Some("Not connected to server".to_string()),
            })
        }
    }
    
    #[command]
    pub async fn get_available_apps(state: State<'_, ClientState>) -> Result<Vec<AppInfo>> {
        let client_lock = state.client.lock().await;
        
        if let Some(_client) = &*client_lock {
            // In a real implementation, we would query the server for available apps
            // For now, let's return some demo apps
            let apps = vec![
                AppInfo {
                    id: "notepad".to_string(),
                    name: "Notepad".to_string(),
                    icon: "app-icon.png".to_string(),
                    description: "Simple text editor".to_string(),
                    last_used: Some("2025-05-08T14:30:00Z".to_string()),
                },
                AppInfo {
                    id: "calculator".to_string(),
                    name: "Calculator".to_string(),
                    icon: "app-icon.png".to_string(),
                    description: "Basic calculator application".to_string(),
                    last_used: None,
                },
                AppInfo {
                    id: "browser".to_string(),
                    name: "Web Browser".to_string(),
                    icon: "app-icon.png".to_string(),
                    description: "Secure web browser".to_string(),
                    last_used: Some("2025-05-07T09:15:00Z".to_string()),
                },
            ];
            
            Ok(apps)
        } else {
            Err(ClientError::ConnectionError("Not connected to server".to_string()))
        }
    }
    
    #[command]
    pub async fn launch_app(
        state: State<'_, ClientState>,
        _app_id: String
    ) -> Result<bool> {
        let client_lock = state.client.lock().await;
        
        if let Some(_client) = &*client_lock {
            // In a real implementation, this would launch the app via rcp-client
            // and return the streaming session information
            
            // For now, let's simulate success
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            Ok(true)
        } else {
            Err(ClientError::ConnectionError("Not connected to server".to_string()))
        }
    }
    
    #[command]
    pub async fn close_app(
        state: State<'_, ClientState>,
        _app_id: String
    ) -> Result<bool> {
        let client_lock = state.client.lock().await;
        
        if let Some(_client) = &*client_lock {
            // In a real implementation, this would close the app via rcp-client
            
            // For now, let's simulate success
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            
            Ok(true)
        } else {
            Err(ClientError::ConnectionError("Not connected to server".to_string()))
        }
    }
    
    #[command]
    pub fn get_saved_credentials() -> Option<ConnectionConfig> {
        // In a real implementation, this would retrieve saved credentials from secure storage
        None
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(ClientState::default())
        .invoke_handler(generate_handler![
            commands::connect,
            commands::login,
            commands::get_available_apps,
            commands::launch_app,
            commands::close_app,
            commands::get_saved_credentials,
        ])
        .setup(|_app| {
            // Any setup code can go here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
