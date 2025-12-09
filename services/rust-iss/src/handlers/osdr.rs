use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::Value;

use crate::config::AppState;
use crate::clients::osdr::fetch_and_store_osdr;
use crate::repo::osdr_repo::get_osdr_items;
use crate::domain::error::ApiError;

pub async fn osdr_sync(State(st): State<AppState>)
-> Result<Json<Value>, ApiError> {
    let written = fetch_and_store_osdr(&st).await
        .map_err(ApiError::from)?;
    Ok(Json(serde_json::json!({ "written": written })))
}

pub async fn osdr_list(State(st): State<AppState>)
-> Result<Json<Value>, ApiError> {
    let limit =  std::env::var("OSDR_LIST_LIMIT").ok()
        .and_then(|s| s.parse::<i64>().ok()).unwrap_or(20);

    let out = get_osdr_items(&st.pool, limit).await
     .map_err(ApiError::from)?;
}