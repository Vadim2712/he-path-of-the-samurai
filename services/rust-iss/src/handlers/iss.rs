use axum::{
    extract::State,
    Json,
};
use serde_json::Value;

use crate::domain::models::AppState;
use crate::domain::error::ApiError;
use crate::domain::models::Trend;
use crate::services::iss_service::IssService;

/// Gets the most recent ISS log.
pub async fn last_iss(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let service = IssService::new(&state);
    let log_opt = service.get_last_iss().await.map_err(ApiError::from)?;

    match log_opt {
        Some(log) => Ok(Json(log)),
        None => Ok(Json(serde_json::json!({ "message": "no data" }))),
    }
}

/// Triggers a new fetch of ISS data and returns the latest log.
pub async fn trigger_iss(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let service = IssService::new(&state);
    service.fetch_and_store_iss().await.map_err(ApiError::from)?;
    // Call the other handler to avoid duplicating logic
    last_iss(State(state)).await
}

/// Calculates and returns the movement trend of the ISS.
pub async fn iss_trend(State(state): State<AppState>) -> Result<Json<Trend>, ApiError> {
    let service = IssService::new(&state);
    let trend = service.get_trend().await.map_err(ApiError::from)?;
    Ok(Json(trend))
}
