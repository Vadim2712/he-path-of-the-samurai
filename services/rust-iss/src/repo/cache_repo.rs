use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};

pub async fn get_latest_space_cache(pool: &PgPool, src: &str) -> Result<Option<Value>> {
    let row = sqlx::query(
        "SELECT fetched_at, payload FROM space_cache
         WHERE source = $1 ORDER BY id DESC LIMIT 1"
    ).bind(src).fetch_optional(pool).await?;

    if let Some(r) = row {
        let fetched_at: DateTime<Utc> = r.get("fetched_at");
        let payload: Value = r.get("payload");
        Ok(Some(serde_json::json!({ "source": src, "fetched_at": fetched_at, "payload": payload })))
    } else {
        Ok(None)
    }
}

pub async fn get_latest_from_cache(pool: &PgPool, src: &str) -> Value {
    sqlx::query("SELECT fetched_at, payload FROM space_cache WHERE source=$1 ORDER BY id DESC LIMIT 1")
        .bind(src)
        .fetch_optional(pool).await.ok().flatten()
        .map(|r| serde_json::json!({"at": r.get::<DateTime<Utc>,_>("fetched_at"), "payload": r.get::<Value,_>("payload")}))
        .unwrap_or(serde_json::json!({}))
}

pub async fn get_osdr_count(pool: &PgPool) -> Result<i64> {
    let osdr_count: i64 = sqlx::query("SELECT count(*) AS c FROM osdr_items")
        .fetch_one(pool).await.map(|r| r.get::<i64,_>("c"))?;
    Ok(osdr_count)
}
