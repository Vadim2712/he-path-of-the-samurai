use anyhow::Result;
use serde_json::Value;
use sqlx::PgPool;

pub async fn write_cache(pool: &PgPool, source: &str, payload: Value) -> Result<()> {
    sqlx::query("INSERT INTO space_cache(source, payload) VALUES ($1,$2)")
        .bind(source).bind(payload).execute(pool).await?;
    Ok(())
}
