use crate::error::{AppError, AppResult};
use crate::models::AuthUser;
use crate::services::AuthService;
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

pub struct RequireAuth;

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<AuthService>: FromRequestParts<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                AppError::Unauthorized("Missing Authorization header".to_string()).into_response()
            })?;

        // Check for Bearer token
        if !auth_header.starts_with("Bearer ") {
            return Err(
                AppError::Unauthorized("Invalid Authorization header format".to_string())
                    .into_response(),
            );
        }

        let token = &auth_header[7..]; // Remove "Bearer " prefix

        // Extract the AuthService from state
        let auth_service = Arc::<AuthService>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                AppError::Internal("Failed to extract AuthService".to_string()).into_response()
            })?;

        // Verify the token
        auth_service
            .verify_access_token(token)
            .map_err(|e| e.into_response())
    }
}

// Optional auth extractor (doesn't fail if no token)
pub struct OptionalAuth(pub Option<AuthUser>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
    Arc<AuthService>: FromRequestParts<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                let token = &header[7..];
                let auth_service = Arc::<AuthService>::from_request_parts(parts, state)
                    .await
                    .map_err(|_| {
                        AppError::Internal("Failed to extract AuthService".to_string())
                            .into_response()
                    })?;

                if let Ok(user) = auth_service.verify_access_token(token) {
                    return Ok(OptionalAuth(Some(user)));
                }
            }
        }

        Ok(OptionalAuth(None))
    }
}
