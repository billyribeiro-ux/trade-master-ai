use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CreateDailyPlanRequest, CreateWatchlistItemRequest, DailyPlan, DailyPlanWithWatchlist,
    UpdateDailyPlanRequest, UpdateWatchlistItemRequest, WatchlistItem,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    pub date: Option<String>,
}

pub async fn create_daily_plan(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Json(req): Json<CreateDailyPlanRequest>,
) -> AppResult<Json<DailyPlan>> {
    // Validate market_bias if provided
    if let Some(ref bias) = req.market_bias {
        match bias.as_str() {
            "bullish" | "bearish" | "neutral" => {}
            _ => {
                return Err(AppError::Validation(
                    "market_bias must be 'bullish', 'bearish', or 'neutral'".to_string(),
                ));
            }
        }
    }

    let plan = sqlx::query_as::<_, DailyPlan>(
        r#"
        INSERT INTO daily_plans (
            user_id, plan_date, market_bias, bias_reasoning,
            session_goals, max_trades, max_daily_loss,
            checklist_items, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(req.plan_date)
    .bind(&req.market_bias)
    .bind(&req.bias_reasoning)
    .bind(&req.session_goals)
    .bind(req.max_trades)
    .bind(req.max_daily_loss)
    .bind(&req.checklist_items)
    .bind(&req.notes)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("A plan for this date already exists".to_string())
        }
        _ => AppError::from(e),
    })?;

    tracing::info!(plan_id = %plan.id, date = %plan.plan_date, "Daily plan created");

    Ok(Json(plan))
}

pub async fn get_daily_plan(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(plan_id): Path<Uuid>,
) -> AppResult<Json<DailyPlanWithWatchlist>> {
    let plan = sqlx::query_as::<_, DailyPlan>(
        "SELECT * FROM daily_plans WHERE id = $1 AND user_id = $2",
    )
    .bind(plan_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Daily plan not found".to_string()))?;

    let watchlist = sqlx::query_as::<_, WatchlistItem>(
        "SELECT * FROM watchlist_items WHERE plan_id = $1 ORDER BY sort_order, created_at",
    )
    .bind(plan_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(DailyPlanWithWatchlist { plan, watchlist }))
}

pub async fn get_daily_plan_by_date(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Query(query): Query<DateQuery>,
) -> AppResult<Json<Option<DailyPlanWithWatchlist>>> {
    let date = if let Some(ref date_str) = query.date {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| AppError::Validation("Invalid date format. Use YYYY-MM-DD.".to_string()))?
    } else {
        chrono::Utc::now().date_naive()
    };

    let plan = sqlx::query_as::<_, DailyPlan>(
        "SELECT * FROM daily_plans WHERE user_id = $1 AND plan_date = $2",
    )
    .bind(auth_user.user_id)
    .bind(date)
    .fetch_optional(pool.as_ref())
    .await?;

    if let Some(plan) = plan {
        let watchlist = sqlx::query_as::<_, WatchlistItem>(
            "SELECT * FROM watchlist_items WHERE plan_id = $1 ORDER BY sort_order, created_at",
        )
        .bind(plan.id)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(Json(Some(DailyPlanWithWatchlist { plan, watchlist })))
    } else {
        Ok(Json(None))
    }
}

pub async fn list_daily_plans(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<DailyPlan>>> {
    let plans = sqlx::query_as::<_, DailyPlan>(
        "SELECT * FROM daily_plans WHERE user_id = $1 ORDER BY plan_date DESC LIMIT 30",
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(plans))
}

pub async fn update_daily_plan(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(plan_id): Path<Uuid>,
    Json(req): Json<UpdateDailyPlanRequest>,
) -> AppResult<Json<DailyPlan>> {
    // Validate market_bias if provided
    if let Some(ref bias) = req.market_bias {
        match bias.as_str() {
            "bullish" | "bearish" | "neutral" => {}
            _ => {
                return Err(AppError::Validation(
                    "market_bias must be 'bullish', 'bearish', or 'neutral'".to_string(),
                ));
            }
        }
    }

    // Verify ownership
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM daily_plans WHERE id = $1 AND user_id = $2)",
    )
    .bind(plan_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Daily plan not found".to_string()))?;

    let plan = sqlx::query_as::<_, DailyPlan>(
        r#"
        UPDATE daily_plans SET
            market_bias = COALESCE($1, market_bias),
            bias_reasoning = COALESCE($2, bias_reasoning),
            session_goals = COALESCE($3, session_goals),
            max_trades = COALESCE($4, max_trades),
            max_daily_loss = COALESCE($5, max_daily_loss),
            checklist_items = COALESCE($6, checklist_items),
            notes = COALESCE($7, notes),
            completed = COALESCE($8, completed),
            completed_at = CASE WHEN $8 = TRUE THEN NOW() ELSE completed_at END,
            updated_at = NOW()
        WHERE id = $9
        RETURNING *
        "#,
    )
    .bind(&req.market_bias)
    .bind(&req.bias_reasoning)
    .bind(&req.session_goals)
    .bind(req.max_trades)
    .bind(req.max_daily_loss)
    .bind(&req.checklist_items)
    .bind(&req.notes)
    .bind(req.completed)
    .bind(plan_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(plan))
}

