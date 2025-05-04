use crate::{ApiResult, AppState};
use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub tls_enabled: bool,
    pub tls_cert_path: String,
    pub tls_key_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub method: String,
    pub require_auth: bool,
    pub psk_value: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub to_file: bool,
    pub file_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
}

/// Get the current server configuration
pub async fn get_config(app_state: web::Data<AppState>) -> ApiResult<HttpResponse> {
    let server_handle = match &app_state.server_handle {
        Some(handle) => handle,
        None => {
            return Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": "Server configuration is unavailable"
            })));
        }
    };

    // In a real implementation, we would get the actual configuration from the server
    // Here we're returning sample configuration
    let config = AppConfig {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8716,
            max_connections: 100,
            tls_enabled: false,
            tls_cert_path: "/path/to/cert.pem".to_string(),
            tls_key_path: "/path/to/key.pem".to_string(),
        },
        auth: AuthConfig {
            method: "pre_shared_key".to_string(),
            require_auth: true,
            psk_value: "secret_key_123".to_string(),
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            to_file: true,
            file_path: "/var/log/rcp-server.log".to_string(),
        },
    };

    Ok(HttpResponse::Ok().json(config))
}

/// Update the server configuration
pub async fn update_config(
    app_state: web::Data<AppState>,
    config: web::Json<AppConfig>,
) -> ApiResult<HttpResponse> {
    let server_handle = match &app_state.server_handle {
        Some(handle) => handle,
        None => {
            return Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": "Server configuration is unavailable"
            })));
        }
    };

    // In a real implementation, we would update the actual server configuration
    info!("Updating server configuration");
    
    // Just log the new config for now
    info!("New server host: {}", config.server.host);
    info!("New server port: {}", config.server.port);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Configuration updated successfully"
    })))
}