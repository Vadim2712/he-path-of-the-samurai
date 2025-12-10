use std::collections::HashMap;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::{json, Value};
use crate::domain::models::AppState;
use crate::domain::error::ApiError;
use crate::services::space_service::SpaceService;

/// Handler to get the latest cached data for a specific source.
pub async fn space_latest(
    Path(src): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    match state.cache_repo.get_latest(&src) {
        Ok(value) => Ok(Json(value)),
        Err(_) => Ok(Json(json!({ "source": src, "message": "no data found in cache" }))),
    }
}

/// Handler to trigger a refresh of cached data for specified sources.
pub async fn space_refresh(
    Query(q): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let service = SpaceService::new(state);
    let sources_query = q.get("src").cloned().unwrap_or_else(|| "apod,neo,donki,spacex".to_string());
    
    let mut refreshed = Vec::new();
    let sources_to_refresh = sources_query.split(',').map(|s| s.trim().to_lowercase());

    for source in sources_to_refresh {
        let result = match source.as_str() {
            "apod" => service.fetch_apod().await,
            "neo" => service.fetch_neo().await,
            "donki" | "flr" | "cme" => service.fetch_donki().await,
            "spacex" => service.fetch_spacex_next().await,
            _ => continue,
        };

        if result.is_ok() {
            refreshed.push(source);
        }
    }

    Ok(Json(json!({ "refreshed_sources": refreshed })))
}

/// Handler to get a summary of all cached space data.
pub async fn space_summary(
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let apod_val = state.cache_repo.get_latest("apod").unwrap_or_default();
    let neo_val = state.cache_repo.get_latest("neo").unwrap_or_default();
    let flr_val = state.cache_repo.get_latest("flr").unwrap_or_default();
    let cme_val = state.cache_repo.get_latest("cme").unwrap_or_default();
    let spacex_val = state.cache_repo.get_latest("spacex").unwrap_or_default();

    let iss_val = state.iss_repo.get_last().await.map_err(ApiError::from)?;
    let osdr_count = state.osdr_repo.count().await.map_err(ApiError::from)?;

    Ok(Json(json!({
        "apod": apod_val,
        "neo": neo_val,
        "flr": flr_val,
        "cme": cme_val,
        "spacex": spacex_val,
        "iss": iss_val,
        "osdr_count": osdr_count,
    })))
}
