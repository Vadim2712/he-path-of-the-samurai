use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;

use crate::domain::models::AppState;
use crate::repo::{cache_repo::CacheRepo, iss_repo::IssRepo, osdr_repo::OsdrRepo};
use crate::services::{
    iss_service::IssService, job_service::JobService, osdr_service::OsdrService,
    space_service::SpaceService,
};

pub async fn new(pool: PgPool) -> AppState {
    dotenvy::dotenv().ok();

    // Redis Pool
    let redis_url = env_str("REDIS_URL", "redis://redis_cache:6379/");
    let manager = RedisConnectionManager::new(redis_url).expect("Failed to create Redis manager");
    let redis_pool = Pool::builder()
        .build(manager)
        .expect("Failed to build Redis pool");

    // Repositories
    let iss_repo = IssRepo::new(pool.clone());
    let osdr_repo = OsdrRepo::new(pool.clone());
    let cache_repo = CacheRepo::new(redis_pool);

    // Config variables
    let nasa_url = env_str("NASA_API_URL", "https://visualization.osdr.nasa.gov/biodata/api/v2/datasets/?format=json");
    let nasa_key = env_str("NASA_API_KEY", "DEMO_KEY");
    let apod_url = env_str("APOD_API_URL", "https://api.nasa.gov/planetary/apod");
    let neo_url = env_str("NEO_API_URL", "https://api.nasa.gov/neo/rest/v1/feed");
    let donki_flr_url = env_str("DONKI_FLR_API_URL", "https://api.nasa.gov/DONKI/FLR");
    let donki_cme_url = env_str("DONKI_CME_API_URL", "https://api.nasa.gov/DONKI/CME");
    let iss_url = env_str("ISS_API_URL", "https://api.wheretheiss.at/v1/satellites/25544");
    let spacex_next_url = env_str("SPACEX_NEXT_API_URL", "https://api.spacexdata.com/v4/launches/next");
    let jwst_api_url = env_str("JWST_API_URL", "");
    let jwst_api_key = env_str("JWST_API_KEY", "");
    let astro_api_url = env_str("ASTRONOMY_API_URL", "https://api.astronomyapi.com/api/v2");
    let astro_api_id = env_str("ASTRONOMY_API_ID", "");
    let astro_api_secret = env_str("ASTRONOMY_API_SECRET", "");

    let every_osdr = env_u64("FETCH_EVERY_SECONDS", 600);
    let every_iss = env_u64("ISS_EVERY_SECONDS", 120);
    let every_apod = env_u64("APOD_EVERY_SECONDS", 43200); // 12ч
    let every_neo = env_u64("NEO_EVERY_SECONDS", 7200); // 2ч
    let every_donki = env_u64("DONKI_EVERY_SECONDS", 3600); // 1ч
    let every_spacex = env_u64("SPACEX_EVERY_SECONDS", 3600);
    let rate_limit_seconds = env_u64("RATE_LIMIT_SECONDS", 1);

    // Services
    let iss_service = IssService::new(iss_repo.clone(), iss_url.clone());
    let osdr_service = OsdrService::new(osdr_repo.clone(), nasa_url.clone());
    let space_service = SpaceService::new(
        cache_repo.clone(),
        nasa_key.clone(),
        apod_url.clone(),
        neo_url.clone(),
        donki_flr_url.clone(),
        donki_cme_url.clone(),
        spacex_next_url.clone(),
    );

    let job_service = JobService::new(
        Arc::new(iss_service.clone()),
        Arc::new(osdr_service.clone()),
        Arc::new(space_service.clone()),
        every_iss,
        every_osdr,
        every_apod,
        every_neo,
        every_donki,
        every_spacex,
    );

    AppState {
        pool,
        iss_repo,
        osdr_repo,
        cache_repo,
        iss_service,
        osdr_service,
        space_service,
        job_service,
        nasa_url,
        nasa_key,
        iss_url,
        apod_url,
        neo_url,
        donki_flr_url,
        donki_cme_url,
        spacex_next_url,
        jwst_api_url,
        jwst_api_key,
        astro_api_url,
        astro_api_id,
        astro_api_secret,
        every_osdr,
        every_iss,
        every_apod,
        every_neo,
        every_donki,
        every_spacex,
        rate_limit_seconds,
    }
}

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
}

fn env_str(k: &str, d: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| d.to_string())
}
