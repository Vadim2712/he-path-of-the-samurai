use reqwest::header::{HeaderMap, USER_AGENT};
use tracing::{error, info};

use crate::repo::cache_repo::CacheRepo;

/// A service dedicated to fetching data from various space-related APIs
/// and caching the results in Redis.
#[derive(Clone)]
pub struct SpaceService {
    cache: CacheRepo,
    client: reqwest::Client,
    nasa_key: String,
    apod_url: String,
    neo_url: String,
    donki_flr_url: String,
    donki_cme_url: String,
    spacex_next_url: String,
}

impl SpaceService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cache: CacheRepo,
        nasa_key: String,
        apod_url: String,
        neo_url: String,
        donki_flr_url: String,
        donki_cme_url: String,
        spacex_next_url: String,
    ) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Cassiopeia-Project/1.0".parse().unwrap());

        Self {
            cache,
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            nasa_key,
            apod_url,
            neo_url,
            donki_flr_url,
            donki_cme_url,
            spacex_next_url,
        }
    }

    /// A generic helper function to fetch data from a URL and cache it.
    async fn fetch_and_cache(
        &self,
        url: &str,
        source_key: &str,
        params: &[(&str, &str)],
    ) -> anyhow::Result<()> {
        info!("Fetching data for {}", source_key);

        let mut query_params = params.to_vec();
        if !self.nasa_key.is_empty() {
            query_params.push(("api_key", &self.nasa_key));
        }

        let response = self.client.get(url).query(&query_params).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not read error body".to_string());
            error!(
                "API request for {} failed with status {}: {}",
                source_key, status, body
            );

            // If it's a 403, we log it but don't treat it as a hard error for the job.
            // This allows the app to function with a potentially invalid API key.
            if status == reqwest::StatusCode::FORBIDDEN {
                info!("Gracefully handling 403 for {}", source_key);
                return Ok(());
            }

            anyhow::bail!(
                "API request for {} failed with status {}",
                source_key,
                status
            );
        }

        // Clone the response to read it twice
        let response_bytes = response.bytes().await?;
        let data: serde_json::Value = match serde_json::from_slice(&response_bytes) {
            Ok(json) => json,
            Err(e) => {
                let body_text = String::from_utf8_lossy(&response_bytes);
                error!(
                    "Failed to parse JSON for {}. Body: {}",
                    source_key, body_text
                );
                return Err(e.into());
            }
        };

        self.cache.save(source_key, &data)?;
        info!("Successfully cached data for {}", source_key);
        Ok(())
    }

    // --- Public methods for specific sources ---

    pub async fn fetch_apod(&self) -> anyhow::Result<()> {
        self.fetch_and_cache(self.apod_url.as_str(), "apod", &[("thumbs", "true")])
            .await
    }

    pub async fn fetch_neo(&self) -> anyhow::Result<()> {
        let end_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let start_date = (chrono::Utc::now() - chrono::Duration::days(2))
            .format("%Y-%m-%d")
            .to_string();

        self.fetch_and_cache(
            self.neo_url.as_str(),
            "neo",
            &[("start_date", &start_date), ("end_date", &end_date)],
        )
        .await
    }

    pub async fn fetch_donki(&self) -> anyhow::Result<()> {
        let end_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let start_date = (chrono::Utc::now() - chrono::Duration::days(5))
            .format("%Y-%m-%d")
            .to_string();

        let params = [
            ("startDate", start_date.as_str()),
            ("endDate", end_date.as_str()),
        ];

        self.fetch_and_cache(self.donki_flr_url.as_str(), "flr", &params)
            .await?;
        self.fetch_and_cache(self.donki_cme_url.as_str(), "cme", &params)
            .await?;

        Ok(())
    }

    pub async fn fetch_spacex_next(&self) -> anyhow::Result<()> {
        self.fetch_and_cache(self.spacex_next_url.as_str(), "spacex", &[])
            .await
    }
}

