use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    timestamp: String,
    database: String,
}

pub async fn health_check(State(pool): State<Arc<PgPool>>) -> Json<HealthResponse> {
    let db_status = match sqlx::query("SELECT 1").fetch_one(pool.as_ref()).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: db_status.to_string(),
    })
}
