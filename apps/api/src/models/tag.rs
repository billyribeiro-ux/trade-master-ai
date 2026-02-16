use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Tag {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub color: String,
    pub category: Option<String>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: String,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct TagWithCount {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub color: String,
    pub category: Option<String>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub trade_count: i64,
}
