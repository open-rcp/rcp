use tauri::command;
use rcp_client::Client;
use rcp_core::{auth::AuthInfo, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

// State management for the RCP client
pub struct RcpState {
    client: Arc<Mutex<Option<Client>>>,
}

impl RcpState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
        }
    }
}

#[command]
pub async fn connect_server(
    state: tauri::State<'_, RcpState>,
    host: String,
    port: u16,
) -> Result<bool> {
    let mut client = Client::connect(&host, port).await?;
    *state.client.lock().await = Some(client);
    Ok(true)
}

#[command]
pub async fn login(
    state: tauri::State<'_, RcpState>,
    username: String,
    password: String,
) -> Result<AuthInfo> {
    let mut client_lock = state.client.lock().await;
    let client = client_lock.as_mut().ok_or_else(|| {
        rcp_core::Error::ConnectionError("Not connected to server".to_string())
    })?;
    
    client.authenticate(username, password).await
}

#[command]
pub async fn get_virtual_apps(
    state: tauri::State<'_, RcpState>,
) -> Result<Vec<VirtualAppInfo>> {
    let mut client_lock = state.client.lock().await;
    let client = client_lock.as_mut().ok_or_else(|| {
        rcp_core::Error::ConnectionError("Not connected to server".to_string())
    })?;
    
    client.get_available_applications().await
}

#[command]
pub async fn launch_virtual_app(
    state: tauri::State<'_, RcpState>,
    app_id: String,
    args: Vec<String>,
) -> Result<String> {
    let mut client_lock = state.client.lock().await;
    let client = client_lock.as_mut().ok_or_else(|| {
        rcp_core::Error::ConnectionError("Not connected to server".to_string())
    })?;
    
    client.launch_application(&app_id, args).await
}

#[derive(serde::Serialize)]
pub struct VirtualAppInfo {
    pub id: String,
    pub name: String,
    pub file_associations: Vec<String>,
}
