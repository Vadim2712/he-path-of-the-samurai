use anyhow::Result;
use serde_json::Value;

use crate::config::AppState;
use crate::write_cache;

pub async fn fetch_spacex_next(st: &AppState) -> Result<()> {
    let url = "https://api.spacexdata.com/v4/launches/next";
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let json: Value = client.get(url).send().await?.json().await?;
    write_cache(&st.pool, "spacex", json).await
}
