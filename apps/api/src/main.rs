mod config;
mod error;
mod middleware;
mod models;
mod routes;
mod services;

use crate::config::Config;
use crate::routes::{auth, health, tags, trades};
use crate::services::AuthService;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trademaster_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    config.validate()?;

    tracing::info!("Starting TradeMaster AI API v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Port: {}", config.port);
    tracing::info!("CORS origins: {:?}", config.cors_origins);

    // Create database connection pool
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(config.max_pool_connections)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Database connected successfully");

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Migrations completed");

    // Create shared services
    let auth_service = Arc::new(AuthService::new(&config));
    let pool = Arc::new(pool);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origins
                .iter()
                .map(|origin| origin.parse().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/api/health", get(health::health_check))
        // Auth routes
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/refresh", post(auth::refresh))
        .route("/api/v1/auth/logout", post(auth::logout))
        .route("/api/v1/auth/me", get(auth::me))
        // Trade routes
        .route("/api/v1/trades", post(trades::create_trade))
        .route("/api/v1/trades", get(trades::list_trades))
        .route("/api/v1/trades/stats", get(trades::get_trade_stats))
        .route("/api/v1/trades/:id", get(trades::get_trade))
        .route("/api/v1/trades/:id", put(trades::update_trade))
        .route("/api/v1/trades/:id", delete(trades::delete_trade))
        .route("/api/v1/trades/:id/close", post(trades::close_trade))
        .route("/api/v1/trades/:id/legs", post(trades::add_trade_leg))
        // Tag routes
        .route("/api/v1/tags", post(tags::create_tag))
        .route("/api/v1/tags", get(tags::list_tags))
        .route("/api/v1/tags/:id", get(tags::get_tag))
        .route("/api/v1/tags/:id", put(tags::update_tag))
        .route("/api/v1/tags/:id", delete(tags::delete_tag))
        .route("/api/v1/trades/:trade_id/tags/:tag_id", post(tags::add_tag_to_trade))
        .route("/api/v1/trades/:trade_id/tags/:tag_id", delete(tags::remove_tag_from_trade))
        // Add state
        .with_state(pool.clone())
        .with_state(auth_service.clone())
        // Add middleware
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("Server listening on {}", addr);
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, starting graceful shutdown");
}