pub async fn delete_daily_plan(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(plan_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM daily_plans WHERE id = $1 AND user_id = $2")
        .bind(plan_id)
        .bind(auth_user.user_id)
        .execute(pool.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Daily plan not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Plan deleted" })))
}

pub async fn add_watchlist_item(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(plan_id): Path<Uuid>,
    Json(req): Json<CreateWatchlistItemRequest>,
) -> AppResult<Json<WatchlistItem>> {
    let symbol = req.symbol.trim().to_uppercase();
    if symbol.is_empty() || symbol.len() > 20 {
        return Err(AppError::Validation(
            "Symbol must be between 1 and 20 characters".to_string(),
        ));
    }

    // Verify plan ownership
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM daily_plans WHERE id = $1 AND user_id = $2)",
    )
    .bind(plan_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Daily plan not found".to_string()))?;

    // Get next sort_order
    let next_order = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(sort_order) FROM watchlist_items WHERE plan_id = $1",
    )
    .bind(plan_id)
    .fetch_one(pool.as_ref())
    .await?
    .map(|n| n + 1)
    .unwrap_or(0);

    let item = sqlx::query_as::<_, WatchlistItem>(
        r#"
        INSERT INTO watchlist_items (
            plan_id, symbol, key_levels, catalysts,
            setup_description, risk_reward_ratio,
            position_size_suggested, sort_order
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(plan_id)
    .bind(&symbol)
    .bind(&req.key_levels)
    .bind(&req.catalysts)
    .bind(&req.setup_description)
    .bind(req.risk_reward_ratio)
    .bind(req.position_size_suggested)
    .bind(next_order)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(item))
}

pub async fn update_watchlist_item(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path((plan_id, item_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateWatchlistItemRequest>,
) -> AppResult<Json<WatchlistItem>> {
    // Verify ownership through plan
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM watchlist_items wi
            JOIN daily_plans dp ON wi.plan_id = dp.id
            WHERE wi.id = $1 AND wi.plan_id = $2 AND dp.user_id = $3
        )
        "#,
    )
    .bind(item_id)
    .bind(plan_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Watchlist item not found".to_string()))?;

    // Validate outcome if provided
    if let Some(ref outcome) = req.outcome {
        match outcome.as_str() {
            "winner" | "loser" | "missed" | "no_setup" => {}
            _ => {
                return Err(AppError::Validation(
                    "outcome must be 'winner', 'loser', 'missed', or 'no_setup'".to_string(),
                ));
            }
        }
    }

    let item = sqlx::query_as::<_, WatchlistItem>(
        r#"
        UPDATE watchlist_items SET
            symbol = COALESCE($1, symbol),
            key_levels = COALESCE($2, key_levels),
            catalysts = COALESCE($3, catalysts),
            setup_description = COALESCE($4, setup_description),
            risk_reward_ratio = COALESCE($5, risk_reward_ratio),
            position_size_suggested = COALESCE($6, position_size_suggested),
            was_traded = COALESCE($7, was_traded),
            outcome = COALESCE($8, outcome),
            updated_at = NOW()
        WHERE id = $9
        RETURNING *
        "#,
    )
    .bind(&req.symbol)
    .bind(&req.key_levels)
    .bind(&req.catalysts)
    .bind(&req.setup_description)
    .bind(req.risk_reward_ratio)
    .bind(req.position_size_suggested)
    .bind(req.was_traded)
    .bind(&req.outcome)
    .bind(item_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(item))
}

pub async fn delete_watchlist_item(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path((plan_id, item_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query(
        r#"
        DELETE FROM watchlist_items
        WHERE id = $1 AND plan_id = $2
        AND plan_id IN (SELECT id FROM daily_plans WHERE user_id = $3)
        "#,
    )
    .bind(item_id)
    .bind(plan_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Watchlist item not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Watchlist item deleted" })))
}
