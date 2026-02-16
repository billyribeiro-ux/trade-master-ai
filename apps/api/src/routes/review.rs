use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CreateReviewRequest, DailyPnl, PeriodicReview, ReviewWithStats,
    SetupSummary, UpdateReviewRequest, validate_review_request, validate_rating,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct TradeStats {
    total_trades: Option<i32>,
    winning_trades: Option<i32>,
    losing_trades: Option<i32>,
    win_rate: Option<Decimal>,
    total_pnl: Option<Decimal>,
    avg_r_multiple: Option<Decimal>,
}

pub async fn create_review(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Json(req): Json<CreateReviewRequest>,
) -> AppResult<Json<PeriodicReview>> {
    validate_review_request(&req).map_err(|e| AppError::Validation(e))?;

    // Auto-compute trade statistics for the period
    let stats = sqlx::query_as::<_, TradeStats>(
        r#"
        SELECT
            COUNT(*)::INTEGER as total_trades,
            COUNT(*) FILTER (WHERE net_pnl > 0)::INTEGER as winning_trades,
            COUNT(*) FILTER (WHERE net_pnl <= 0)::INTEGER as losing_trades,
            CASE WHEN COUNT(*) > 0
                THEN CAST(COUNT(*) FILTER (WHERE net_pnl > 0) AS DECIMAL) / COUNT(*) * 100
                ELSE NULL
            END as win_rate,
            SUM(net_pnl) as total_pnl,
            AVG(r_multiple) as avg_r_multiple
        FROM trades
        WHERE user_id = $1
          AND status = 'closed'
          AND DATE(exit_date) >= $2
          AND DATE(exit_date) <= $3
        "#,
    )
    .bind(auth_user.user_id)
    .bind(req.period_start)
    .bind(req.period_end)
    .fetch_one(pool.as_ref())
    .await?;

    let review = sqlx::query_as::<_, PeriodicReview>(
        r#"
        INSERT INTO periodic_reviews (
            user_id, review_type, period_start, period_end,
            total_trades, winning_trades, losing_trades, win_rate, total_pnl, avg_r_multiple,
            what_went_well, what_to_improve, key_lessons, rules_broken,
            best_trade_id, worst_trade_id,
            goals_met, goals_missed, goals_next_period,
            discipline_rating, patience_rating, execution_rating, overall_rating
        )
        VALUES (
            $1, $2, $3, $4,
            $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14,
            $15, $16,
            $17, $18, $19,
            $20, $21, $22, $23
        )
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(&req.review_type)
    .bind(req.period_start)
    .bind(req.period_end)
    .bind(stats.total_trades)
    .bind(stats.winning_trades)
    .bind(stats.losing_trades)
    .bind(stats.win_rate)
    .bind(stats.total_pnl)
    .bind(stats.avg_r_multiple)
    .bind(&req.what_went_well)
    .bind(&req.what_to_improve)
    .bind(&req.key_lessons)
    .bind(&req.rules_broken)
    .bind(req.best_trade_id)
    .bind(req.worst_trade_id)
    .bind(&req.goals_met)
    .bind(&req.goals_missed)
    .bind(&req.goals_next_period)
    .bind(req.discipline_rating)
    .bind(req.patience_rating)
    .bind(req.execution_rating)
    .bind(req.overall_rating)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("A review for this period already exists".to_string())
        }
        _ => AppError::from(e),
    })?;

    tracing::info!(
        review_id = %review.id,
        review_type = %review.review_type,
        period = %format!("{} to {}", review.period_start, review.period_end),
        "Periodic review created"
    );

    Ok(Json(review))
}

