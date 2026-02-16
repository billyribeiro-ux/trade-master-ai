use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct DailyPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_date: DateTime<Utc>,
    pub market_bias: Option<String>,
    pub key_levels: Option<Vec<String>>,
    pub trade_plan: Option<String>,
    pub max_trades: Option<i32>,
    pub max_loss: Option<rust_decimal::Decimal>,
    pub focus_setups: Option<Vec<String>>,
    pub avoid_setups: Option<Vec<String>>,
    pub pre_market_notes: Option<String>,
    pub post_market_notes: Option<String>,
    pub goals_for_day: Option<Vec<String>>,
    pub emotional_state: Option<String>,
    pub plan_followed: Option<bool>,
    pub actual_trades: Option<i32>,
    pub actual_pnl: Option<rust_decimal::Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDailyPlanRequest {
    pub plan_date: DateTime<Utc>,
    pub market_bias: Option<String>,
    pub key_levels: Option<Vec<String>>,
    pub trade_plan: Option<String>,
    pub max_trades: Option<i32>,
    pub max_loss: Option<rust_decimal::Decimal>,
    pub focus_setups: Option<Vec<String>>,
    pub avoid_setups: Option<Vec<String>>,
    pub pre_market_notes: Option<String>,
    pub goals_for_day: Option<Vec<String>>,
    pub emotional_state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDailyPlanRequest {
    pub market_bias: Option<String>,
    pub key_levels: Option<Vec<String>>,
    pub trade_plan: Option<String>,
    pub max_trades: Option<i32>,
    pub max_loss: Option<rust_decimal::Decimal>,
    pub focus_setups: Option<Vec<String>>,
    pub avoid_setups: Option<Vec<String>>,
    pub pre_market_notes: Option<String>,
    pub post_market_notes: Option<String>,
    pub goals_for_day: Option<Vec<String>>,
    pub emotional_state: Option<String>,
    pub plan_followed: Option<bool>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct WatchlistItem {
    pub id: Uuid,
    pub daily_plan_id: Uuid,
    pub symbol: String,
    pub entry_price: Option<rust_decimal::Decimal>,
    pub stop_loss: Option<rust_decimal::Decimal>,
    pub target_price: Option<rust_decimal::Decimal>,
    pub setup_type: Option<String>,
    pub notes: Option<String>,
    pub triggered: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWatchlistItemRequest {
    pub symbol: String,
    pub entry_price: Option<rust_decimal::Decimal>,
    pub stop_loss: Option<rust_decimal::Decimal>,
    pub target_price: Option<rust_decimal::Decimal>,
    pub setup_type: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWatchlistItemRequest {
    pub symbol: Option<String>,
    pub entry_price: Option<rust_decimal::Decimal>,
    pub stop_loss: Option<rust_decimal::Decimal>,
    pub target_price: Option<rust_decimal::Decimal>,
    pub setup_type: Option<String>,
    pub notes: Option<String>,
    pub triggered: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct DailyPlanWithWatchlist {
    #[serde(flatten)]
    pub plan: DailyPlan,
    pub watchlist: Vec<WatchlistItem>,
}
