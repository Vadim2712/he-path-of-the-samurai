use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub nasa_url: String,
    pub nasa_key: String,
    pub iss_url: String,
    pub apod_url: String,
    pub neo_url: String,
    pub donki_flr_url: String,
    pub donki_cme_url: String,
    pub spacex_next_url: String,

    pub jwst_api_url: String,
    pub jwst_api_key: String,
    pub astro_api_url: String,
    pub astro_api_id: String,
    pub astro_api_secret: String,

    pub every_osdr: u64,
    pub every_iss: u64,
    pub every_apod: u64,
    pub every_neo: u64,
    pub every_donki: u64,
    pub every_spacex: u64,
    pub rate_limit_seconds: u64,
}

#[derive(Serialize)]
pub struct Health { status: &'static str, now: DateTime<Utc> }

#[derive(Serialize)]
pub struct Trend {
    pub movement: bool,
    pub delta_km: f64,
    pub dt_sec: f64,
    pub velocity_kmh: Option<f64>,
    pub from_time: Option<DateTime<Utc>>,
    pub to_time: Option<DateTime<Utc>>,
    pub from_lat: Option<f64>,
    pub from_lon: Option<f64>,
    pub to_lat: Option<f64>,
    pub to_lon: Option<f64>,
}