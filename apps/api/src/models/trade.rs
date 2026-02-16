use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "trade_direction", rename_all = "lowercase")]
pub enum TradeDirection {
    Long,
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "trade_status", rename_all = "lowercase")]
pub enum TradeStatus {
    Open,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "asset_class", rename_all = "lowercase")]
pub enum AssetClass {
    Stocks,
    Options,
    Futures,
    Forex,
    Crypto,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "conviction_level", rename_all = "lowercase")]
pub enum ConvictionLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Trade {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub direction: TradeDirection,
    pub asset_class: AssetClass,
    pub status: TradeStatus,
    
    // Entry details
    pub entry_date: DateTime<Utc>,
    pub entry_price: Decimal,
    pub quantity: Decimal,
    pub stop_loss: Option<Decimal>,
    pub take_profit: Option<Decimal>,
    
    // Exit details
    pub exit_date: Option<DateTime<Utc>>,
    pub exit_price: Option<Decimal>,
    pub actual_exit_price: Option<Decimal>,
    
    // P&L and metrics
    pub pnl: Option<Decimal>,
    pub pnl_percent: Option<Decimal>,
    pub commissions: Option<Decimal>,
    pub net_pnl: Option<Decimal>,
    pub r_multiple: Option<Decimal>,
    pub mae: Option<Decimal>,
    pub mfe: Option<Decimal>,
    pub hold_time_minutes: Option<i32>,
    
    // Risk and setup
    pub risk_amount: Option<Decimal>,
    pub risk_percent: Option<Decimal>,
    pub position_size_pct: Option<Decimal>,
    pub conviction: Option<ConvictionLevel>,
    pub setup_name: Option<String>,
    pub timeframe: Option<String>,
    
    // Notes and analysis
    pub thesis: Option<String>,
    pub mistakes: Option<String>,
    pub lessons: Option<String>,
    pub emotional_state: Option<String>,
    pub market_condition: Option<String>,
    
    // Grading
    pub execution_grade: Option<String>,
    pub patience_grade: Option<String>,
    pub discipline_grade: Option<String>,
    pub overall_grade: Option<String>,
    
    // Metadata
    pub is_paper_trade: bool,
    pub is_revenge_trade: bool,
    pub broke_rules: bool,
    pub followed_plan: bool,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTradeRequest {
    pub symbol: String,
    pub direction: TradeDirection,
    pub asset_class: AssetClass,
    pub entry_date: DateTime<Utc>,
    pub entry_price: Decimal,
    pub quantity: Decimal,
    pub stop_loss: Option<Decimal>,
    pub take_profit: Option<Decimal>,
    pub risk_amount: Option<Decimal>,
    pub risk_percent: Option<Decimal>,
    pub position_size_pct: Option<Decimal>,
    pub conviction: Option<ConvictionLevel>,
    pub setup_name: Option<String>,
    pub timeframe: Option<String>,
    pub thesis: Option<String>,
    pub emotional_state: Option<String>,
    pub market_condition: Option<String>,
    pub is_paper_trade: Option<bool>,
    pub commissions: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTradeRequest {
    pub symbol: Option<String>,
    pub direction: Option<TradeDirection>,
    pub asset_class: Option<AssetClass>,
    pub entry_date: Option<DateTime<Utc>>,
    pub entry_price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub stop_loss: Option<Decimal>,
    pub take_profit: Option<Decimal>,
    pub exit_date: Option<DateTime<Utc>>,
    pub exit_price: Option<Decimal>,
    pub actual_exit_price: Option<Decimal>,
    pub risk_amount: Option<Decimal>,
    pub risk_percent: Option<Decimal>,
    pub position_size_pct: Option<Decimal>,
    pub conviction: Option<ConvictionLevel>,
    pub setup_name: Option<String>,
    pub timeframe: Option<String>,
    pub thesis: Option<String>,
    pub mistakes: Option<String>,
    pub lessons: Option<String>,
    pub emotional_state: Option<String>,
    pub market_condition: Option<String>,
    pub execution_grade: Option<String>,
    pub patience_grade: Option<String>,
    pub discipline_grade: Option<String>,
    pub overall_grade: Option<String>,
    pub is_paper_trade: Option<bool>,
    pub is_revenge_trade: Option<bool>,
    pub broke_rules: Option<bool>,
    pub followed_plan: Option<bool>,
    pub commissions: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct CloseTradeRequest {
    pub exit_date: DateTime<Utc>,
    pub exit_price: Decimal,
    pub actual_exit_price: Option<Decimal>,
    pub mistakes: Option<String>,
    pub lessons: Option<String>,
    pub execution_grade: Option<String>,
    pub patience_grade: Option<String>,
    pub discipline_grade: Option<String>,
    pub overall_grade: Option<String>,
    pub broke_rules: Option<bool>,
    pub followed_plan: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TradeWithDetails {
    #[serde(flatten)]
    pub trade: Trade,
    pub tags: Vec<TradeTag>,
    pub legs: Vec<TradeLeg>,
    pub media: Vec<TradeMedia>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TradeLeg {
    pub id: Uuid,
    pub trade_id: Uuid,
    pub leg_number: i32,
    pub action: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTradeLegRequest {
    pub action: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TradeMedia {
    pub id: Uuid,
    pub trade_id: Uuid,
    pub media_type: String,
    pub s3_key: String,
    pub s3_url: String,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub caption: Option<String>,
    pub annotations: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TradeTag {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TradeFilters {
    pub status: Option<TradeStatus>,
    pub direction: Option<TradeDirection>,
    pub asset_class: Option<AssetClass>,
    pub symbol: Option<String>,
    pub setup_name: Option<String>,
    pub conviction: Option<ConvictionLevel>,
    pub is_paper_trade: Option<bool>,
    pub is_revenge_trade: Option<bool>,
    pub broke_rules: Option<bool>,
    pub followed_plan: Option<bool>,
    pub tag_ids: Option<Vec<Uuid>>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub min_pnl: Option<Decimal>,
    pub max_pnl: Option<Decimal>,
    pub min_r_multiple: Option<Decimal>,
    pub max_r_multiple: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct TradeListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    #[serde(flatten)]
    pub filters: TradeFilters,
}

#[derive(Debug, Serialize)]
pub struct TradeListResponse {
    pub trades: Vec<Trade>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct TradeStats {
    pub total_trades: i64,
    pub winning_trades: i64,
    pub losing_trades: i64,
    pub win_rate: Decimal,
    pub total_pnl: Decimal,
    pub avg_win: Decimal,
    pub avg_loss: Decimal,
    pub profit_factor: Option<Decimal>,
    pub avg_r_multiple: Option<Decimal>,
    pub largest_win: Decimal,
    pub largest_loss: Decimal,
    pub avg_hold_time_minutes: Option<i32>,
}
