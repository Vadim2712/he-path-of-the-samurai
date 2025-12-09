use anyhow::Result;
use serde_json::Value;
use sqlx::PgPool;
use tracing::error; // Import error for logging validation failures

use crate::config::AppState;
use crate::clients::nasa::{s_pick, t_pick};
use crate::domain::validation::OsdrItemValidation; // Import the validation struct

pub async fn fetch_and_store_osdr(st: &AppState) -> Result<usize> {
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let resp = client.get(&st.nasa_url).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("OSDR request status {}", resp.status());
    }
    let json: Value = resp.json().await?;
    let items = if let Some(a) = json.as_array() { a.clone() }
        else if let Some(v) = json.get("items").and_then(|x| x.as_array()) { v.clone() }
        else if let Some(v) = json.get("results").and_then(|x| x.as_array()) { v.clone() }
        else { vec![json.clone()] };

    let mut written = 0usize;
    for item in items {
        // Validate the item before processing
        let validation_result = serde_json::from_value::<OsdrItemValidation>(item.clone());
        match validation_result {
            Ok(validated_item) => {
                if let Err(e) = validated_item.validate() {
                    error!("OSDR item validation failed: {:?} for item: {:?}", e, item);
                    continue; // Skip invalid item
                }
            },
            Err(e) => {
                error!("Failed to deserialize OSDR item for validation: {:?} for item: {:?}", e, item);
                continue; // Skip item if deserialization fails
            }
        }

        let id = s_pick(&item, &["dataset_id","id","uuid","studyId","accession","osdr_id"]);
        let title = s_pick(&item, &["title","name","label"]);
        let status = s_pick(&item, &["status","state","lifecycle"]);
        let updated = t_pick(&item, &["updated","updated_at","modified","lastUpdated","timestamp"]);
        if let Some(ds) = id.clone() {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)
                 ON CONFLICT (dataset_id) DO UPDATE
                 SET title=EXCLUDED.title, status=EXCLUDED.status,
                     updated_at=EXCLUDED.updated_at, raw=EXCLUDED.raw"
            ).bind(ds).bind(title).bind(status).bind(updated).bind(item).execute(&st.pool).await?;
        } else {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)"
            ).bind::<Option<String>>(None).bind(title).bind(status).bind(updated).bind(item).execute(&st.pool).await?;
        }
        written += 1;
    }
    Ok(written)
}