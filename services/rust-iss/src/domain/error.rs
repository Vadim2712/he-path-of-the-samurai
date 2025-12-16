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
}

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    message: String,
    trace_id: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    ok: bool,
    error: ErrorBody,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message, trace_id) = match self {
            ApiError::InternalServerError { code, message, trace_id } => {
                (StatusCode::INTERNAL_SERVER_ERROR, code, message, trace_id)
            }
            ApiError::NotFound { code, message, trace_id } => {
                (StatusCode::NOT_FOUND, code, message, trace_id)
            }
        };

        let error_body = ErrorBody {
            code,
            message,
            trace_id: trace_id.clone(),
        };

        let error_response = ErrorResponse {
            ok: false,
            error: error_body,
        };
        
        error!("API Error: status={}, trace_id={}", status, trace_id);

        (status, Json(error_response)).into_response()
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