use std::sync::Arc;
use std::time::Duration;

use axum::{routing::get, Router};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use crate::domain::models::AppState;
use crate::handlers::{health, iss, osdr, space};

pub fn create_router(state: AppState) -> Router {
    // Create a rate limiter configuration
    let governor_conf = GovernorConfigBuilder::default()
        .period(Duration::from_secs(state.rate_limit_seconds))
        .burst_size(1) // Allow 1 request per period
        .finish()
        .unwrap();

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
        .layer(GovernorLayer {
            config: Arc::new(governor_conf),
        })
        .with_state(state)
}