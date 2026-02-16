use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Matches `mood_logs` table from migration 009.
/// Columns: pre_market_mood, post_market_mood, stress_level, confidence_level,
/// sleep_quality, emotions TEXT[], notes.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct MoodLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub log_date: NaiveDate,
    pub pre_market_mood: Option<i32>,
    pub post_market_mood: Option<i32>,
    pub stress_level: Option<i32>,
    pub confidence_level: Option<i32>,
    pub sleep_quality: Option<i32>,
    pub emotions: Option<Vec<String>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMoodLogRequest {
    pub log_date: NaiveDate,
    pub pre_market_mood: Option<i32>,
    pub post_market_mood: Option<i32>,
    pub stress_level: Option<i32>,
    pub confidence_level: Option<i32>,
    pub sleep_quality: Option<i32>,
    pub emotions: Option<Vec<String>>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMoodLogRequest {
    pub pre_market_mood: Option<i32>,
    pub post_market_mood: Option<i32>,
    pub stress_level: Option<i32>,
    pub confidence_level: Option<i32>,
    pub sleep_quality: Option<i32>,
    pub emotions: Option<Vec<String>>,
    pub notes: Option<String>,
}

/// Aggregated psychology insights computed from mood_logs + trades.
#[derive(Debug, Serialize, FromRow)]
pub struct PsychologyInsights {
    pub avg_pre_market_mood: Option<Decimal>,
    pub avg_post_market_mood: Option<Decimal>,
    pub avg_stress_level: Option<Decimal>,
    pub avg_confidence_level: Option<Decimal>,
    pub avg_sleep_quality: Option<Decimal>,
    pub total_logs: i64,
    pub high_mood_avg_pnl: Option<Decimal>,
    pub low_mood_avg_pnl: Option<Decimal>,
    pub good_sleep_avg_pnl: Option<Decimal>,
    pub poor_sleep_avg_pnl: Option<Decimal>,
}

/// Validates that a score is within the 1-10 range.
pub fn validate_mood_score(value: i32, field_name: &str) -> Result<(), String> {
    if !(1..=10).contains(&value) {
        return Err(format!("{} must be between 1 and 10", field_name));
    }
    Ok(())
}