pub async fn get_review(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(review_id): Path<Uuid>,
) -> AppResult<Json<ReviewWithStats>> {
    let review = sqlx::query_as::<_, PeriodicReview>(
        "SELECT * FROM periodic_reviews WHERE id = $1 AND user_id = $2",
    )
    .bind(review_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Review not found".to_string()))?;

    let (top_setups, daily_pnl) = tokio::try_join!(
        sqlx::query_as::<_, SetupSummary>(
            r#"
            SELECT
                COALESCE(setup_name, 'Unknown') as setup_name,
                COUNT(*) as count,
                COALESCE(SUM(net_pnl), 0) as total_pnl,
                COALESCE(
                    CAST(COUNT(*) FILTER (WHERE net_pnl > 0) AS DECIMAL)
                    / NULLIF(COUNT(*), 0) * 100,
                    0
                ) as win_rate
            FROM trades
            WHERE user_id = $1
              AND status = 'closed'
              AND DATE(exit_date) >= $2
              AND DATE(exit_date) <= $3
            GROUP BY setup_name
            ORDER BY total_pnl DESC
            LIMIT 10
            "#,
        )
        .bind(auth_user.user_id)
        .bind(review.period_start)
        .bind(review.period_end)
        .fetch_all(pool.as_ref()),

        sqlx::query_as::<_, DailyPnl>(
            r#"
            SELECT
                DATE(exit_date) as trade_date,
                COALESCE(SUM(net_pnl), 0) as pnl,
                COUNT(*) as trade_count
            FROM trades
            WHERE user_id = $1
              AND status = 'closed'
              AND DATE(exit_date) >= $2
              AND DATE(exit_date) <= $3
            GROUP BY DATE(exit_date)
            ORDER BY trade_date
            "#,
        )
        .bind(auth_user.user_id)
        .bind(review.period_start)
        .bind(review.period_end)
        .fetch_all(pool.as_ref()),
    )?;

    Ok(Json(ReviewWithStats {
        review,
        top_setups,
        daily_pnl,
    }))
}

#[derive(Debug, Deserialize)]
pub struct ReviewListQuery {
    pub review_type: Option<String>,
    pub limit: Option<i64>,
}

pub async fn list_reviews(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Query(query): Query<ReviewListQuery>,
) -> AppResult<Json<Vec<PeriodicReview>>> {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);

    let reviews = if let Some(ref review_type) = query.review_type {
        sqlx::query_as::<_, PeriodicReview>(
            "SELECT * FROM periodic_reviews WHERE user_id = $1 AND review_type = $2 ORDER BY period_end DESC LIMIT $3",
        )
        .bind(auth_user.user_id)
        .bind(review_type)
        .bind(limit)
        .fetch_all(pool.as_ref())
        .await?
    } else {
        sqlx::query_as::<_, PeriodicReview>(
            "SELECT * FROM periodic_reviews WHERE user_id = $1 ORDER BY period_end DESC LIMIT $2",
        )
        .bind(auth_user.user_id)
        .bind(limit)
        .fetch_all(pool.as_ref())
        .await?
    };

    Ok(Json(reviews))
}

pub async fn update_review(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(review_id): Path<Uuid>,
    Json(req): Json<UpdateReviewRequest>,
) -> AppResult<Json<PeriodicReview>> {
    if let Some(r) = req.discipline_rating {
        validate_rating(r, "discipline_rating").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(r) = req.patience_rating {
        validate_rating(r, "patience_rating").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(r) = req.execution_rating {
        validate_rating(r, "execution_rating").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(r) = req.overall_rating {
        validate_rating(r, "overall_rating").map_err(|e| AppError::Validation(e))?;
    }

    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM periodic_reviews WHERE id = $1 AND user_id = $2)",
    )
    .bind(review_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Review not found".to_string()))?;

    let review = sqlx::query_as::<_, PeriodicReview>(
        r#"
        UPDATE periodic_reviews SET
            what_went_well = COALESCE($1, what_went_well),
            what_to_improve = COALESCE($2, what_to_improve),
            key_lessons = COALESCE($3, key_lessons),
            rules_broken = COALESCE($4, rules_broken),
            best_trade_id = COALESCE($5, best_trade_id),
            worst_trade_id = COALESCE($6, worst_trade_id),
            goals_met = COALESCE($7, goals_met),
            goals_missed = COALESCE($8, goals_missed),
            goals_next_period = COALESCE($9, goals_next_period),
            discipline_rating = COALESCE($10, discipline_rating),
            patience_rating = COALESCE($11, patience_rating),
            execution_rating = COALESCE($12, execution_rating),
            overall_rating = COALESCE($13, overall_rating),
            updated_at = NOW()
        WHERE id = $14
        RETURNING *
        "#,
    )
    .bind(&req.what_went_well)
    .bind(&req.what_to_improve)
    .bind(&req.key_lessons)
    .bind(&req.rules_broken)
    .bind(req.best_trade_id)
    .bind(req.worst_trade_id)
    .bind(&req.goals_met)
    .bind(&req.goals_missed)
    .bind(&req.goals_next_period)
    .bind(req.discipline_rating)
    .bind(req.patience_rating)
    .bind(req.execution_rating)
    .bind(req.overall_rating)
    .bind(review_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(review))
}

pub async fn delete_review(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(review_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query(
        "DELETE FROM periodic_reviews WHERE id = $1 AND user_id = $2",
    )
    .bind(review_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Review not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Review deleted" })))
}
