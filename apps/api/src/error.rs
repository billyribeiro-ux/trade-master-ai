use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred".to_string(),
                )
            }
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR", msg.clone()),
            AppError::RateLimited(msg) => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED", msg.clone()),
            AppError::AiError(msg) => (StatusCode::SERVICE_UNAVAILABLE, "AI_ERROR", msg.clone()),
            AppError::BrokerError(msg) => (StatusCode::BAD_GATEWAY, "BROKER_ERROR", msg.clone()),
        };

        let body = Json(ErrorResponse {
            error: ErrorDetail {
                code: code.to_string(),
                message,
                details: None,
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
                if let Some(constraint) = db_err.constraint() {
                    AppError::Conflict(format!("Constraint violation: {}", constraint))
                } else {
                    AppError::Internal(format!("Database error: {}", db_err))
                }
            }
            _ => AppError::Internal(format!("Database error: {}", err)),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Internal(format!("HTTP client error: {}", err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(format!("Error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized(format!("JWT error: {}", err))
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::Internal(format!("Password hashing error: {}", err))
    }
}

pub type AppResult<T> = Result<T, AppError>;
