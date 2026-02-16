use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Matches `playbook_setups` table from migration 010.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PlaybookSetup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub criteria: Option<serde_json::Value>,
    pub expected_r_min: Option<Decimal>,
    pub expected_r_max: Option<Decimal>,
    pub min_conviction: Option<i32>,
    pub preferred_timeframe: Option<String>,
    pub market_regimes: Option<Vec<String>>,
    pub example_screenshots: Option<Vec<String>>,
    pub common_mistakes: Option<String>,
    pub trade_count: i32,
    pub win_rate: Option<Decimal>,
    pub avg_r: Option<Decimal>,
    pub profit_factor: Option<Decimal>,
    pub total_pnl: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlaybookSetupRequest {
    pub name: String,
    pub description: Option<String>,
    pub criteria: Option<serde_json::Value>,
    pub expected_r_min: Option<Decimal>,
    pub expected_r_max: Option<Decimal>,
    pub min_conviction: Option<i32>,
    pub preferred_timeframe: Option<String>,
    pub market_regimes: Option<Vec<String>>,
    pub common_mistakes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlaybookSetupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub criteria: Option<serde_json::Value>,
    pub expected_r_min: Option<Decimal>,
    pub expected_r_max: Option<Decimal>,
    pub min_conviction: Option<i32>,
    pub preferred_timeframe: Option<String>,
    pub market_regimes: Option<Vec<String>>,
    pub common_mistakes: Option<String>,
    pub is_active: Option<bool>,
}

/// Live performance statistics for a playbook setup, computed from trades.
#[derive(Debug, Serialize, FromRow)]
pub struct PlaybookPerformance {
    pub setup_name: String,
    pub total_trades: i64,
    pub winning_trades: i64,
    pub losing_trades: i64,
    pub win_rate: Decimal,
    pub total_pnl: Decimal,
    pub avg_pnl: Decimal,
    pub avg_r_multiple: Option<Decimal>,
    pub largest_win: Decimal,
    pub largest_loss: Decimal,
    pub avg_hold_minutes: Option<i32>,
}

const MAX_NAME_LENGTH: usize = 255;
const MAX_DESCRIPTION_LENGTH: usize = 10_000;

pub fn validate_playbook_setup(req: &CreatePlaybookSetupRequest) -> Result<(), String> {
    let name = req.name.trim();
    if name.is_empty() || name.len() > MAX_NAME_LENGTH {
        return Err(format!(
            "Name must be between 1 and {} characters",
            MAX_NAME_LENGTH
        ));
    }

    if let Some(ref desc) = req.description {
        if desc.len() > MAX_DESCRIPTION_LENGTH {
            return Err(format!(
                "Description must be {} characters or fewer",
                MAX_DESCRIPTION_LENGTH
            ));
        }
    }

    if let Some(conviction) = req.min_conviction {
        if !(1..=5).contains(&conviction) {
            return Err("min_conviction must be between 1 and 5".to_string());
        }
    }

    if let Some(r_min) = req.expected_r_min {
        if r_min < Decimal::ZERO {
            return Err("expected_r_min must be non-negative".to_string());
        }
    }

    if let Some(r_max) = req.expected_r_max {
        if r_max < Decimal::ZERO {
            return Err("expected_r_max must be non-negative".to_string());
        }
    }

    if let (Some(r_min), Some(r_max)) = (req.expected_r_min, req.expected_r_max) {
        if r_min > r_max {
            return Err("expected_r_min must be <= expected_r_max".to_string());
        }
    }

    Ok(())
}
