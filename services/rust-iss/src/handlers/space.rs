use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::Value;

use crate::config::AppState;
use crate::services::space_service::latest_from_cache;
use crate::repo::cache_repo::{get_latest_space_cache, get_osdr_count};
use crate::clients::{nasa, spacex};
use crate::domain::error::ApiError;

pub async fn space_latest(Path(src): Path<String>, State(st): State<AppState>)
-> Result<Json<Value>, ApiError> {
    let row_opt = get_latest_space_cache(&st.pool, &src).await
     .map_err(ApiError::from)?;

    if let Some(payload) = row_opt {
        return Ok(Json(payload));
    }
    Ok(Json(serde_json::json!({ "source": src, "message":"no data" })))
}

pub async fn space_refresh(Query(q): Query<HashMap<String,String>>, State(st): State<AppState>)
-> Result<Json<Value>, ApiError> {
    let list = q.get("src").cloned().unwrap_or_else(|| "apod,neo,flr,cme,spacex".to_string());
    let mut done = Vec::new();
    for s in list.split(',').map(|x| x.trim().to_lowercase()) {
        match s.as_str() {
            "apod"   => { let _ = nasa::fetch_apod(&st).await?;       done.push("apod"); }
            "neo"    => { let _ = nasa::fetch_neo_feed(&st).await?;   done.push("neo"); }
            "flr"    => { let _ = nasa::fetch_donki_flr(&st).await?;  done.push("flr"); }
            "cme"    => { let _ = nasa::fetch_donki_cme(&st).await?;  done.push("cme"); }
            "spacex" => { let _ = spacex::fetch_spacex_next(&st).await?; done.push("spacex"); }
            _ => {}
        }
    }
    Ok(Json(serde_json::json!({ "refreshed": done })))
}

use crate::clients::{nasa, spacex};

use crate::domain::error::ApiError;

use crate::repo::iss_repo::get_last_iss;



pub async fn space_summary(State(st): State<AppState>)

-> Result<Json<Value>, ApiError> {

    let apod   = latest_from_cache(&st.pool, "apod").await;

    let neo    = latest_from_cache(&st.pool, "neo").await;

    let flr    = latest_from_cache(&st.pool, "flr").await;

    let cme    = latest_from_cache(&st.pool, "cme").await;

    let spacex = latest_from_cache(&st.pool, "spacex").await;



    let iss_last = get_last_iss(&st.pool).await

        .map_err(ApiError::from)?;



    let osdr_count = get_osdr_count(&st.pool).await

     .map_err(ApiError::from)?;
