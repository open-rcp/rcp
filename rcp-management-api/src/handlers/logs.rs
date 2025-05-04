use crate::{ApiResult, AppState};
use actix_web::{web, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Deserialize)]
pub struct LogQueryParams {
    pub level: Option<String>,
    pub limit: Option<usize>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

/// Get server logs with optional filtering
pub async fn get_logs(
    app_state: web::Data<AppState>,
    query: web::Query<LogQueryParams>,
) -> ApiResult<HttpResponse> {
    // In a real implementation, we would fetch logs from the server or log storage
    info!("Fetching logs with filters: level={:?}, limit={:?}", 
          query.level, query.limit);
    
    // Generate some sample log entries for demonstration
    let logs = vec![
        LogEntry {
            timestamp: Utc::now(),
            level: "info".to_string(),
            message: "Server started successfully".to_string(),
            source: Some("rcp_server::main".to_string()),
        },
        LogEntry {
            timestamp: Utc::now(),
            level: "info".to_string(),
            message: "Client connected (id=client-001)".to_string(),
            source: Some("rcp_server::session".to_string()),
        },
        LogEntry {
            timestamp: Utc::now(),
            level: "warning".to_string(),
            message: "Authentication attempt failed for user 'guest'".to_string(),
            source: Some("rcp_server::auth".to_string()),
        },
        LogEntry {
            timestamp: Utc::now(),
            level: "error".to_string(),
            message: "Failed to read configuration: file not found".to_string(),
            source: Some("rcp_server::config".to_string()),
        },
    ];
    
    // Apply level filter if specified
    let filtered_logs = if let Some(ref level) = query.level {
        logs.into_iter()
            .filter(|log| log.level == *level)
            .collect::<Vec<_>>()
    } else {
        logs
    };
    
    // Apply limit if specified
    let limited_logs = if let Some(limit) = query.limit {
        filtered_logs.into_iter().take(limit).collect::<Vec<_>>()
    } else {
        filtered_logs
    };

    Ok(HttpResponse::Ok().json(limited_logs))
}