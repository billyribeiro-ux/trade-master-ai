use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CreateMoodLogRequest, MoodLog, PsychologyInsights, UpdateMoodLogRequest,
    validate_mood_score,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

const MAX_NOTES_LENGTH: usize = 20_000;
const MAX_EMOTIONS: usize = 10;

fn validate_mood_log(req: &CreateMoodLogRequest) -> AppResult<()> {
    if let Some(s) = req.pre_market_mood {
        validate_mood_score(s, "pre_market_mood").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.post_market_mood {
        validate_mood_score(s, "post_market_mood").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.stress_level {
        validate_mood_score(s, "stress_level").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.confidence_level {
        validate_mood_score(s, "confidence_level").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.sleep_quality {
        validate_mood_score(s, "sleep_quality").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(ref notes) = req.notes {
        if notes.len() > MAX_NOTES_LENGTH {
            return Err(AppError::Validation(format!(
                "Notes must be {} characters or fewer", MAX_NOTES_LENGTH
            )));
        }
    }
    if let Some(ref emotions) = req.emotions {
        if emotions.len() > MAX_EMOTIONS {
            return Err(AppError::Validation(format!(
                "Maximum {} emotions allowed", MAX_EMOTIONS
            )));
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

    let emotions: Option<Vec<String>> = req.emotions.as_ref().map(|e| {
        e.iter().map(|s| s.trim().to_lowercase()).collect()
    });

    let log = sqlx::query_as::<_, MoodLog>(
        r#"
        INSERT INTO mood_logs (
            user_id, log_date, pre_market_mood, post_market_mood,
            stress_level, confidence_level, sleep_quality,
            emotions, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(req.log_date)
    .bind(req.pre_market_mood)
    .bind(req.post_market_mood)
    .bind(req.stress_level)
    .bind(req.confidence_level)
    .bind(req.sleep_quality)
    .bind(&emotions)
    .bind(&req.notes)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("A mood log for this date already exists".to_string())
        }
        _ => AppError::from(e),
    })?;

    tracing::info!(log_id = %log.id, date = %log.log_date, "Mood log created");
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
        "SELECT * FROM mood_logs WHERE user_id = $1 ORDER BY log_date DESC LIMIT $2 OFFSET $3",
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
    if let Some(s) = req.pre_market_mood {
        validate_mood_score(s, "pre_market_mood").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.post_market_mood {
        validate_mood_score(s, "post_market_mood").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.stress_level {
        validate_mood_score(s, "stress_level").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.confidence_level {
        validate_mood_score(s, "confidence_level").map_err(|e| AppError::Validation(e))?;
    }
    if let Some(s) = req.sleep_quality {
        validate_mood_score(s, "sleep_quality").map_err(|e| AppError::Validation(e))?;
    }

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
            post_market_mood = COALESCE($2, post_market_mood),
            stress_level = COALESCE($3, stress_level),
            confidence_level = COALESCE($4, confidence_level),
            sleep_quality = COALESCE($5, sleep_quality),
            emotions = COALESCE($6, emotions),
            notes = COALESCE($7, notes),
            updated_at = NOW()
        WHERE id = $8
        RETURNING *
        "#,
    )
    .bind(req.pre_market_mood)
    .bind(req.post_market_mood)
    .bind(req.stress_level)
    .bind(req.confidence_level)
    .bind(req.sleep_quality)
    .bind(&req.emotions)
    .bind(&req.notes)
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
    let result = sqlx::query("DELETE FROM mood_logs WHERE id = $1 AND user_id = $2")
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
    let insights = sqlx::query_as::<_, PsychologyInsights>(
        r#"
        WITH mood_data AS (
            SELECT
                ml.*,
                COALESCE(
                    (SELECT SUM(net_pnl) FROM trades t
                     WHERE t.user_id = ml.user_id
                       AND t.status = 'closed'
                       AND DATE(t.exit_date) = ml.log_date),
                    0
                ) as day_pnl_calc
            FROM mood_logs ml
            WHERE ml.user_id = $1
        )
        SELECT
            AVG(pre_market_mood)::DECIMAL as avg_pre_market_mood,
            AVG(post_market_mood)::DECIMAL as avg_post_market_mood,
            AVG(stress_level)::DECIMAL as avg_stress_level,
            AVG(confidence_level)::DECIMAL as avg_confidence_level,
            AVG(sleep_quality)::DECIMAL as avg_sleep_quality,
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
