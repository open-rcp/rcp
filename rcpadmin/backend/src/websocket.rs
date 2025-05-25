use std::{sync::Arc, time::Duration};
use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tracing::{info, error};

use crate::{error::Result, models::SystemMetrics, AppState};

// Channel size for broadcasting updates to connected clients
const CHANNEL_SIZE: usize = 32;

type MetricsSender = broadcast::Sender<SystemMetrics>;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/metrics", get(ws_metrics_handler))
}

async fn ws_metrics_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    info!("New WebSocket connection for metrics");
    
    ws.on_upgrade(|socket| async move {
        handle_metrics_socket(socket, state).await
    })
}

async fn handle_metrics_socket(
    socket: WebSocket,
    state: AppState,
) {
    // Split socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();
    
    // Spawn a task to periodically send metrics
    let mut send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
            // Generate sample metrics (in production, fetch from daemon)
            let metrics = SystemMetrics {
                cpu_usage: 25.5,
                memory_usage: 1024 * 1024 * 512, // 512MB
                total_memory: 1024 * 1024 * 1024 * 8, // 8GB
                disk_usage: 1024 * 1024 * 1024 * 10, // 10GB
                total_disk: 1024 * 1024 * 1024 * 100, // 100GB
            };
            
            // Convert metrics to JSON
            let json = match serde_json::to_string(&metrics) {
                Ok(json) => json,
                Err(e) => {
                    error!("Failed to serialize metrics: {}", e);
                    continue;
                }
            };
            
            // Send message to client
            if sender.send(Message::Text(json)).await.is_err() {
                // Client disconnected
                break;
            }
        }
    });
    
    // Spawn a task to receive messages from the client (for pings/pongs and close detection)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Ping(_bytes) => {
                    // Note: We can't respond to ping here since sender was moved
                    // In a real implementation, you'd use channels to communicate between tasks
                    break;
                }
                _ => {} // Ignore other messages
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
    
    info!("WebSocket connection closed");
}