use anyhow::Result;
use serde_json::Value;
use crate::domain::models::AppState;
use crate::domain::error::ApiError;
use crate::domain::models::Trend;
use crate::repo::iss_repo::IssRepo;

/// Service for handling ISS-related business logic.
#[derive(Clone)]
pub struct IssService {
    repo: IssRepo,
    client: reqwest::Client,
    iss_url: String,
}

impl IssService {
    pub fn new(state: &AppState) -> Self {
        Self {
            repo: state.iss_repo.clone(),
            client: reqwest::Client::new(),
            iss_url: state.iss_url.clone(),
        }
    }

    /// Fetches the current ISS position and stores it in the database.
    pub async fn fetch_and_store_iss(&self) -> Result<()> {
        let resp = self.client.get(&self.iss_url)
            .timeout(std::time::Duration::from_secs(20))
            .send()
            .await?;
        
        let json: Value = resp.json().await?;
        self.repo.create_log(&self.iss_url, &json).await?;
        Ok(())
    }

    /// Gets the most recent ISS log from the database.
    pub async fn get_last_iss(&self) -> Result<Option<Value>> {
        self.repo.get_last().await
    }
    
    /// Calculates the trend of ISS movement based on the last two data points.
    pub async fn get_trend(&self) -> Result<Trend> {
        let rows = self.repo.get_last_two().await?;

        if rows.len() < 2 {
            return Ok(Trend {
                movement: false, delta_km: 0.0, dt_sec: 0.0, velocity_kmh: None,
                from_time: None, to_time: None, from_lat: None,
                from_lon: None, to_lat: None, to_lon: None,
            });
        }

        let t2 = rows[0].fetched_at;
        let t1 = rows[1].fetched_at;
        let p2 = &rows[0].payload;
        let p1 = &rows[1].payload;

        let lat1 = Self::num_from_json(&p1["latitude"]);
        let lon1 = Self::num_from_json(&p1["longitude"]);
        let lat2 = Self::num_from_json(&p2["latitude"]);
        let lon2 = Self::num_from_json(&p2["longitude"]);
        let v2 = Self::num_from_json(&p2["velocity"]);

        let mut delta_km = 0.0;
        let mut movement = false;
        if let (Some(lat1), Some(lon1), Some(lat2), Some(lon2)) = (lat1, lon1, lat2, lon2) {
            delta_km = Self::haversine_km(lat1, lon1, lat2, lon2);
            movement = delta_km > 0.1;
        }
        
        let dt_sec = (t2 - t1).num_milliseconds() as f64 / 1000.0;

        Ok(Trend {
            movement, delta_km, dt_sec, velocity_kmh: v2,
            from_time: Some(t1), to_time: Some(t2),
            from_lat: lat1, from_lon: lon1, to_lat: lat2, to_lon: lon2,
        })
    }

    // --- Private Helper Functions ---

    fn num_from_json(v: &Value) -> Option<f64> {
        v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))
    }

    fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        const R: f64 = 6371.0; // Earth radius in kilometers
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin() * (d_lon / 2.0).sin();
        2.0 * R * a.sqrt().asin()
    }
}
