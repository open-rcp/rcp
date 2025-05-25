use axum::{extract::State, response::Json, routing::get, Router};
use tracing::info;

use crate::{error::Result, models::SystemMetrics, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new().route("/metrics", get(get_metrics))
}

async fn get_metrics(State(state): State<AppState>) -> Result<Json<SystemMetrics>> {
    info!("Retrieving system metrics");
    let metrics = state.rcpdaemon_client.get_system_metrics().await?;
    Ok(Json(metrics))
}
