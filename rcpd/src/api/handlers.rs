#[cfg(feature = "api")]
//! API request handlers implementation

use crate::{
    api::server::ApiState,
    error::ServiceError,
};

use axum::{
    Json, 
    extract::State, 
    http::StatusCode, 
    response::{IntoResponse, Response}
};
use serde::{Serialize, Deserialize};
use log::{debug, error, info};

/// Root handler
pub async fn root() -> &'static str {
    "RCP API Server"
}

/// Health check handler
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { 
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Status handler
pub async fn status(State(state): State<ApiState>) -> Json<StatusResponse> {
    let server_status = match &state.server {
        Some(server) => {
            let server = server.lock().await;
            let running = server.is_running().await;
            let sessions = if running {
                let sessions = server.get_sessions().await;
                Some(sessions.len())
            } else {
                None
            };
            
            ServerStatus {
                running,
                sessions,
            }
        },
        None => ServerStatus {
            running: false,
            sessions: None,
        },
    };
    
    Json(StatusResponse {
        service: "running".to_string(),
        server: server_status,
    })
}

/// Session management handler - lists all current sessions
pub async fn list_sessions(State(state): State<ApiState>) -> Json<SessionsResponse> {
    let sessions = match &state.server {
        Some(server) => {
            let server = server.lock().await;
            if server.is_running().await {
                let sessions = server.get_sessions().await;
                sessions.into_iter().map(|s| {
                    SessionInfo {
                        id: s.id().to_string(),
                        client_address: s.client_address().to_string(),
                        connected_at: s.connected_at().to_string(),
                        authenticated: s.is_authenticated(),
                    }
                }).collect()
            } else {
                vec![]
            }
        },
        None => vec![],
    };
    
    Json(SessionsResponse {
        count: sessions.len(),
        sessions,
    })
}

/// Start server handler
pub async fn start_server(State(state): State<ApiState>) -> Json<ServerActionResponse> {
    let result = match &state.server {
        Some(server) => {
            let mut server = server.lock().await;
            if !server.is_running().await {
                match server.start().await {
                    Ok(_) => "started",
                    Err(e) => {
                        error!("Failed to start server: {}", e);
                        "error"
                    }
                }
            } else {
                "already_running"
            }
        },
        None => "not_available",
    };
    
    Json(ServerActionResponse {
        action: "start".to_string(),
        result: result.to_string(),
    })
}

/// Stop server handler
pub async fn stop_server(State(state): State<ApiState>) -> Json<ServerActionResponse> {
    let result = match &state.server {
        Some(server) => {
            let mut server = server.lock().await;
            if server.is_running().await {
                match server.stop().await {
                    Ok(_) => "stopped",
                    Err(e) => {
                        error!("Failed to stop server: {}", e);
                        "error"
                    }
                }
            } else {
                "not_running"
            }
        },
        None => "not_available",
    };
    
    Json(ServerActionResponse {
        action: "stop".to_string(),
        result: result.to_string(),
    })
}

/// Get configuration handler
pub async fn get_config(State(state): State<ApiState>) -> Json<ServiceConfigResponse> {
    Json(ServiceConfigResponse {
        config: state.service_config.as_ref().clone(),
    })
}

/// Health response
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Status response
#[derive(Serialize)]
pub struct StatusResponse {
    pub service: String,
    pub server: ServerStatus,
}

/// Server status
#[derive(Serialize)]
pub struct ServerStatus {
    pub running: bool,
    pub sessions: Option<usize>,
}

/// Sessions response
#[derive(Serialize)]
pub struct SessionsResponse {
    pub count: usize,
    pub sessions: Vec<SessionInfo>,
}

/// Session information
#[derive(Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub client_address: String,
    pub connected_at: String,
    pub authenticated: bool,
}

/// Server action response
#[derive(Serialize)]
pub struct ServerActionResponse {
    pub action: String,
    pub result: String,
}

/// Service configuration response
#[derive(Serialize)]
pub struct ServiceConfigResponse {
    pub config: crate::config::ServiceConfig,
}

/// Configuration response (sanitized)
#[derive(Serialize)]
pub struct ConfigResponse {
    pub service_address: String,
    pub service_port: u16,
    pub server_enabled: bool,
    pub api_enabled: bool,
}

/// Get configuration handler (sanitized version without sensitive information)
pub async fn get_config(State(state): State<ApiState>) -> Json<ConfigResponse> {
    // Return a sanitized config (without sensitive data)
    Json(ConfigResponse {
        service_address: state.service_config.address.clone(),
        service_port: state.service_config.port,
        server_enabled: true,
        api_enabled: true,
    })
}
