use anyhow::Result;
use chrono::{DateTime, Utc, NaiveDateTime};
use serde_json::Value;
use sqlx::PgPool;

use crate::config::AppState;
use crate::write_cache;

// APOD
pub async fn fetch_apod(st: &AppState) -> Result<()> {
    let url = &st.apod_url;
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let mut req = client.get(url).query(&[("thumbs","true")]);
    if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
    let json: Value = req.send().await?.json().await?;
    write_cache(&st.pool, "apod", json).await
}

// NeoWs
pub async fn fetch_neo_feed(st: &AppState) -> Result<()> {
    let today = Utc::now().date_naive();
    let start = today - chrono::Days::new(2);
    let url = &st.neo_url;
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let mut req = client.get(url).query(&[
        ("start_date", start.to_string()),
        ("end_date", today.to_string()),
    ]);
    if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
    let json: Value = req.send().await?.json().await?;
    write_cache(&st.pool, "neo", json).await
}

// DONKI объединённая
pub async fn fetch_donki(st: &AppState) -> Result<()> {
    let _ = fetch_donki_flr(st).await;
    let _ = fetch_donki_cme(st).await;
    Ok(())
}
pub async fn fetch_donki_flr(st: &AppState) -> Result<()> {
    let (from,to) = last_days(5);
    let url = &st.donki_flr_url;
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let mut req = client.get(url).query(&[("startDate",from),("endDate",to)]);
    if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
    let json: Value = req.send().await?.json().await?;
    write_cache(&st.pool, "flr", json).await
}
pub async fn fetch_donki_cme(st: &AppState) -> Result<()> {
    let (from,to) = last_days(5);
    let url = &st.donki_cme_url;
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let mut req = client.get(url).query(&[("startDate",from),("endDate",to)]);
    if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
    let json: Value = req.send().await?.json().await?;
    write_cache(&st.pool, "cme", json).await
}

pub fn last_days(n: i64) -> (String,String) {
    let to = Utc::now().date_naive();
    let from = to - chrono::Days::new(n as u64);
    (from.to_string(), to.to_string())
}

pub fn s_pick(v: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() { if !s.is_empty() { return Some(s.to_string()); } }
            else if x.is_number() { return Some(x.to_string()); }
        }
    }
    None
}
pub fn t_pick(v: &Value, keys: &[&str]) -> Option<DateTime<Utc>> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() {
                if let Ok(dt) = s.parse::<DateTime<Utc>>() { return Some(dt); }
                if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                    return Some(Utc.from_utc_datetime(&ndt));
                }
            } else if let Some(n) = x.as_i64() {
                return Some(Utc.timestamp_opt(n, 0).single().unwrap_or_else(Utc::now));
            }
        }
    }
    None
}