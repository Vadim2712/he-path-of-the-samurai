use axum::{
    extract::State,
    Json,
};
use serde_json::Value;
use tracing::info;

use crate::domain::models::AppState;
use crate::domain::error::ApiError;
use crate::services::osdr_service::OsdrService;

/// Asynchronously triggers a sync of the OSDR data.
pub async fn osdr_sync(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        let service = OsdrService::new(&state);
        info!("Starting background OSDR sync...");
        if let Ok(count) = service.fetch_and_store_osdr().await {
            info!("OSDR background sync completed. Upserted {} records.", count);
        } else {
            info!("OSDR background sync failed.");
        }
    });

    Ok(Json(serde_json::json!({ "message": "OSDR sync triggered in background." })))
}

/// Lists the most recent OSDR items.
pub async fn osdr_list(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let limit = std::env::var("OSDR_LIST_LIMIT")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(20);

    let items = state.osdr_repo.list(limit).await.map_err(ApiError::from)?;
    Ok(Json(serde_json::json!({ "items": items })))
}