use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CreateDailyPlanRequest, CreateWatchlistItemRequest, DailyPlan, DailyPlanWithWatchlist,
    UpdateDailyPlanRequest, UpdateWatchlistItemRequest, WatchlistItem,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
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
    let plan = sqlx::query_as::<_, DailyPlan>(
        r#"
        INSERT INTO daily_plans (
            user_id, plan_date, market_bias, key_levels, trade_plan,
            max_trades, max_loss, focus_setups, avoid_setups,
            pre_market_notes, goals_for_day, emotional_state
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(req.plan_date)
    .bind(&req.market_bias)
    .bind(&req.key_levels)
    .bind(&req.trade_plan)
    .bind(req.max_trades)
    .bind(req.max_loss)
    .bind(&req.focus_setups)
    .bind(&req.avoid_setups)
    .bind(&req.pre_market_notes)
    .bind(&req.goals_for_day)
    .bind(&req.emotional_state)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Plan for this date already exists".to_string())
        }
        _ => AppError::from(e),
    })?;

    Ok(Json(plan))
}

pub async fn get_daily_plan(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(plan_id): Path<Uuid>,
) -> AppResult<Json<DailyPlanWithWatchlist>> {
    let plan = sqlx::query_as::<_, DailyPlan>(
        r#"
        SELECT * FROM daily_plans
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(plan_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Daily plan not found".to_string()))?;

    let watchlist = sqlx::query_as::<_, WatchlistItem>(
        r#"
        SELECT * FROM watchlist_items
        WHERE daily_plan_id = $1
        ORDER BY created_at
        "#,
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
    let date = if let Some(date_str) = query.date {
        DateTime::parse_from_rfc3339(&date_str)
            .map_err(|_| AppError::Validation("Invalid date format".to_string()))?
            .with_timezone(&Utc)
    } else {
        Utc::now()
    };

    let plan = sqlx::query_as::<_, DailyPlan>(
        r#"
        SELECT * FROM daily_plans
        WHERE user_id = $1 AND DATE(plan_date) = DATE($2)
        "#,
    )
    .bind(auth_user.user_id)
    .bind(date)
    .fetch_optional(pool.as_ref())
    .await?;

    if let Some(plan) = plan {
        let watchlist = sqlx::query_as::<_, WatchlistItem>(
            r#"
            SELECT * FROM watchlist_items
            WHERE daily_plan_id = $1
            ORDER BY created_at
            "#,
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
        r#"
        SELECT * FROM daily_plans
        WHERE user_id = $1
        ORDER BY plan_date DESC
        LIMIT 30
        "#,
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
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(SELECT 1 FROM daily_plans WHERE id = $1 AND user_id = $2)
        "#,
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
            key_levels = COALESCE($2, key_levels),
            trade_plan = COALESCE($3, trade_plan),
            max_trades = COALESCE($4, max_trades),
            max_loss = COALESCE($5, max_loss),
            focus_setups = COALESCE($6, focus_setups),
            avoid_setups = COALESCE($7, avoid_setups),
            pre_market_notes = COALESCE($8, pre_market_notes),
            post_market_notes = COALESCE($9, post_market_notes),
            goals_for_day = COALESCE($10, goals_for_day),
            emotional_state = COALESCE($11, emotional_state),
            plan_followed = COALESCE($12, plan_followed),
            updated_at = NOW()
        WHERE id = $13
        RETURNING *
        "#,
    )
    .bind(&req.market_bias)
    .bind(&req.key_levels)
    .bind(&req.trade_plan)
    .bind(req.max_trades)
    .bind(req.max_loss)
    .bind(&req.focus_setups)
    .bind(&req.avoid_setups)
    .bind(&req.pre_market_notes)
    .bind(&req.post_market_notes)
    .bind(&req.goals_for_day)
    .bind(&req.emotional_state)
    .bind(req.plan_followed)
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
    let result = sqlx::query(
        r#"
        DELETE FROM daily_plans WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(plan_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Daily plan not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Plan deleted successfully" })))
}

pub async fn add_watchlist_item(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(plan_id): Path<Uuid>,
    Json(req): Json<CreateWatchlistItemRequest>,
) -> AppResult<Json<WatchlistItem>> {
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(SELECT 1 FROM daily_plans WHERE id = $1 AND user_id = $2)
        "#,
    )
    .bind(plan_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Daily plan not found".to_string()))?;

    let item = sqlx::query_as::<_, WatchlistItem>(
        r#"
        INSERT INTO watchlist_items (
            daily_plan_id, symbol, entry_price, stop_loss,
            target_price, setup_type, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(plan_id)
    .bind(&req.symbol.to_uppercase())
    .bind(req.entry_price)
    .bind(req.stop_loss)
    .bind(req.target_price)
    .bind(&req.setup_type)
    .bind(&req.notes)
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
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM watchlist_items wi
            JOIN daily_plans dp ON wi.daily_plan_id = dp.id
            WHERE wi.id = $1 AND wi.daily_plan_id = $2 AND dp.user_id = $3
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

    let item = sqlx::query_as::<_, WatchlistItem>(
        r#"
        UPDATE watchlist_items SET
            symbol = COALESCE($1, symbol),
            entry_price = COALESCE($2, entry_price),
            stop_loss = COALESCE($3, stop_loss),
            target_price = COALESCE($4, target_price),
            setup_type = COALESCE($5, setup_type),
            notes = COALESCE($6, notes),
            triggered = COALESCE($7, triggered),
            updated_at = NOW()
        WHERE id = $8
        RETURNING *
        "#,
    )
    .bind(&req.symbol)
    .bind(req.entry_price)
    .bind(req.stop_loss)
    .bind(req.target_price)
    .bind(&req.setup_type)
    .bind(&req.notes)
    .bind(req.triggered)
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
        WHERE id = $1 AND daily_plan_id = $2
        AND daily_plan_id IN (SELECT id FROM daily_plans WHERE user_id = $3)
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
