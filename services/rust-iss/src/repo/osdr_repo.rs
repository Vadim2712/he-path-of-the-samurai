use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};

/// Repository for managing OSDR items in the database.
#[derive(Clone)]
pub struct OsdrRepo {
    pool: PgPool,
}

impl OsdrRepo {
    /// Creates a new OsdrRepo.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Lists the most recent OSDR items.
    pub async fn list(&self, limit: i64) -> Result<Vec<Value>> {
        let rows = sqlx::query(
            "SELECT id, dataset_id, title, status, updated_at, inserted_at, raw
             FROM osdr_items
             ORDER BY inserted_at DESC
             LIMIT $1"
        ).bind(limit).fetch_all(&self.pool).await?;

        let out: Vec<Value> = rows.into_iter().map(|r| {
            serde_json::json!({
                "id": r.get::<i64,_>("id"),
                "dataset_id": r.get::<Option<String>,_>("dataset_id"),
                "title": r.get::<Option<String>,_>("title"),
                "status": r.get::<Option<String>,_>("status"),
                "updated_at": r.get::<Option<DateTime<Utc>>,_>("updated_at"),
                "inserted_at": r.get::<DateTime<Utc>, _>("inserted_at"),
                "raw": r.get::<Value,_>("raw"),
            })
        }).collect();
        Ok(out)
    }

    /// Gets the total count of items in the osdr_items table.
    pub async fn count(&self) -> Result<i64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM osdr_items")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    /// Inserts a new item, or updates it if the dataset_id already exists.
    pub async fn upsert(
        &self,
        dataset_id: &str,
        title: Option<&str>,
        status: Option<&str>,
        updated_at: Option<DateTime<Utc>>,
        raw: &Value,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
             VALUES($1, $2, $3, $4, $5)
             ON CONFLICT (dataset_id) DO UPDATE
             SET title = EXCLUDED.title,
                 status = EXCLUDED.status,
                 updated_at = EXCLUDED.updated_at,
                 raw = EXCLUDED.raw,
                 inserted_at = NOW()"
        )
        .bind(dataset_id)
        .bind(title)
        .bind(status)
        .bind(updated_at)
        .bind(raw)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}