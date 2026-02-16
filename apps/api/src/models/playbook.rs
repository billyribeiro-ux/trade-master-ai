use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A playbook entry defines a specific trading setup with rules, criteria,
/// and grading rubrics. Traders build a library of setups they trade.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PlaybookEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub asset_classes: Option<Vec<String>>,
    pub timeframes: Option<Vec<String>>,

    /// Entry criteria checklist items
    pub entry_criteria: Option<Vec<String>>,
    /// Exit criteria checklist items
    pub exit_criteria: Option<Vec<String>>,
    /// Risk management rules specific to this setup
    pub risk_rules: Option<Vec<String>>,

    /// Example trade IDs for reference
    pub example_trade_ids: Option<Vec<Uuid>>,

    /// Minimum conviction level to take this setup
    pub min_conviction: Option<String>,
    /// Maximum risk percent for this setup
    pub max_risk_percent: Option<Decimal>,
    /// Ideal market conditions
    pub ideal_market_conditions: Option<Vec<String>>,
    /// Conditions to avoid
    pub avoid_conditions: Option<Vec<String>>,

    /// Whether this setup is currently active in the playbook
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlaybookEntryRequest {
    pub name: String,
    pub description: Option<String>,
    pub asset_classes: Option<Vec<String>>,
    pub timeframes: Option<Vec<String>>,
    pub entry_criteria: Option<Vec<String>>,
    pub exit_criteria: Option<Vec<String>>,
    pub risk_rules: Option<Vec<String>>,
    pub min_conviction: Option<String>,
    pub max_risk_percent: Option<Decimal>,
    pub ideal_market_conditions: Option<Vec<String>>,
    pub avoid_conditions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlaybookEntryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub asset_classes: Option<Vec<String>>,
    pub timeframes: Option<Vec<String>>,
    pub entry_criteria: Option<Vec<String>>,
    pub exit_criteria: Option<Vec<String>>,
    pub risk_rules: Option<Vec<String>>,
    pub min_conviction: Option<String>,
    pub max_risk_percent: Option<Decimal>,
    pub ideal_market_conditions: Option<Vec<String>>,
    pub avoid_conditions: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

/// Performance statistics for a specific playbook entry, computed from
/// closed trades that reference this setup name.
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

const MAX_NAME_LENGTH: usize = 200;
const MAX_DESCRIPTION_LENGTH: usize = 10_000;
const MAX_CRITERIA_ITEMS: usize = 30;
const MAX_CRITERIA_ITEM_LENGTH: usize = 500;

/// Validates a playbook entry creation request.
pub fn validate_playbook_entry(req: &CreatePlaybookEntryRequest) -> Result<(), String> {
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

    validate_string_list(&req.entry_criteria, "entry_criteria")?;
    validate_string_list(&req.exit_criteria, "exit_criteria")?;
    validate_string_list(&req.risk_rules, "risk_rules")?;
    validate_string_list(&req.ideal_market_conditions, "ideal_market_conditions")?;
    validate_string_list(&req.avoid_conditions, "avoid_conditions")?;

    if let Some(ref max_risk) = req.max_risk_percent {
        if *max_risk <= Decimal::ZERO || *max_risk > Decimal::from(100) {
            return Err("max_risk_percent must be between 0 and 100".to_string());
        }
    }

    Ok(())
}

fn validate_string_list(
    list: &Option<Vec<String>>,
    field_name: &str,
) -> Result<(), String> {
    if let Some(items) = list {
        if items.len() > MAX_CRITERIA_ITEMS {
            return Err(format!(
                "{} can have at most {} items",
                field_name, MAX_CRITERIA_ITEMS
            ));
        }
        for item in items {
            if item.trim().is_empty() || item.len() > MAX_CRITERIA_ITEM_LENGTH {
                return Err(format!(
                    "Each {} item must be between 1 and {} characters",
                    field_name, MAX_CRITERIA_ITEM_LENGTH
                ));
            }
        }
    }
    Ok(())
}
