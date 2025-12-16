use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, PgPool};

use crate::repo::{cache_repo::CacheRepo, iss_repo::IssRepo, osdr_repo::OsdrRepo};
use crate::services::{
    iss_service::IssService, job_service::JobService, osdr_service::OsdrService,
    space_service::SpaceService,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub iss_repo: IssRepo,
    pub osdr_repo: OsdrRepo,
    pub cache_repo: CacheRepo,

    pub iss_service: IssService,
    pub osdr_service: OsdrService,
    pub space_service: SpaceService,
    pub job_service: JobService,
    
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
pub struct Health { pub status: &'static str, pub now: DateTime<Utc> }

#[derive(Serialize, FromRow, Debug)]
pub struct IssFetchLog {
    pub id: i64,
    pub fetched_at: DateTime<Utc>,
    pub source_url: String,
    pub payload: Value,
}


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