use std::sync::Arc;
use tokio::time::{self, Duration};
use tracing::{error, info};

use crate::services::{
    iss_service::IssService, osdr_service::OsdrService, space_service::SpaceService,
};

/// Service responsible for managing all periodic background jobs.
#[derive(Clone)]
pub struct JobService {
    iss_service: Arc<IssService>,
    osdr_service: Arc<OsdrService>,
    space_service: Arc<SpaceService>,
    every_iss: u64,
    every_osdr: u64,
    every_apod: u64,
    every_neo: u64,
    every_donki: u64,
    every_spacex: u64,
}

impl JobService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        iss_service: Arc<IssService>,
        osdr_service: Arc<OsdrService>,
        space_service: Arc<SpaceService>,
        every_iss: u64,
        every_osdr: u64,
        every_apod: u64,
        every_neo: u64,
        every_donki: u64,
        every_spacex: u64,
    ) -> Self {
        Self {
            iss_service,
            osdr_service,
            space_service,
            every_iss,
            every_osdr,
            every_apod,
            every_neo,
            every_donki,
            every_spacex,
        }
    }

    /// Spawns all background tasks for periodically fetching data.
    pub fn spawn_all_jobs(self: Arc<Self>) {
        info!("Spawning all background jobs...");

        self.spawn_iss_job();
        self.spawn_osdr_job();
        self.spawn_apod_job();
        self.spawn_neo_job();
        self.spawn_donki_job();
        self.spawn_spacex_job();

        info!("All background jobs have been spawned.");
    }

    fn spawn_iss_job(&self) {
        let service = self.iss_service.clone();
        let period = self.every_iss;
        tokio::spawn(async move {
            if period == 0 { return; }
            let mut interval = time::interval(Duration::from_secs(period));
            loop {
                interval.tick().await;
                if let Err(e) = service.fetch_and_store_iss().await {
                    error!("ISS fetch job failed: {:?}", e);
                }
            }
        });
    }

    fn spawn_osdr_job(&self) {
        let service = self.osdr_service.clone();
        let period = self.every_osdr;
        tokio::spawn(async move {
            if period == 0 { return; }
            let mut interval = time::interval(Duration::from_secs(period));
            loop {
                interval.tick().await;
                if let Err(e) = service.fetch_and_store_osdr().await {
                    error!("OSDR fetch job failed: {:?}", e);
                }
            }
        });
    }

    fn spawn_apod_job(&self) {
        let service = self.space_service.clone();
        let period = self.every_apod;
        tokio::spawn(async move {
            if period == 0 { return; }
            let mut interval = time::interval(Duration::from_secs(period));
            loop {
                interval.tick().await;
                if let Err(e) = service.fetch_apod().await {
                    error!("APOD fetch job failed: {:?}", e);
                }
            }
        });
    }

    fn spawn_neo_job(&self) {
        let service = self.space_service.clone();
        let period = self.every_neo;
        tokio::spawn(async move {
            if period == 0 { return; }
            let mut interval = time::interval(Duration::from_secs(period));
            loop {
                interval.tick().await;
                if let Err(e) = service.fetch_neo().await {
                    error!("NEO fetch job failed: {:?}", e);
                }
            }
        });
    }

    fn spawn_donki_job(&self) {
        let service = self.space_service.clone();
        let period = self.every_donki;
        tokio::spawn(async move {
            if period == 0 { return; }
            let mut interval = time::interval(Duration::from_secs(period));
            loop {
                interval.tick().await;
                if let Err(e) = service.fetch_donki().await {
                    error!("DONKI fetch job failed: {:?}", e);
                }
            }
        });
    }

    fn spawn_spacex_job(&self) {
        let service = self.space_service.clone();
        let period = self.every_spacex;
        tokio::spawn(async move {
            if period == 0 { return; }
            let mut interval = time::interval(Duration::from_secs(period));
            loop {
                interval.tick().await;
                if let Err(e) = service.fetch_spacex_next().await {
                    error!("SpaceX fetch job failed: {:?}", e);
                }
            }
        });
    }
}
