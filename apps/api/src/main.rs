mod config;
mod error;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use crate::config::Config;
use crate::routes::{ai_review, analytics, auth, csv, health, planning, playbook, psychology, review, risk, tags, trades};
use crate::services::{AiService, AuthService};
use crate::state::AppState;
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
    let ai_service = Arc::new(AiService::new(&config));
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
        // CSV import routes
        .route("/api/v1/csv/import", post(csv::import_csv))
        .route("/api/v1/csv/template", get(csv::get_csv_template))
        // Analytics routes
        .route("/api/v1/analytics/equity-curve", get(analytics::get_equity_curve))
        .route("/api/v1/analytics/win-loss-distribution", get(analytics::get_win_loss_distribution))
        .route("/api/v1/analytics/setup-performance", get(analytics::get_setup_performance))
        .route("/api/v1/analytics/time-based", get(analytics::get_time_based_analytics))
        .route("/api/v1/analytics/drawdown", get(analytics::get_drawdown_analysis))
        // Planning routes
        .route("/api/v1/plans", post(planning::create_daily_plan))
        .route("/api/v1/plans", get(planning::list_daily_plans))
        .route("/api/v1/plans/by-date", get(planning::get_daily_plan_by_date))
        .route("/api/v1/plans/:id", get(planning::get_daily_plan))
        .route("/api/v1/plans/:id", put(planning::update_daily_plan))
        .route("/api/v1/plans/:id", delete(planning::delete_daily_plan))
        .route("/api/v1/plans/:id/watchlist", post(planning::add_watchlist_item))
        .route("/api/v1/plans/:plan_id/watchlist/:item_id", put(planning::update_watchlist_item))
        .route("/api/v1/plans/:plan_id/watchlist/:item_id", delete(planning::delete_watchlist_item))
        // AI Review routes
        .route("/api/v1/ai/reviews", post(ai_review::create_ai_review))
        .route("/api/v1/ai/reviews", get(ai_review::list_ai_reviews))
        .route("/api/v1/ai/reviews/:id", get(ai_review::get_ai_review))
        .route("/api/v1/ai/reviews/:id", delete(ai_review::delete_ai_review))
        .route("/api/v1/ai/reviews/:id/chat", post(ai_review::continue_chat))
        // Risk Management routes
        .route("/api/v1/risk/position-size", post(risk::calculate_position_size))
        .route("/api/v1/risk/risk-reward", post(risk::calculate_risk_reward))
        .route("/api/v1/risk/kelly", post(risk::calculate_kelly))
        .route("/api/v1/risk/portfolio-heat", post(risk::calculate_portfolio_heat))
        // Psychology routes
        .route("/api/v1/psychology/mood-logs", post(psychology::create_mood_log))
        .route("/api/v1/psychology/mood-logs", get(psychology::list_mood_logs))
        .route("/api/v1/psychology/mood-logs/:id", get(psychology::get_mood_log))
        .route("/api/v1/psychology/mood-logs/:id", put(psychology::update_mood_log))
        .route("/api/v1/psychology/mood-logs/:id", delete(psychology::delete_mood_log))
        .route("/api/v1/psychology/insights", get(psychology::get_psychology_insights))
        // Playbook routes
        .route("/api/v1/playbook", post(playbook::create_playbook_entry))
        .route("/api/v1/playbook", get(playbook::list_playbook_entries))
        .route("/api/v1/playbook/performance", get(playbook::get_playbook_performance))
        .route("/api/v1/playbook/:id", get(playbook::get_playbook_entry))
        .route("/api/v1/playbook/:id", put(playbook::update_playbook_entry))
        .route("/api/v1/playbook/:id", delete(playbook::delete_playbook_entry))
        // Review routes
        .route("/api/v1/reviews", post(review::create_review))
        .route("/api/v1/reviews", get(review::list_reviews))
        .route("/api/v1/reviews/:id", get(review::get_review))
        .route("/api/v1/reviews/:id", put(review::update_review))
        .route("/api/v1/reviews/:id", delete(review::delete_review))
        // Add unified state
        .with_state(AppState {
            pool: pool.clone(),
            auth_service: auth_service.clone(),
            ai_service: ai_service.clone(),
        })
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
