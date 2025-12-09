use std::num::NonZeroU32;
use std::time::Duration;
use axum::{
    routing::get,
    Router,
};
use axum_governor::GovernorLayer;
use axum_governor::governor::GovernorConfigBuilder;


use crate::config::AppState;
use crate::handlers::{health, iss, osdr, space};

pub fn create_router(state: AppState) -> Router {
    // Configure a rate limiter to allow 60 requests per minute,
    // which is a generous but safe default.
    let governor_config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(60)
            .burst_size(NonZeroU32::new(60).unwrap())
            .finish()
            .unwrap(),
    );

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
            config: governor_config,
        })
        .with_state(state)
}