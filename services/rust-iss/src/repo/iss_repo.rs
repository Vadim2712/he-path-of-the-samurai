use anyhow::Result;
use serde_json::Value;
use sqlx::PgPool;
use crate::domain::models::IssFetchLog;

/// Repository for managing ISS fetch logs in the database.
#[derive(Clone)]
pub struct IssRepo {
    pool: PgPool,
}

impl IssRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new log entry for an ISS fetch.
    pub async fn create_log(&self, url: &str, payload: &Value) -> Result<()> {
        sqlx::query("INSERT INTO iss_fetch_log (source_url, payload) VALUES ($1, $2)")
            .bind(url)
            .bind(payload)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Gets the most recent ISS log entry.
    pub async fn get_last(&self) -> Result<Option<Value>> {
        let result: Option<IssFetchLog> = sqlx::query_as("SELECT * FROM iss_fetch_log ORDER BY id DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await?;
        Ok(result.map(|log| serde_json::to_value(log).unwrap_or_default()))
    }

    /// Gets the last two ISS log entries for trend calculation.
    pub async fn get_last_two(&self) -> Result<Vec<IssFetchLog>> {
        let logs: Vec<IssFetchLog> = sqlx::query_as("SELECT * FROM iss_fetch_log ORDER BY id DESC LIMIT 2")
            .fetch_all(&self.pool)
            .await?;
        Ok(logs)
    }
}
