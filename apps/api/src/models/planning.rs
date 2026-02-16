use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Matches `daily_plans` table from migration 007.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct DailyPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_date: NaiveDate,
    pub market_bias: Option<String>,
    pub bias_reasoning: Option<String>,
    pub session_goals: Option<Vec<String>>,
    pub max_trades: Option<i32>,
    pub max_daily_loss: Option<Decimal>,
    pub checklist_items: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub ai_plan_of_attack: Option<String>,
    pub adherence_score: Option<Decimal>,
    pub adherence_details: Option<serde_json::Value>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDailyPlanRequest {
    pub plan_date: NaiveDate,
    pub market_bias: Option<String>,
    pub bias_reasoning: Option<String>,
    pub session_goals: Option<Vec<String>>,
    pub max_trades: Option<i32>,
    pub max_daily_loss: Option<Decimal>,
    pub checklist_items: Option<serde_json::Value>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDailyPlanRequest {
    pub market_bias: Option<String>,
    pub bias_reasoning: Option<String>,
    pub session_goals: Option<Vec<String>>,
    pub max_trades: Option<i32>,
    pub max_daily_loss: Option<Decimal>,
    pub checklist_items: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub completed: Option<bool>,
}

/// Matches `watchlist_items` table from migration 007.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct WatchlistItem {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub symbol: String,
    pub key_levels: Option<serde_json::Value>,
    pub catalysts: Option<String>,
    pub setup_description: Option<String>,
    pub risk_reward_ratio: Option<Decimal>,
    pub position_size_suggested: Option<Decimal>,
    pub was_traded: bool,
    pub outcome: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWatchlistItemRequest {
    pub symbol: String,
    pub key_levels: Option<serde_json::Value>,
    pub catalysts: Option<String>,
    pub setup_description: Option<String>,
    pub risk_reward_ratio: Option<Decimal>,
    pub position_size_suggested: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWatchlistItemRequest {
    pub symbol: Option<String>,
    pub key_levels: Option<serde_json::Value>,
    pub catalysts: Option<String>,
    pub setup_description: Option<String>,
    pub risk_reward_ratio: Option<Decimal>,
    pub position_size_suggested: Option<Decimal>,
    pub was_traded: Option<bool>,
    pub outcome: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DailyPlanWithWatchlist {
    #[serde(flatten)]
    pub plan: DailyPlan,
    pub watchlist: Vec<WatchlistItem>,
}
