use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};

pub async fn latest_from_cache(pool: &PgPool, src: &str) -> Value {
    sqlx::query("SELECT fetched_at, payload FROM space_cache WHERE source=$1 ORDER BY id DESC LIMIT 1")
        .bind(src)
        .fetch_optional(pool).await.ok().flatten()
        .map(|r| serde_json::json!({"at": r.get::<DateTime<Utc>,_>("fetched_at"), "payload": r.get::<Value,_>("payload")}))
        .unwrap_or(serde_json::json!({}))
}