use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CreateMoodLogRequest, MoodLog, PsychologyInsights, UpdateMoodLogRequest,
    validate_mood_score,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

const MAX_JOURNAL_LENGTH: usize = 20_000;
const MAX_EMOTIONAL_LABELS: usize = 10;
const MAX_LABEL_LENGTH: usize = 50;

/// Validates all fields on a mood log creation request.
fn validate_mood_log(req: &CreateMoodLogRequest) -> AppResult<()> {
    if let Some(score) = req.pre_market_mood {
        validate_mood_score(score, "pre_market_mood")
            .map_err(|e| AppError::Validation(e))?;
    }
    if let Some(score) = req.during_trading_mood {
        validate_mood_score(score, "during_trading_mood")
            .map_err(|e| AppError::Validation(e))?;
    }
    if let Some(score) = req.post_market_mood {
        validate_mood_score(score, "post_market_mood")
            .map_err(|e| AppError::Validation(e))?;
    }
    if let Some(score) = req.sleep_quality {
        validate_mood_score(score, "sleep_quality")
            .map_err(|e| AppError::Validation(e))?;
    }
    if let Some(hours) = req.sleep_hours {
        if hours < Decimal::ZERO || hours > Decimal::from(24) {
            return Err(AppError::Validation(
                "sleep_hours must be between 0 and 24".to_string(),
            ));
        }
    }
    if let Some(ref journal) = req.journal_entry {
        if journal.len() > MAX_JOURNAL_LENGTH {
            return Err(AppError::Validation(format!(
                "Journal entry must be {} characters or fewer",
                MAX_JOURNAL_LENGTH
            )));
        }
    }
    if let Some(ref labels) = req.emotional_labels {
        if labels.len() > MAX_EMOTIONAL_LABELS {
            return Err(AppError::Validation(format!(
                "Maximum {} emotional labels allowed",
                MAX_EMOTIONAL_LABELS
            )));
        }
        for label in labels {
            if label.trim().is_empty() || label.len() > MAX_LABEL_LENGTH {
                return Err(AppError::Validation(format!(
                    "Each emotional label must be between 1 and {} characters",
                    MAX_LABEL_LENGTH
                )));
            }
        }
    }
    Ok(())
}

pub async fn create_mood_log(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Json(req): Json<CreateMoodLogRequest>,
) -> AppResult<Json<MoodLog>> {
    validate_mood_log(&req)?;

    // Sanitize emotional labels
    let labels: Option<Vec<String>> = req.emotional_labels.as_ref().map(|l| {
        l.iter().map(|s| s.trim().to_lowercase()).collect()
    });

    let log = sqlx::query_as::<_, MoodLog>(
        r#"
        INSERT INTO mood_logs (
            user_id, log_date, pre_market_mood, during_trading_mood,
            post_market_mood, emotional_labels, sleep_quality, sleep_hours,
            exercised, meditated, journal_entry,
            felt_fomo, felt_revenge, felt_overconfident, felt_fearful
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(req.log_date)
    .bind(req.pre_market_mood)
    .bind(req.during_trading_mood)
    .bind(req.post_market_mood)
    .bind(&labels)
    .bind(req.sleep_quality)
    .bind(req.sleep_hours)
    .bind(req.exercised)
    .bind(req.meditated)
    .bind(&req.journal_entry)
    .bind(req.felt_fomo.unwrap_or(false))
    .bind(req.felt_revenge.unwrap_or(false))
    .bind(req.felt_overconfident.unwrap_or(false))
    .bind(req.felt_fearful.unwrap_or(false))
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("A mood log for this date already exists".to_string())
        }
        _ => AppError::from(e),
    })?;

    tracing::info!(
        log_id = %log.id,
        user_id = %auth_user.user_id,
        date = %log.log_date,
        "Mood log created"
    );

    Ok(Json(log))
}

