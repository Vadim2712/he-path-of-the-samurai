use anyhow::Result;
use serde_json::Value;
use tracing::error;

use crate::domain::utils::{s_pick, t_pick};
use crate::domain::validation::OsdrItemValidation;
use crate::repo::osdr_repo::OsdrRepo;

/// Service for handling OSDR data fetching and processing.
#[derive(Clone)]
pub struct OsdrService {
    repo: OsdrRepo,
    client: reqwest::Client,
    nasa_url: String,
}

impl OsdrService {
    pub fn new(repo: OsdrRepo, nasa_url: String) -> Self {
        Self {
            repo,
            client: reqwest::Client::new(),
            nasa_url,
        }
    }

    pub async fn fetch_and_store_osdr(&self) -> Result<usize> {
        let resp = self.client.get(&self.nasa_url)
            .timeout(std::time::Duration::from_secs(45))
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("OSDR request failed with status {}", resp.status());
        }

        let json: Value = resp.json().await?;
        let items = if let Some(a) = json.as_array() {
            a.clone()
        } else if let Some(v) = json.get("items").and_then(|x| x.as_array()) {
            v.clone()
        } else if let Some(v) = json.get("results").and_then(|x| x.as_array()) {
            v.clone()
        } else {
            vec![json]
        };

        let mut written_count = 0;
        for item in items {
            // Validate the item before processing
            if let Err(e) = serde_json::from_value::<OsdrItemValidation>(item.clone()) {
                error!("Failed to deserialize or validate OSDR item: {:?}. Item: {:?}", e, item);
                continue;
            }

            let dataset_id = s_pick(&item, &["dataset_id", "id", "uuid", "studyId", "accession", "osdr_id"]);
            let title = s_pick(&item, &["title", "name", "label"]);
            let status = s_pick(&item, &["status", "state", "lifecycle"]);
            let updated_at = t_pick(&item, &["updated", "updated_at", "modified", "lastUpdated", "timestamp"]);

            if let Some(id) = dataset_id {
                self.repo.upsert(&id, title.as_deref(), status.as_deref(), updated_at, &item).await?;
                written_count += 1;
            }
        }
        Ok(written_count)
    }
}
