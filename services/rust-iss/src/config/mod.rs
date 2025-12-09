use sqlx::PgPool;

use crate::domain::models::AppState;

pub async fn new(pool: PgPool) -> AppState {
    dotenvy::dotenv().ok();

    // NASA
    let nasa_url = env_str("NASA_API_URL", "https://visualization.osdr.nasa.gov/biodata/api/v2/datasets/?format=json");
    let nasa_key = env_str("NASA_API_KEY", "");
    let apod_url = env_str("APOD_API_URL", "https://api.nasa.gov/planetary/apod");
    let neo_url = env_str("NEO_API_URL", "https://api.nasa.gov/neo/rest/v1/feed");
    let donki_flr_url = env_str("DONKI_FLR_API_URL", "https://api.nasa.gov/DONKI/FLR");
    let donki_cme_url = env_str("DONKI_CME_API_URL", "https://api.nasa.gov/DONKI/CME");
    // ISS
    let iss_url = env_str("ISS_API_URL", "https://api.wheretheiss.at/v1/satellites/25544");
    // SpaceX
    let spacex_next_url = env_str("SPACEX_NEXT_API_URL", "https://api.spacexdata.com/v4/launches/next");
    // JWST (placeholder)
    let jwst_api_url = env_str("JWST_API_URL", "");
    let jwst_api_key = env_str("JWST_API_KEY", "");
    // AstronomyAPI (placeholder)
    let astro_api_url = env_str("ASTRONOMY_API_URL", "https://api.astronomyapi.com/api/v2");
    let astro_api_id = env_str("ASTRONOMY_API_ID", "");
    let astro_api_secret = env_str("ASTRONOMY_API_SECRET", "");


    let every_osdr   = env_u64("FETCH_EVERY_SECONDS", 600);
    let every_iss    = env_u64("ISS_EVERY_SECONDS",   120);
    let every_apod   = env_u64("APOD_EVERY_SECONDS",  43200); // 12ч
    let every_neo    = env_u64("NEO_EVERY_SECONDS",   7200);  // 2ч
    let every_donki  = env_u64("DONKI_EVERY_SECONDS", 3600);  // 1ч
    let every_spacex = env_u64("SPACEX_EVERY_SECONDS",3600);
    let rate_limit_seconds = env_u64("RATE_LIMIT_SECONDS", 1); // Default to 1 second

    AppState {
        pool,
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
        every_osdr, every_iss, every_apod, every_neo, every_donki, every_spacex,
        rate_limit_seconds,
    }
}

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
}

fn env_str(k: &str, d: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| d.to_string())
}