pub async fn get_mood_log(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(log_id): Path<Uuid>,
) -> AppResult<Json<MoodLog>> {
    let log = sqlx::query_as::<_, MoodLog>(
        "SELECT * FROM mood_logs WHERE id = $1 AND user_id = $2",
    )
    .bind(log_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Mood log not found".to_string()))?;

    Ok(Json(log))
}

#[derive(Debug, Deserialize)]
pub struct MoodLogListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_mood_logs(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Query(query): Query<MoodLogListQuery>,
) -> AppResult<Json<Vec<MoodLog>>> {
    let limit = query.limit.unwrap_or(30).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);

    let logs = sqlx::query_as::<_, MoodLog>(
        r#"
        SELECT * FROM mood_logs
        WHERE user_id = $1
        ORDER BY log_date DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(auth_user.user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(logs))
}

pub async fn update_mood_log(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(log_id): Path<Uuid>,
    Json(req): Json<UpdateMoodLogRequest>,
) -> AppResult<Json<MoodLog>> {
    // Validate scores if provided
    if let Some(score) = req.pre_market_mood {
        validate_mood_score(score, "pre_market_mood")
            .map_err(|e| AppError::Validation(e))?;
    }
    if let Some(score) = req.during_trading_mood {
        validate_mood_score(score, "during_trading_mood")
            .map_err(|e| AppError::Validation(e))?;
    }
    if let Some(score) = req.post_market_mood {
        validate_mood_score(score, "post_market_mood")
            .map_err(|e| AppError::Validation(e))?;
    }
    if let Some(score) = req.sleep_quality {
        validate_mood_score(score, "sleep_quality")
            .map_err(|e| AppError::Validation(e))?;
    }

    // Verify ownership
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM mood_logs WHERE id = $1 AND user_id = $2)",
    )
    .bind(log_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Mood log not found".to_string()))?;

    let log = sqlx::query_as::<_, MoodLog>(
        r#"
        UPDATE mood_logs SET
            pre_market_mood = COALESCE($1, pre_market_mood),
            during_trading_mood = COALESCE($2, during_trading_mood),
            post_market_mood = COALESCE($3, post_market_mood),
            emotional_labels = COALESCE($4, emotional_labels),
            sleep_quality = COALESCE($5, sleep_quality),
            sleep_hours = COALESCE($6, sleep_hours),
            exercised = COALESCE($7, exercised),
            meditated = COALESCE($8, meditated),
            journal_entry = COALESCE($9, journal_entry),
            felt_fomo = COALESCE($10, felt_fomo),
            felt_revenge = COALESCE($11, felt_revenge),
            felt_overconfident = COALESCE($12, felt_overconfident),
            felt_fearful = COALESCE($13, felt_fearful),
            updated_at = NOW()
        WHERE id = $14
        RETURNING *
        "#,
    )
    .bind(req.pre_market_mood)
    .bind(req.during_trading_mood)
    .bind(req.post_market_mood)
    .bind(&req.emotional_labels)
    .bind(req.sleep_quality)
    .bind(req.sleep_hours)
    .bind(req.exercised)
    .bind(req.meditated)
    .bind(&req.journal_entry)
    .bind(req.felt_fomo)
    .bind(req.felt_revenge)
    .bind(req.felt_overconfident)
    .bind(req.felt_fearful)
    .bind(log_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(log))
}

pub async fn delete_mood_log(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(log_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query(
        "DELETE FROM mood_logs WHERE id = $1 AND user_id = $2",
    )
    .bind(log_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Mood log not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Mood log deleted" })))
}

pub async fn get_psychology_insights(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<PsychologyInsights>> {
    // Aggregate mood statistics
    let insights = sqlx::query_as::<_, PsychologyInsights>(
        r#"
        WITH mood_data AS (
            SELECT
                ml.*,
                COALESCE(
                    (SELECT SUM(net_pnl) FROM trades t
                     WHERE t.user_id = ml.user_id
                       AND t.status = 'closed'
                       AND DATE(t.exit_date) = DATE(ml.log_date)),
                    0
                ) as day_pnl_calc
            FROM mood_logs ml
            WHERE ml.user_id = $1
        )
        SELECT
            AVG(pre_market_mood)::DECIMAL as avg_pre_market_mood,
            AVG(during_trading_mood)::DECIMAL as avg_during_mood,
            AVG(post_market_mood)::DECIMAL as avg_post_mood,
            AVG(sleep_quality)::DECIMAL as avg_sleep_quality,
            AVG(sleep_hours) as avg_sleep_hours,
            COUNT(*) FILTER (WHERE felt_fomo) as fomo_count,
            COUNT(*) FILTER (WHERE felt_revenge) as revenge_count,
            COUNT(*) FILTER (WHERE felt_overconfident) as overconfident_count,
            COUNT(*) FILTER (WHERE felt_fearful) as fearful_count,
            COUNT(*) as total_logs,
            AVG(day_pnl_calc) FILTER (WHERE pre_market_mood >= 7) as high_mood_avg_pnl,
            AVG(day_pnl_calc) FILTER (WHERE pre_market_mood < 7 AND pre_market_mood IS NOT NULL) as low_mood_avg_pnl,
            AVG(day_pnl_calc) FILTER (WHERE sleep_quality >= 7) as good_sleep_avg_pnl,
            AVG(day_pnl_calc) FILTER (WHERE sleep_quality < 7 AND sleep_quality IS NOT NULL) as poor_sleep_avg_pnl
        FROM mood_data
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(insights))
}
