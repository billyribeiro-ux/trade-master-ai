use crate::error::AppError;
use crate::models::AuthUser;
use crate::services::AuthService;
use axum::{
    extract::FromRequestParts,
    extract::FromRef,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<AuthService>: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                AppError::Unauthorized("Missing Authorization header".to_string()).into_response()
            })?;

        if !auth_header.starts_with("Bearer ") {
            return Err(
                AppError::Unauthorized("Invalid Authorization header format".to_string())
                    .into_response(),
            );
        }

        let token = &auth_header[7..];

        // Use FromRef to get AuthService without borrowing parts mutably
        let auth_service = Arc::<AuthService>::from_ref(state);

        auth_service
            .verify_access_token(token)
            .map_err(|e| e.into_response())
    }
}

pub struct OptionalAuth(pub Option<AuthUser>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
    Arc<AuthService>: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                let token = &header[7..];
                let auth_service = Arc::<AuthService>::from_ref(state);

                if let Ok(user) = auth_service.verify_access_token(token) {
                    return Ok(OptionalAuth(Some(user)));
                }
            }
        }

        Ok(OptionalAuth(None))
    }
}
