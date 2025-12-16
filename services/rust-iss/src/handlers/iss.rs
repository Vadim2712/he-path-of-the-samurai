use axum::{
    extract::State,
    Json,
};
use serde_json::Value;

use crate::domain::models::AppState;
use crate::domain::error::ApiError;
use crate::domain::models::Trend;
use tracing::info;

/// Gets the most recent ISS log.
pub async fn last_iss(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    info!("Received request for last ISS position");
    let log_opt = state.iss_service.get_last_iss().await.map_err(ApiError::from)?;

    match log_opt {
        Some(log) => Ok(Json(log)),
        None => Ok(Json(serde_json::json!({ "message": "no data" }))),
    }
}

/// Triggers a new fetch of ISS data and returns the latest log.
pub async fn trigger_iss(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    state.iss_service.fetch_and_store_iss().await.map_err(ApiError::from)?;
    // Call the other handler to avoid duplicating logic
    last_iss(State(state)).await
}

/// Calculates and returns the movement trend of the ISS.
pub async fn iss_trend(State(state): State<AppState>) -> Result<Json<Trend>, ApiError> {
    info!("Received request for ISS trend");
    let trend = state.iss_service.get_trend().await.map_err(ApiError::from)?;
    Ok(Json(trend))
}
