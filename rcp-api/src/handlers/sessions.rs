use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::handlers::auth::AuthUser;
use crate::{db, ApiError, AppState};

/// Session information response
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    id: String,
    server_id: String,
    server_name: Option<String>,
    user_id: Option<String>,
    username: Option<String>,
    connected_at: String,
    client_address: String,
    client_info: Option<String>,
}

/// Session query parameters
#[derive(Debug, Deserialize)]
pub struct SessionQuery {
    server_id: Option<String>,
}

/// Message request to send to session
#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    message: String,
    message_type: Option<String>,
}

/// List all active sessions
pub async fn list_sessions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<SessionQuery>,
) -> Result<Json<Vec<SessionResponse>>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Get session list from service
    let sessions = service_client
        .list_sessions(params.server_id.clone())
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list sessions: {}", e)))?;

    // Get user information from the service if needed
    // Note: Instead of querying a local user table, we should get user information
    // from the RCP service which manages users

    // Create a map of server IDs to server names
    let servers = service_client
        .list_servers()
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list servers: {}", e)))?;

    let server_map: HashMap<String, String> = servers
        .into_iter()
        .map(|s| (s.id, s.name))
        .collect::<HashMap<String, String>>();

    // Convert to response format
    let responses = sessions
        .into_iter()
        .map(|session| {
            // The username would need to be obtained from the service in a real implementation
            // For now, we'll use a placeholder based on user_id
            let username = if !session.user_id.is_empty() {
                Some(format!("user-{}", session.user_id))
            } else {
                None
            };

            let server_name = server_map.get(&session.server_id).cloned();

            // Clone server_id before using it to avoid move issues
            let server_id = session.server_id.clone();

            SessionResponse {
                id: session.id,
                server_id: session.server_id.clone(),
                server_name,
                user_id: if session.user_id.is_empty() {
                    None
                } else {
                    Some(session.user_id)
                },
                username,
                connected_at: session.started_at,
                client_address: format!("{}-client", server_id), // Placeholder since field isn't available
                client_info: None,                               // No equivalent field available
            }
        })
        .collect();

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "list_sessions",
        None,
        None,
        params
            .server_id
            .as_ref()
            .map(|id| format!("server_id={}", id))
            .as_deref(),
    )
    .await?;

    Ok(Json(responses))
}

/// Get a specific session by ID
pub async fn get_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<SessionResponse>, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Get sessions from service
    let sessions = service_client
        .list_sessions(None)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list sessions: {}", e)))?;

    // Find the specific session
    let session = sessions
        .into_iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFoundError(format!("Session '{}' not found", id)))?;

    // Get username from the service or create a placeholder
    // In a complete implementation, we would make an RPC call to get user details
    let username = if !session.user_id.is_empty() {
        Some(format!("user-{}", session.user_id))
    } else {
        None
    };

    // Get server name
    let servers = service_client
        .list_servers()
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to list servers: {}", e)))?;

    let server_name = servers
        .into_iter()
        .find(|s| s.id == session.server_id)
        .map(|s| s.name);

    // Convert to response format
    // Clone server_id before using it to avoid move errors
    let server_id_clone = session.server_id.clone();

    let response = SessionResponse {
        id: session.id,
        server_id: server_id_clone,
        server_name,
        user_id: if session.user_id.is_empty() {
            None
        } else {
            Some(session.user_id)
        },
        username,
        connected_at: session.started_at,
        client_address: session.server_id.clone(), // Use server_id as placeholder since client_address doesn't exist
        client_info: None,                         // No equivalent field available
    };

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "get_session",
        Some("session"),
        Some(&id),
        None,
    )
    .await?;

    Ok(Json(response))
}

/// Terminate a session
pub async fn terminate_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Send terminate command to service
    // This is a placeholder as the actual command might vary
    let command = "terminate-session";
    let args = serde_json::to_vec(&id)?;
    service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to terminate session: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "terminate_session",
        Some("session"),
        Some(&id),
        None,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Send a message to a session
pub async fn send_message(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<MessageRequest>,
) -> Result<StatusCode, ApiError> {
    // Get service client to call the RCP service
    let service_client = state.service_client.lock().await;

    // Prepare the message request
    #[derive(Serialize)]
    struct MessageCommand<'a> {
        session_id: &'a str,
        message: &'a str,
        message_type: &'a str,
    }

    let message_command = MessageCommand {
        session_id: &id,
        message: &payload.message,
        message_type: payload.message_type.as_deref().unwrap_or("text"),
    };

    // Send message command to service
    let command = "send-message";
    let args = serde_json::to_vec(&message_command)?;
    service_client
        .send_command(command, &args)
        .await
        .map_err(|e| ApiError::ServiceError(format!("Failed to send message: {}", e)))?;

    // Log the action
    db::add_audit_log(
        &state.db_pool,
        Some(&auth_user.id),
        "send_message",
        Some("session"),
        Some(&id),
        Some(&format!(
            "message_type={}",
            payload.message_type.as_deref().unwrap_or("text")
        )),
    )
    .await?;

    Ok(StatusCode::OK)
}
