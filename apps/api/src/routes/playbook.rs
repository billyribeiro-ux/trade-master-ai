use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CreatePlaybookEntryRequest, PlaybookEntry, PlaybookPerformance,
    UpdatePlaybookEntryRequest, validate_playbook_entry,
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
    Json(req): Json<CreatePlaybookEntryRequest>,
) -> AppResult<Json<PlaybookEntry>> {
    validate_playbook_entry(&req).map_err(|e| AppError::Validation(e))?;

    let name = req.name.trim().to_string();

    let entry = sqlx::query_as::<_, PlaybookEntry>(
        r#"
        INSERT INTO playbook_entries (
            user_id, name, description, asset_classes, timeframes,
            entry_criteria, exit_criteria, risk_rules,
            min_conviction, max_risk_percent,
            ideal_market_conditions, avoid_conditions
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(&name)
    .bind(&req.description)
    .bind(&req.asset_classes)
    .bind(&req.timeframes)
    .bind(&req.entry_criteria)
    .bind(&req.exit_criteria)
    .bind(&req.risk_rules)
    .bind(&req.min_conviction)
    .bind(req.max_risk_percent)
    .bind(&req.ideal_market_conditions)
    .bind(&req.avoid_conditions)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!("A playbook entry named '{}' already exists", name))
        }
        _ => AppError::from(e),
    })?;

    tracing::info!(
        entry_id = %entry.id,
        name = %entry.name,
        "Playbook entry created"
    );

    Ok(Json(entry))
}

pub async fn get_playbook_entry(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(entry_id): Path<Uuid>,
) -> AppResult<Json<PlaybookEntry>> {
    let entry = sqlx::query_as::<_, PlaybookEntry>(
        "SELECT * FROM playbook_entries WHERE id = $1 AND user_id = $2",
    )
    .bind(entry_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Playbook entry not found".to_string()))?;

    Ok(Json(entry))
}

pub async fn list_playbook_entries(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<PlaybookEntry>>> {
    let entries = sqlx::query_as::<_, PlaybookEntry>(
        r#"
        SELECT * FROM playbook_entries
        WHERE user_id = $1
        ORDER BY is_active DESC, name ASC
        "#,
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
    Json(req): Json<UpdatePlaybookEntryRequest>,
) -> AppResult<Json<PlaybookEntry>> {
    // Validate name length if provided
    if let Some(ref name) = req.name {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed.len() > 200 {
            return Err(AppError::Validation(
                "Name must be between 1 and 200 characters".to_string(),
            ));
        }
    }

    // Verify ownership
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM playbook_entries WHERE id = $1 AND user_id = $2)",
    )
    .bind(entry_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Playbook entry not found".to_string()))?;

    let entry = sqlx::query_as::<_, PlaybookEntry>(
        r#"
        UPDATE playbook_entries SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            asset_classes = COALESCE($3, asset_classes),
            timeframes = COALESCE($4, timeframes),
            entry_criteria = COALESCE($5, entry_criteria),
            exit_criteria = COALESCE($6, exit_criteria),
            risk_rules = COALESCE($7, risk_rules),
            min_conviction = COALESCE($8, min_conviction),
            max_risk_percent = COALESCE($9, max_risk_percent),
            ideal_market_conditions = COALESCE($10, ideal_market_conditions),
            avoid_conditions = COALESCE($11, avoid_conditions),
            is_active = COALESCE($12, is_active),
            updated_at = NOW()
        WHERE id = $13
        RETURNING *
        "#,
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.asset_classes)
    .bind(&req.timeframes)
    .bind(&req.entry_criteria)
    .bind(&req.exit_criteria)
    .bind(&req.risk_rules)
    .bind(&req.min_conviction)
    .bind(req.max_risk_percent)
    .bind(&req.ideal_market_conditions)
    .bind(&req.avoid_conditions)
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
        "DELETE FROM playbook_entries WHERE id = $1 AND user_id = $2",
    )
    .bind(entry_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Playbook entry not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Playbook entry deleted" })))
}

/// Returns performance statistics for each playbook setup, computed from
/// closed trades that match the setup name.
pub async fn get_playbook_performance(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<PlaybookPerformance>>> {
    let performance = sqlx::query_as::<_, PlaybookPerformance>(
        r#"
        SELECT
            pe.name as setup_name,
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
        FROM playbook_entries pe
        LEFT JOIN trades t
            ON t.user_id = pe.user_id
            AND t.setup_name = pe.name
            AND t.status = 'closed'
        WHERE pe.user_id = $1 AND pe.is_active = true
        GROUP BY pe.name
        ORDER BY total_pnl DESC
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(performance))
}
