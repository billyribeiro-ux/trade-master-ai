use crate::services::{AiService, AuthService};
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

/// Unified application state shared across all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub auth_service: Arc<AuthService>,
    pub ai_service: Arc<AiService>,
}

// Allow extracting Arc<PgPool> from AppState
impl FromRef<AppState> for Arc<PgPool> {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

// Allow extracting Arc<AuthService> from AppState
impl FromRef<AppState> for Arc<AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

// Allow extracting Arc<AiService> from AppState
impl FromRef<AppState> for Arc<AiService> {
    fn from_ref(state: &AppState) -> Self {
        state.ai_service.clone()
    }
}
