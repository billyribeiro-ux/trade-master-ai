use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Matches `ai_reviews` table from migration 008.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AiReview {
    pub id: Uuid,
    pub user_id: Uuid,
    pub trade_id: Option<Uuid>,
    pub review_type: Option<String>,
    pub overall_score: Option<Decimal>,
    pub execution_quality_score: Option<Decimal>,
    pub risk_management_score: Option<Decimal>,
    pub plan_adherence_score: Option<Decimal>,
    pub thesis_alignment_score: Option<Decimal>,
    pub strengths: Option<Vec<String>>,
    pub weaknesses: Option<Vec<String>>,
    pub key_lesson: Option<String>,
    pub actionable_fixes: Option<Vec<String>>,
    pub alternative_scenario: Option<String>,
    pub chart_analysis: Option<String>,
    pub emotional_state_detected: Option<String>,
    pub raw_response: Option<String>,
    pub tokens_used: Option<i32>,
    pub cost_usd: Option<Decimal>,
    pub prompt_version: Option<String>,
    pub model_used: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Matches `ai_review_messages` table from migration 008.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AiReviewMessage {
    pub id: Uuid,
    pub review_id: Uuid,
    pub role: String,
    pub content: String,
    pub tokens_used: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAiReviewRequest {
    pub trade_id: Option<Uuid>,
    pub review_type: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AiReviewResponse {
    pub review: AiReview,
    pub messages: Vec<AiReviewMessage>,
}

// --- Claude API types (not DB-mapped) ---

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ClaudeRequest {
    pub model: String,
    pub max_tokens: i32,
    pub messages: Vec<ClaudeMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    pub content: Vec<ClaudeContent>,
    pub usage: ClaudeUsage,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeUsage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}
