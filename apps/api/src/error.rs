use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
    Validation(String),
    RateLimited(String),
    AiError(String),
    BrokerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal Error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            AppError::RateLimited(msg) => write!(f, "Rate Limited: {}", msg),
            AppError::AiError(msg) => write!(f, "AI Error: {}", msg),
            AppError::BrokerError(msg) => write!(f, "Broker Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace_id: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let trace_id = Uuid::new_v4().to_string();

        let (status, code, message, include_trace) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone(), false),
            AppError::Unauthorized(_) => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Authentication required".to_string(),
                false,
            ),
            AppError::Forbidden(_) => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "You do not have permission to perform this action".to_string(),
                false,
            ),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone(), false),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone(), false),
            AppError::Internal(msg) => {
                tracing::error!(trace_id = %trace_id, error = %msg, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred. If this persists, contact support.".to_string(),
                    true,
                )
            }
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR", msg.clone(), false),
            AppError::RateLimited(msg) => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED", msg.clone(), false),
            AppError::AiError(msg) => {
                tracing::warn!(trace_id = %trace_id, error = %msg, "AI service error");
                (StatusCode::SERVICE_UNAVAILABLE, "AI_ERROR", msg.clone(), true)
            }
            AppError::BrokerError(msg) => {
                tracing::warn!(trace_id = %trace_id, error = %msg, "Broker error");
                (StatusCode::BAD_GATEWAY, "BROKER_ERROR", msg.clone(), true)
            }
        };

        let body = Json(ErrorResponse {
            error: ErrorDetail {
                code: code.to_string(),
                message,
                details: None,
                trace_id: if include_trace { Some(trace_id) } else { None },
            },
        });

        (status, body).into_response()
    }
}

// Conversions from common error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    AppError::Conflict("A resource with that identifier already exists".to_string())
                } else if db_err.is_foreign_key_violation() {
                    AppError::BadRequest("Referenced resource does not exist".to_string())
                } else if db_err.is_check_violation() {
                    AppError::Validation("Data constraint check failed".to_string())
                } else {
                    // Log the real error server-side, return a generic message
                    tracing::error!(error = %db_err, "Unhandled database error");
                    AppError::Internal(format!("Database error: {}", db_err))
                }
            }
            _ => {
                tracing::error!(error = %err, "Unhandled database error");
                AppError::Internal(format!("Database error: {}", err))
            }
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        tracing::error!(error = %err, "HTTP client error");
        AppError::Internal("An external service request failed".to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(format!("Error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        tracing::debug!(error = %err, "JWT validation failed");
        AppError::Unauthorized("Invalid or expired token".to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        tracing::error!(error = %err, "Password hashing error");
        AppError::Internal("Authentication processing error".to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
