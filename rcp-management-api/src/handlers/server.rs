use crate::{ApiResult, AppState};
use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct ServerStatus {
    status: String,
    stats: Option<ServerStats>,
}

#[derive(Serialize)]
pub struct ServerStats {
    active_sessions: usize,
    connected_clients: usize,
    uptime: String,
    cpu_usage: f32,
    memory_usage: String,
    memory_usage_percent: f32,
}

#[derive(Deserialize)]
pub struct ServerAction {
    action: Option<String>,
}

/// Get the current status of the RCP server
pub async fn get_status(app_state: web::Data<AppState>) -> ApiResult<HttpResponse> {
    let server_handle = match &app_state.server_handle {
        Some(handle) => handle,
        None => {
            return Ok(HttpResponse::ServiceUnavailable().json(ServerStatus {
                status: "unavailable".to_string(),
                stats: None,
            }));
        }
    };

    // Acquire a lock on the server
    let server = server_handle.lock().await;

    // Get server status
    let is_running = server.is_running();

    let status = if is_running { "running" } else { "stopped" };

    // Get server stats if it's running
    let stats = if is_running {
        let active_sessions = server.active_session_count();
        let connected_clients = server.connected_client_count();
        let uptime = server.uptime_formatted();

        // In a real implementation, we would get actual CPU and memory usage
        // Here we're just providing sample values
        Some(ServerStats {
            active_sessions,
            connected_clients,
            uptime,
            cpu_usage: 5.2, // Example value
            memory_usage: "128 MB".to_string(),
            memory_usage_percent: 8.5,
        })
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ServerStatus {
        status: status.to_string(),
        stats,
    }))
}

/// Start the RCP server
pub async fn start_server(app_state: web::Data<AppState>) -> ApiResult<HttpResponse> {
    let server_handle = match &app_state.server_handle {
        Some(handle) => handle.clone(),
        None => {
            return Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": "Server control is unavailable"
            })));
        }
    };

    // Start the server in a separate task so we don't block the API request
    let server_arc = Arc::clone(&server_handle);
    tokio::spawn(async move {
        let mut server = server_arc.lock().await;
        if !server.is_running() {
            info!("Starting RCP server from management API");
            if let Err(e) = server.start().await {
                error!("Failed to start RCP server: {}", e);
            }
        }
    });

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "starting",
        "message": "Server is starting"
    })))
}

/// Stop the RCP server
pub async fn stop_server(app_state: web::Data<AppState>) -> ApiResult<HttpResponse> {
    let server_handle = match &app_state.server_handle {
        Some(handle) => handle.clone(),
        None => {
            return Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": "Server control is unavailable"
            })));
        }
    };

    // Stop the server in a separate task
    let server_arc = Arc::clone(&server_handle);
    tokio::spawn(async move {
        let mut server = server_arc.lock().await;
        if server.is_running() {
            info!("Stopping RCP server from management API");
            if let Err(e) = server.stop().await {
                error!("Failed to stop RCP server: {}", e);
            }
        }
    });

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "stopping",
        "message": "Server is stopping"
    })))
}

/// Restart the RCP server
pub async fn restart_server(app_state: web::Data<AppState>) -> ApiResult<HttpResponse> {
    let server_handle = match &app_state.server_handle {
        Some(handle) => handle.clone(),
        None => {
            return Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": "Server control is unavailable"
            })));
        }
    };

    // Restart the server in a separate task
    let server_arc = Arc::clone(&server_handle);
    tokio::spawn(async move {
        let mut server = server_arc.lock().await;
        info!("Restarting RCP server from management API");

        // Stop the server if it's running
        if server.is_running() {
            if let Err(e) = server.stop().await {
                error!("Failed to stop RCP server during restart: {}", e);
                return;
            }
        }

        // Start the server again
        if let Err(e) = server.start().await {
            error!("Failed to start RCP server during restart: {}", e);
        }
    });

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "restarting",
        "message": "Server is restarting"
    })))
}
