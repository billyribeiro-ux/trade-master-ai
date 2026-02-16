use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub trading_style: Option<String>,
    pub primary_assets: Option<Vec<String>>,
    pub experience_level: Option<String>,
    pub account_size_range: Option<String>,
    pub default_risk_pct: Option<rust_decimal::Decimal>,
    pub timezone: Option<String>,
    pub active_sessions: Option<Vec<String>>,
    pub goals: Option<Vec<String>>,
    pub ai_personality: Option<String>,
    pub max_trades_per_day: Option<i32>,
    pub max_daily_loss: Option<rust_decimal::Decimal>,
    pub default_commissions: Option<rust_decimal::Decimal>,
    pub onboarding_completed: bool,
    pub onboarding_completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub trading_style: Option<String>,
    pub primary_assets: Option<Vec<String>>,
    pub experience_level: Option<String>,
    pub account_size_range: Option<String>,
    pub default_risk_pct: Option<rust_decimal::Decimal>,
    pub timezone: Option<String>,
    pub active_sessions: Option<Vec<String>>,
    pub goals: Option<Vec<String>>,
    pub ai_personality: Option<String>,
    pub max_trades_per_day: Option<i32>,
    pub max_daily_loss: Option<rust_decimal::Decimal>,
    pub default_commissions: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Serialize)]
pub struct UserWithProfile {
    #[serde(flatten)]
    pub user: User,
    pub profile: Option<UserProfile>,
}
