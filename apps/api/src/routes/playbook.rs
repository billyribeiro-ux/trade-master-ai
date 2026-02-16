use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CreatePlaybookSetupRequest, PlaybookSetup, PlaybookPerformance,
    UpdatePlaybookSetupRequest, validate_playbook_setup,
};
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_playbook_entry(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Json(req): Json<CreatePlaybookSetupRequest>,
) -> AppResult<Json<PlaybookSetup>> {
    validate_playbook_setup(&req).map_err(|e| AppError::Validation(e))?;

    let name = req.name.trim().to_string();

    let entry = sqlx::query_as::<_, PlaybookSetup>(
        r#"
        INSERT INTO playbook_setups (
            user_id, name, description, criteria,
            expected_r_min, expected_r_max, min_conviction,
            preferred_timeframe, market_regimes, common_mistakes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(&name)
    .bind(&req.description)
    .bind(&req.criteria)
    .bind(req.expected_r_min)
    .bind(req.expected_r_max)
    .bind(req.min_conviction)
    .bind(&req.preferred_timeframe)
    .bind(&req.market_regimes)
    .bind(&req.common_mistakes)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!("A playbook setup named '{}' already exists", name))
        }
        _ => AppError::from(e),
    })?;

    tracing::info!(setup_id = %entry.id, name = %entry.name, "Playbook setup created");
    Ok(Json(entry))
}

pub async fn get_playbook_entry(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(entry_id): Path<Uuid>,
) -> AppResult<Json<PlaybookSetup>> {
    let entry = sqlx::query_as::<_, PlaybookSetup>(
        "SELECT * FROM playbook_setups WHERE id = $1 AND user_id = $2",
    )
    .bind(entry_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Playbook setup not found".to_string()))?;

    Ok(Json(entry))
}

pub async fn list_playbook_entries(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<PlaybookSetup>>> {
    let entries = sqlx::query_as::<_, PlaybookSetup>(
        "SELECT * FROM playbook_setups WHERE user_id = $1 ORDER BY is_active DESC, name ASC",
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(entries))
}

pub async fn update_playbook_entry(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(entry_id): Path<Uuid>,
    Json(req): Json<UpdatePlaybookSetupRequest>,
) -> AppResult<Json<PlaybookSetup>> {
    if let Some(ref name) = req.name {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed.len() > 255 {
            return Err(AppError::Validation(
                "Name must be between 1 and 255 characters".to_string(),
            ));
        }
    }

    if let Some(conviction) = req.min_conviction {
        if !(1..=5).contains(&conviction) {
            return Err(AppError::Validation(
                "min_conviction must be between 1 and 5".to_string(),
            ));
        }
    }

    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM playbook_setups WHERE id = $1 AND user_id = $2)",
    )
    .bind(entry_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Playbook setup not found".to_string()))?;

    let entry = sqlx::query_as::<_, PlaybookSetup>(
        r#"
        UPDATE playbook_setups SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            criteria = COALESCE($3, criteria),
            expected_r_min = COALESCE($4, expected_r_min),
            expected_r_max = COALESCE($5, expected_r_max),
            min_conviction = COALESCE($6, min_conviction),
            preferred_timeframe = COALESCE($7, preferred_timeframe),
            market_regimes = COALESCE($8, market_regimes),
            common_mistakes = COALESCE($9, common_mistakes),
            is_active = COALESCE($10, is_active),
            updated_at = NOW()
        WHERE id = $11
        RETURNING *
        "#,
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.criteria)
    .bind(req.expected_r_min)
    .bind(req.expected_r_max)
    .bind(req.min_conviction)
    .bind(&req.preferred_timeframe)
    .bind(&req.market_regimes)
    .bind(&req.common_mistakes)
    .bind(req.is_active)
    .bind(entry_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(entry))
}

pub async fn delete_playbook_entry(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(entry_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query(
        "DELETE FROM playbook_setups WHERE id = $1 AND user_id = $2",
    )
    .bind(entry_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Playbook setup not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Playbook setup deleted" })))
}

pub async fn get_playbook_performance(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<PlaybookPerformance>>> {
    let performance = sqlx::query_as::<_, PlaybookPerformance>(
        r#"
        SELECT
            ps.name as setup_name,
            COUNT(t.id) as total_trades,
            COUNT(t.id) FILTER (WHERE t.net_pnl > 0) as winning_trades,
            COUNT(t.id) FILTER (WHERE t.net_pnl <= 0) as losing_trades,
            COALESCE(
                CAST(COUNT(t.id) FILTER (WHERE t.net_pnl > 0) AS DECIMAL)
                / NULLIF(COUNT(t.id), 0) * 100,
                0
            ) as win_rate,
            COALESCE(SUM(t.net_pnl), 0) as total_pnl,
            COALESCE(AVG(t.net_pnl), 0) as avg_pnl,
            AVG(t.r_multiple) as avg_r_multiple,
            COALESCE(MAX(t.net_pnl), 0) as largest_win,
            COALESCE(MIN(t.net_pnl), 0) as largest_loss,
            AVG(t.hold_time_minutes)::INTEGER as avg_hold_minutes
        FROM playbook_setups ps
        LEFT JOIN trades t
            ON t.user_id = ps.user_id
            AND t.setup_name = ps.name
            AND t.status = 'closed'
        WHERE ps.user_id = $1 AND ps.is_active = true
        GROUP BY ps.name
        ORDER BY total_pnl DESC
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(performance))
}
