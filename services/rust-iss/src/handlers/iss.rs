use axum::{
    extract::State,
    Json,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::Row;

use crate::config::AppState;
use crate::services::iss_service::{num, haversine_km};
use crate::clients::iss::fetch_and_store_iss;
use crate::repo::iss_repo::{get_last_iss, get_iss_trend_data};
use crate::domain::error::ApiError;
use crate::domain::models::Trend;

pub async fn last_iss(State(st): State<AppState>)
-> Result<Json<Value>, ApiError> {
    let row_opt = get_last_iss(&st.pool).await
     .map_err(ApiError::from)?;

    if let Some(payload) = row_opt {
        return Ok(Json(payload));
    }
    Ok(Json(serde_json::json!({"message":"no data"})))
}

pub async fn trigger_iss(State(st): State<AppState>)
-> Result<Json<Value>, ApiError> {
    fetch_and_store_iss(&st.pool, &st.fallback_url).await
        .map_err(ApiError::from)?;
    last_iss(State(st)).await
}

pub async fn iss_trend(State(st): State<AppState>)
-> Result<Json<Trend>, ApiError> {
    let rows = get_iss_trend_data(&st.pool).await
        .map_err(ApiError::from)?;

    if rows.len() < 2 {
        return Ok(Json(Trend {
            movement: false, delta_km: 0.0, dt_sec: 0.0, velocity_kmh: None,
            from_time: None, to_time: None,
            from_lat: None, from_lon: None, to_lat: None, to_lon: None
        }));
    }

    let t2: DateTime<Utc> = rows[0].get("fetched_at");
    let t1: DateTime<Utc> = rows[1].get("fetched_at");
    let p2: Value = rows[0].get("payload");
    let p1: Value = rows[1].get("payload");

    let lat1 = num(&p1["latitude"]);
    let lon1 = num(&p1["longitude"]);
    let lat2 = num(&p2["latitude"]);
    let lon2 = num(&p2["longitude"]);
    let v2 = num(&p2["velocity"]);

    let mut delta_km = 0.0;
    let mut movement = false;
    if let (Some(a1), Some(o1), Some(a2), Some(o2)) = (lat1, lon1, lat2, lon2) {
        delta_km = haversine_km(a1, o1, a2, o2);
        movement = delta_km > 0.1;
    }
    let dt_sec = (t2 - t1).num_milliseconds() as f64 / 1000.0;

    Ok(Json(Trend {
        movement,
        delta_km,
        dt_sec,
        velocity_kmh: v2,
        from_time: Some(t1),
        to_time: Some(t2),
        from_lat: lat1, from_lon: lon1, to_lat: lat2, to_lon: lon2,
    }))
}
