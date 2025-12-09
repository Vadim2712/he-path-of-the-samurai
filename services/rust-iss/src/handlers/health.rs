use axum::{
    http::StatusCode,
    Json,
};
use crate::domain::error::ApiError;
use crate::domain::models::Health;

pub async fn health_check() -> Result<Json<Health>, ApiError> {
    Ok(Json(Health { status: "ok", now: Utc::now() }))
}