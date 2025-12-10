use r2d2_redis::redis::Commands;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::RedisResult;
use serde_json::Value;

/// Repository for accessing the Redis cache.
#[derive(Clone)]
pub struct CacheRepo {
    pool: Pool<RedisConnectionManager>,
}

impl CacheRepo {
    /// Creates a new CacheRepo with a connection pool.
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Self { pool }
    }

    /// Saves a JSON value to the cache with a given key.
    pub fn save(&self, key: &str, value: &Value) -> RedisResult<()> {
        let mut conn = self.pool.get().map_err(|e| redis::RedisError::from((redis::ErrorKind::IoError, "Pool Error", e.to_string())))?;
        let json_string = serde_json::to_string(value).map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "JSON Serialize Error", e.to_string())))?;
        
        // Manually map the error to fix the type conflict
        conn.set(key, json_string).map_err(|e| redis::RedisError::from((redis::ErrorKind::ResponseError, "Redis SET failed", e.to_string())))
    }

    /// Retrieves a JSON value from the cache by key.
    pub fn get_latest(&self, key: &str) -> RedisResult<Value> {
        let mut conn = self.pool.get().map_err(|e| redis::RedisError::from((redis::ErrorKind::IoError, "Pool Error", e.to_string())))?;

        // Manually map the error to fix the type conflict
        let result: String = conn.get(key).map_err(|e| redis::RedisError::from((redis::ErrorKind::ResponseError, "Redis GET failed", e.to_string())))?;
        
        serde_json::from_str(&result).map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "JSON Parse Error", e.to_string())))
    }
}