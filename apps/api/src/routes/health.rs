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

#[derive(Serialize)]
pub struct DetailedHealthResponse {
    status: String,
    version: String,
    timestamp: String,
    database: DatabaseHealth,
    uptime_seconds: u64,
}

#[derive(Serialize)]
pub struct DatabaseHealth {
    status: String,
    connections: u32,
    response_time_ms: u64,
}

/// Basic health check endpoint - fast response for load balancers
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

/// Detailed health check with metrics - for monitoring dashboards
pub async fn detailed_health_check(
    State(pool): State<Arc<PgPool>>,
) -> Json<DetailedHealthResponse> {
    let start = std::time::Instant::now();

    // Check database connectivity and measure response time
    let db_health = match sqlx::query("SELECT 1")
        .fetch_one(pool.as_ref())
        .await
    {
        Ok(_) => {
            let response_time = start.elapsed().as_millis() as u64;
            DatabaseHealth {
                status: "healthy".to_string(),
                connections: pool.size(),
                response_time_ms: response_time,
            }
        }
        Err(e) => {
            tracing::error!(error = ?e, "Database health check failed");
            DatabaseHealth {
                status: "unhealthy".to_string(),
                connections: 0,
                response_time_ms: 0,
            }
        }
    };

    let overall_status = if db_health.status == "healthy" {
        "healthy"
    } else {
        "degraded"
    };

    Json(DetailedHealthResponse {
        status: overall_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: db_health,
        uptime_seconds: 0, // TODO: Track actual uptime with lazy_static
    })
}
