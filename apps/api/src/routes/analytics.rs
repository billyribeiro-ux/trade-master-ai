use crate::error::AppResult;
use crate::models::AuthUser;
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;

#[derive(Debug, Serialize, FromRow)]
pub struct EquityCurvePoint {
    pub date: DateTime<Utc>,
    pub cumulative_pnl: Decimal,
    pub trade_count: i64,
}

#[derive(Debug, Serialize)]
pub struct EquityCurveResponse {
    pub points: Vec<EquityCurvePoint>,
    pub starting_balance: Decimal,
}

pub async fn get_equity_curve(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<EquityCurveResponse>> {
    let points = sqlx::query_as::<_, EquityCurvePoint>(
        r#"
        SELECT 
            exit_date as date,
            SUM(net_pnl) OVER (ORDER BY exit_date) as cumulative_pnl,
            ROW_NUMBER() OVER (ORDER BY exit_date) as trade_count
        FROM trades
        WHERE user_id = $1 
            AND status = 'closed' 
            AND exit_date IS NOT NULL
            AND net_pnl IS NOT NULL
        ORDER BY exit_date
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(EquityCurveResponse {
        points,
        starting_balance: Decimal::from(0),
    }))
}

#[derive(Debug, Serialize)]
pub struct WinLossDistribution {
    pub wins: Vec<Decimal>,
    pub losses: Vec<Decimal>,
    pub avg_win: Decimal,
    pub avg_loss: Decimal,
    pub largest_win: Decimal,
    pub largest_loss: Decimal,
}

pub async fn get_win_loss_distribution(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<WinLossDistribution>> {
    let wins = sqlx::query_scalar::<_, Decimal>(
        r#"
        SELECT net_pnl
        FROM trades
        WHERE user_id = $1 AND status = 'closed' AND net_pnl > 0
        ORDER BY net_pnl DESC
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    let losses = sqlx::query_scalar::<_, Decimal>(
        r#"
        SELECT net_pnl
        FROM trades
        WHERE user_id = $1 AND status = 'closed' AND net_pnl <= 0
        ORDER BY net_pnl ASC
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    let avg_win = if !wins.is_empty() {
        wins.iter().sum::<Decimal>() / Decimal::from(wins.len())
    } else {
        Decimal::ZERO
    };

    let avg_loss = if !losses.is_empty() {
        losses.iter().sum::<Decimal>() / Decimal::from(losses.len())
    } else {
        Decimal::ZERO
    };

    let largest_win = wins.first().copied().unwrap_or(Decimal::ZERO);
    let largest_loss = losses.first().copied().unwrap_or(Decimal::ZERO);

    Ok(Json(WinLossDistribution {
        wins,
        losses,
        avg_win,
        avg_loss,
        largest_win,
        largest_loss,
    }))
}

#[derive(Debug, Serialize, FromRow)]
pub struct SetupPerformance {
    pub setup_name: String,
    pub trade_count: i64,
    pub win_count: i64,
    pub loss_count: i64,
    pub win_rate: Decimal,
    pub total_pnl: Decimal,
    pub avg_pnl: Decimal,
    pub avg_r_multiple: Option<Decimal>,
}

pub async fn get_setup_performance(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<SetupPerformance>>> {
    let setups = sqlx::query_as::<_, SetupPerformance>(
        r#"
        SELECT 
            COALESCE(setup_name, 'No Setup') as setup_name,
            COUNT(*) as trade_count,
            COUNT(*) FILTER (WHERE net_pnl > 0) as win_count,
            COUNT(*) FILTER (WHERE net_pnl <= 0) as loss_count,
            COALESCE(
                CAST(COUNT(*) FILTER (WHERE net_pnl > 0) AS DECIMAL) / NULLIF(COUNT(*), 0) * 100,
                0
            ) as win_rate,
            COALESCE(SUM(net_pnl), 0) as total_pnl,
            COALESCE(AVG(net_pnl), 0) as avg_pnl,
            AVG(r_multiple) as avg_r_multiple
        FROM trades
        WHERE user_id = $1 AND status = 'closed'
        GROUP BY setup_name
        HAVING COUNT(*) >= 3
        ORDER BY total_pnl DESC
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(setups))
}

#[derive(Debug, Serialize)]
pub struct TimeBasedAnalytics {
    pub hourly: Vec<HourlyPerformance>,
    pub daily: Vec<DailyPerformance>,
    pub monthly: Vec<MonthlyPerformance>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HourlyPerformance {
    pub hour: i32,
    pub trade_count: i64,
    pub win_rate: Decimal,
    pub avg_pnl: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct DailyPerformance {
    pub day_of_week: i32,
    pub day_name: String,
    pub trade_count: i64,
    pub win_rate: Decimal,
    pub avg_pnl: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MonthlyPerformance {
    pub month: String,
    pub trade_count: i64,
    pub total_pnl: Decimal,
    pub win_rate: Decimal,
}

pub async fn get_time_based_analytics(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<TimeBasedAnalytics>> {
    // Hourly performance
    let hourly = sqlx::query_as::<_, HourlyPerformance>(
        r#"
        SELECT 
            EXTRACT(HOUR FROM entry_date)::INTEGER as hour,
            COUNT(*) as trade_count,
            COALESCE(
                CAST(COUNT(*) FILTER (WHERE net_pnl > 0) AS DECIMAL) / NULLIF(COUNT(*), 0) * 100,
                0
            ) as win_rate,
            COALESCE(AVG(net_pnl), 0) as avg_pnl
        FROM trades
        WHERE user_id = $1 AND status = 'closed'
        GROUP BY hour
        ORDER BY hour
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    // Daily performance
    let daily = sqlx::query_as::<_, DailyPerformance>(
        r#"
        SELECT 
            EXTRACT(DOW FROM entry_date)::INTEGER as day_of_week,
            TO_CHAR(entry_date, 'Day') as day_name,
            COUNT(*) as trade_count,
            COALESCE(
                CAST(COUNT(*) FILTER (WHERE net_pnl > 0) AS DECIMAL) / NULLIF(COUNT(*), 0) * 100,
                0
            ) as win_rate,
            COALESCE(AVG(net_pnl), 0) as avg_pnl
        FROM trades
        WHERE user_id = $1 AND status = 'closed'
        GROUP BY day_of_week, day_name
        ORDER BY day_of_week
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    // Monthly performance
    let monthly = sqlx::query_as::<_, MonthlyPerformance>(
        r#"
        SELECT 
            TO_CHAR(entry_date, 'YYYY-MM') as month,
            COUNT(*) as trade_count,
            COALESCE(SUM(net_pnl), 0) as total_pnl,
            COALESCE(
                CAST(COUNT(*) FILTER (WHERE net_pnl > 0) AS DECIMAL) / NULLIF(COUNT(*), 0) * 100,
                0
            ) as win_rate
        FROM trades
        WHERE user_id = $1 AND status = 'closed'
        GROUP BY month
        ORDER BY month DESC
        LIMIT 12
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(TimeBasedAnalytics {
        hourly,
        daily,
        monthly,
    }))
}

#[derive(Debug, Serialize)]
pub struct DrawdownAnalysis {
    pub current_drawdown: Decimal,
    pub max_drawdown: Decimal,
    pub max_drawdown_date: Option<DateTime<Utc>>,
    pub recovery_factor: Option<Decimal>,
    pub drawdown_periods: Vec<DrawdownPeriod>,
}

#[derive(Debug, Serialize)]
pub struct DrawdownPeriod {
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub drawdown_amount: Decimal,
    pub drawdown_percent: Decimal,
    pub duration_days: Option<i32>,
}

pub async fn get_drawdown_analysis(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<DrawdownAnalysis>> {
    // Get equity curve points
    let points = sqlx::query_as::<_, EquityCurvePoint>(
        r#"
        SELECT 
            exit_date as date,
            SUM(net_pnl) OVER (ORDER BY exit_date) as cumulative_pnl,
            ROW_NUMBER() OVER (ORDER BY exit_date) as trade_count
        FROM trades
        WHERE user_id = $1 AND status = 'closed' AND exit_date IS NOT NULL
        ORDER BY exit_date
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    let mut max_drawdown = Decimal::ZERO;
    let mut max_drawdown_date = None;
    let mut peak = Decimal::ZERO;
    let mut current_drawdown = Decimal::ZERO;

    for point in &points {
        if point.cumulative_pnl > peak {
            peak = point.cumulative_pnl;
        }

        let drawdown = peak - point.cumulative_pnl;
        if drawdown > max_drawdown {
            max_drawdown = drawdown;
            max_drawdown_date = Some(point.date);
        }

        if point.date == points.last().map(|p| p.date).unwrap_or_default() {
            current_drawdown = drawdown;
        }
    }

    let total_pnl = points.last().map(|p| p.cumulative_pnl).unwrap_or(Decimal::ZERO);
    let recovery_factor = if max_drawdown > Decimal::ZERO {
        Some(total_pnl / max_drawdown)
    } else {
        None
    };

    Ok(Json(DrawdownAnalysis {
        current_drawdown,
        max_drawdown,
        max_drawdown_date,
        recovery_factor,
        drawdown_periods: vec![],
    }))
}
