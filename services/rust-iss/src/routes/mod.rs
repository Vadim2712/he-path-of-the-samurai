use axum::{
    routing::get,
    Router,
};

use crate::config::AppState;
use crate::handlers::{health, iss, osdr, space};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        // ISS
        .route("/last", get(iss::last_iss))
        .route("/fetch", get(iss::trigger_iss))
        .route("/iss/trend", get(iss::iss_trend))
        // OSDR
        .route("/osdr/sync", get(osdr::osdr_sync))
        .route("/osdr/list", get(osdr::osdr_list))
        // Space cache
        .route("/space/:src/latest", get(space::space_latest))
        .route("/space/refresh", get(space::space_refresh))
        .route("/space/summary", get(space::space_summary))
        .with_state(state)
}