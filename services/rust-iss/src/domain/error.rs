use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub enum ApiError {
    InternalServerError {
        code: String,
        message: String,
        trace_id: String,
    },
    NotFound {
        code: String,
        message: String,
        trace_id: String,
    },
    // Add other custom errors as needed
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::InternalServerError { .. } => (StatusCode::INTERNAL_SERVER_ERROR, self.message()),
            ApiError::NotFound { .. } => (StatusCode::NOT_FOUND, self.message()),
        };

        // Log the error
        error!("API Error: {:?}", self);

        (status, Json(serde_json::json!({
            "ok": false,
            "error": {
                "code": self.code(),
                "message": error_message,
                "trace_id": self.trace_id(),
            }
        }))).into_response()
    }
}

impl ApiError {
    pub fn new_internal_error(message: String) -> Self {
        ApiError::InternalServerError {
            code: "INTERNAL_SERVER_ERROR".to_string(),
            message,
            trace_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn new_not_found(message: String) -> Self {
        ApiError::NotFound {
            code: "NOT_FOUND".to_string(),
            message,
            trace_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn code(&self) -> String {
        match self {
            ApiError::InternalServerError { code, .. } => code.clone(),
            ApiError::NotFound { code, .. } => code.clone(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            ApiError::InternalServerError { message, .. } => message.clone(),
            ApiError::NotFound { message, .. } => message.clone(),
        }
    }

    pub fn trace_id(&self) -> String {
        match self {
            ApiError::InternalServerError { trace_id, .. } => trace_id.clone(),
            ApiError::NotFound { trace_id, .. } => trace_id.clone(),
        }
    }
}

// Implement From traits for easy error conversion
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::new_internal_error(err.to_string())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::new_internal_error(err.to_string())
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::new_internal_error(err.to_string())
    }
}

impl From<redis::RedisError> for ApiError {
    fn from(err: redis::RedisError) -> Self {
        ApiError::new_internal_error(err.to_string())
    }
}