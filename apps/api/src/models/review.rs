use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Matches `periodic_reviews` table from migration 013.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PeriodicReview {
    pub id: Uuid,
    pub user_id: Uuid,
    pub review_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_trades: Option<i32>,
    pub winning_trades: Option<i32>,
    pub losing_trades: Option<i32>,
    pub win_rate: Option<Decimal>,
    pub total_pnl: Option<Decimal>,
    pub avg_r_multiple: Option<Decimal>,
    pub what_went_well: Option<String>,
    pub what_to_improve: Option<String>,
    pub key_lessons: Option<Vec<String>>,
    pub rules_broken: Option<Vec<String>>,
    pub best_trade_id: Option<Uuid>,
    pub worst_trade_id: Option<Uuid>,
    pub goals_met: Option<Vec<String>>,
    pub goals_missed: Option<Vec<String>>,
    pub goals_next_period: Option<Vec<String>>,
    pub discipline_rating: Option<i32>,
    pub patience_rating: Option<i32>,
    pub execution_rating: Option<i32>,
    pub overall_rating: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub review_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub what_went_well: Option<String>,
    pub what_to_improve: Option<String>,
    pub key_lessons: Option<Vec<String>>,
    pub rules_broken: Option<Vec<String>>,
    pub best_trade_id: Option<Uuid>,
    pub worst_trade_id: Option<Uuid>,
    pub goals_met: Option<Vec<String>>,
    pub goals_missed: Option<Vec<String>>,
    pub goals_next_period: Option<Vec<String>>,
    pub discipline_rating: Option<i32>,
    pub patience_rating: Option<i32>,
    pub execution_rating: Option<i32>,
    pub overall_rating: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReviewRequest {
    pub what_went_well: Option<String>,
    pub what_to_improve: Option<String>,
    pub key_lessons: Option<Vec<String>>,
    pub rules_broken: Option<Vec<String>>,
    pub best_trade_id: Option<Uuid>,
    pub worst_trade_id: Option<Uuid>,
    pub goals_met: Option<Vec<String>>,
    pub goals_missed: Option<Vec<String>>,
    pub goals_next_period: Option<Vec<String>>,
    pub discipline_rating: Option<i32>,
    pub patience_rating: Option<i32>,
    pub execution_rating: Option<i32>,
    pub overall_rating: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ReviewWithStats {
    #[serde(flatten)]
    pub review: PeriodicReview,
    pub top_setups: Vec<SetupSummary>,
    pub daily_pnl: Vec<DailyPnl>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SetupSummary {
    pub setup_name: String,
    pub count: i64,
    pub total_pnl: Decimal,
    pub win_rate: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct DailyPnl {
    pub trade_date: NaiveDate,
    pub pnl: Decimal,
    pub trade_count: i64,
}

const MAX_TEXT_LENGTH: usize = 20_000;
const MAX_LIST_ITEMS: usize = 30;

pub fn validate_review_type(review_type: &str) -> Result<&'static str, String> {
    match review_type {
        "weekly" => Ok("weekly"),
        "monthly" => Ok("monthly"),
        "quarterly" => Ok("quarterly"),
        _ => Err(format!(
            "Invalid review type '{}'. Allowed: weekly, monthly, quarterly",
            review_type
        )),
    }
}

pub fn validate_rating(value: i32, field_name: &str) -> Result<(), String> {
    if !(1..=10).contains(&value) {
        return Err(format!("{} must be between 1 and 10", field_name));
    }
    Ok(())
}

pub fn validate_review_request(req: &CreateReviewRequest) -> Result<(), String> {
    validate_review_type(&req.review_type)?;

    if req.period_end <= req.period_start {
        return Err("period_end must be after period_start".to_string());
    }

    if let Some(ref text) = req.what_went_well {
        if text.len() > MAX_TEXT_LENGTH {
            return Err(format!("what_went_well must be {} characters or fewer", MAX_TEXT_LENGTH));
        }
    }
    if let Some(ref text) = req.what_to_improve {
        if text.len() > MAX_TEXT_LENGTH {
            return Err(format!("what_to_improve must be {} characters or fewer", MAX_TEXT_LENGTH));
        }
    }

    for (list, name) in [
        (&req.key_lessons, "key_lessons"),
        (&req.rules_broken, "rules_broken"),
        (&req.goals_met, "goals_met"),
        (&req.goals_missed, "goals_missed"),
        (&req.goals_next_period, "goals_next_period"),
    ] {
        if let Some(items) = list {
            if items.len() > MAX_LIST_ITEMS {
                return Err(format!("{} can have at most {} items", name, MAX_LIST_ITEMS));
            }
        }
    }

    if let Some(r) = req.discipline_rating { validate_rating(r, "discipline_rating")?; }
    if let Some(r) = req.patience_rating { validate_rating(r, "patience_rating")?; }
    if let Some(r) = req.execution_rating { validate_rating(r, "execution_rating")?; }
    if let Some(r) = req.overall_rating { validate_rating(r, "overall_rating")?; }

    Ok(())
}
