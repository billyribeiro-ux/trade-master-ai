use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a single mood/emotional state log entry for a trading day.
/// Tracks the trader's psychological state before, during, and after trading.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct MoodLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub log_date: DateTime<Utc>,

    /// Pre-market emotional state (1-10 scale)
    pub pre_market_mood: Option<i32>,
    /// During-trading emotional state (1-10 scale)
    pub during_trading_mood: Option<i32>,
    /// Post-market emotional state (1-10 scale)
    pub post_market_mood: Option<i32>,

    /// Qualitative labels (e.g., "calm", "anxious", "confident")
    pub emotional_labels: Option<Vec<String>>,

    /// Sleep quality the night before (1-10)
    pub sleep_quality: Option<i32>,
    /// Hours of sleep
    pub sleep_hours: Option<Decimal>,
    /// Exercise before trading
    pub exercised: Option<bool>,
    /// Meditation before trading
    pub meditated: Option<bool>,

    /// Free-form journal entry
    pub journal_entry: Option<String>,

    /// Tilt indicators
    pub felt_fomo: bool,
    pub felt_revenge: bool,
    pub felt_overconfident: bool,
    pub felt_fearful: bool,

    /// Correlation with trading performance (populated by system)
    pub trades_taken: Option<i32>,
    pub day_pnl: Option<Decimal>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMoodLogRequest {
    pub log_date: DateTime<Utc>,
    pub pre_market_mood: Option<i32>,
    pub during_trading_mood: Option<i32>,
    pub post_market_mood: Option<i32>,
    pub emotional_labels: Option<Vec<String>>,
    pub sleep_quality: Option<i32>,
    pub sleep_hours: Option<Decimal>,
    pub exercised: Option<bool>,
    pub meditated: Option<bool>,
    pub journal_entry: Option<String>,
    pub felt_fomo: Option<bool>,
    pub felt_revenge: Option<bool>,
    pub felt_overconfident: Option<bool>,
    pub felt_fearful: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMoodLogRequest {
    pub pre_market_mood: Option<i32>,
    pub during_trading_mood: Option<i32>,
    pub post_market_mood: Option<i32>,
    pub emotional_labels: Option<Vec<String>>,
    pub sleep_quality: Option<i32>,
    pub sleep_hours: Option<Decimal>,
    pub exercised: Option<bool>,
    pub meditated: Option<bool>,
    pub journal_entry: Option<String>,
    pub felt_fomo: Option<bool>,
    pub felt_revenge: Option<bool>,
    pub felt_overconfident: Option<bool>,
    pub felt_fearful: Option<bool>,
}

/// Aggregated psychology insights over a time period.
#[derive(Debug, Serialize)]
pub struct PsychologyInsights {
    pub avg_pre_market_mood: Option<Decimal>,
    pub avg_during_mood: Option<Decimal>,
    pub avg_post_mood: Option<Decimal>,
    pub avg_sleep_quality: Option<Decimal>,
    pub avg_sleep_hours: Option<Decimal>,
    pub fomo_count: i64,
    pub revenge_count: i64,
    pub overconfident_count: i64,
    pub fearful_count: i64,
    pub total_logs: i64,
    /// Correlation: avg P&L on days with high mood (>=7) vs low mood (<7)
    pub high_mood_avg_pnl: Option<Decimal>,
    pub low_mood_avg_pnl: Option<Decimal>,
    /// Correlation: avg P&L on days with good sleep (>=7) vs poor sleep (<7)
    pub good_sleep_avg_pnl: Option<Decimal>,
    pub poor_sleep_avg_pnl: Option<Decimal>,
}

/// Validates that a mood score is within the 1-10 range.
pub fn validate_mood_score(value: i32, field_name: &str) -> Result<(), String> {
    if !(1..=10).contains(&value) {
        return Err(format!("{} must be between 1 and 10", field_name));
    }
    Ok(())
}
