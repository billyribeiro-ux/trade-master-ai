use crate::error::{AppError, AppResult};
use crate::models::{
    AiReview, AiReviewMessage, AiReviewResponse, AuthUser, ChatMessageRequest,
    ClaudeMessage, CreateAiReviewRequest, Trade,
};
use crate::services::AiService;
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

const MODEL_NAME: &str = "claude-sonnet-4-20250514";

pub async fn create_ai_review(
    State(pool): State<Arc<PgPool>>,
    State(ai_service): State<Arc<AiService>>,
    auth_user: AuthUser,
    Json(req): Json<CreateAiReviewRequest>,
) -> AppResult<Json<AiReviewResponse>> {
    let prompt = req.prompt.trim();
    if prompt.is_empty() || prompt.len() > 10_000 {
        return Err(AppError::Validation(
            "Prompt must be between 1 and 10,000 characters".to_string(),
        ));
    }

    let (response_text, tokens) = if let Some(trade_id) = req.trade_id {
        let trade = sqlx::query_as::<_, Trade>(
            "SELECT * FROM trades WHERE id = $1 AND user_id = $2",
        )
        .bind(trade_id)
        .bind(auth_user.user_id)
        .fetch_optional(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Trade not found".to_string()))?;

        let text = ai_service.analyze_trade(&trade).await?;
        (text, None)
    } else {
        let (response, tok) = ai_service
            .chat(vec![ClaudeMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }])
            .await?;
        (response, Some(tok))
    };

    // Insert into ai_reviews using actual DB columns from migration 008
    let review = sqlx::query_as::<_, AiReview>(
        r#"
        INSERT INTO ai_reviews (
            user_id, trade_id, review_type,
            raw_response, tokens_used, model_used
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(req.trade_id)
    .bind(req.review_type.as_deref().unwrap_or("trade"))
    .bind(&response_text)
    .bind(tokens)
    .bind(MODEL_NAME)
    .fetch_one(pool.as_ref())
    .await?;

    // Store the conversation as messages
    let user_msg = sqlx::query_as::<_, AiReviewMessage>(
        "INSERT INTO ai_review_messages (review_id, role, content) VALUES ($1, 'user', $2) RETURNING *",
    )
    .bind(review.id)
    .bind(prompt)
    .fetch_one(pool.as_ref())
    .await?;

    let assistant_msg = sqlx::query_as::<_, AiReviewMessage>(
        "INSERT INTO ai_review_messages (review_id, role, content) VALUES ($1, 'assistant', $2) RETURNING *",
    )
    .bind(review.id)
    .bind(&response_text)
    .fetch_one(pool.as_ref())
    .await?;

    tracing::info!(review_id = %review.id, "AI review created");

    Ok(Json(AiReviewResponse {
        review,
        messages: vec![user_msg, assistant_msg],
    }))
}

pub async fn get_ai_review(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(review_id): Path<Uuid>,
) -> AppResult<Json<AiReviewResponse>> {
    let review = sqlx::query_as::<_, AiReview>(
        "SELECT * FROM ai_reviews WHERE id = $1 AND user_id = $2",
    )
    .bind(review_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("AI review not found".to_string()))?;

    let messages = sqlx::query_as::<_, AiReviewMessage>(
        "SELECT * FROM ai_review_messages WHERE review_id = $1 ORDER BY created_at",
    )
    .bind(review_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(AiReviewResponse { review, messages }))
}

pub async fn list_ai_reviews(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<AiReview>>> {
    let reviews = sqlx::query_as::<_, AiReview>(
        "SELECT * FROM ai_reviews WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50",
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(reviews))
}

pub async fn continue_chat(
    State(pool): State<Arc<PgPool>>,
    State(ai_service): State<Arc<AiService>>,
    auth_user: AuthUser,
    Path(review_id): Path<Uuid>,
    Json(req): Json<ChatMessageRequest>,
) -> AppResult<Json<AiReviewResponse>> {
    let message = req.message.trim();
    if message.is_empty() || message.len() > 10_000 {
        return Err(AppError::Validation(
            "Message must be between 1 and 10,000 characters".to_string(),
        ));
    }

    let review = sqlx::query_as::<_, AiReview>(
        "SELECT * FROM ai_reviews WHERE id = $1 AND user_id = $2",
    )
    .bind(review_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("AI review not found".to_string()))?;

    let existing_messages = sqlx::query_as::<_, AiReviewMessage>(
        "SELECT * FROM ai_review_messages WHERE review_id = $1 ORDER BY created_at",
    )
    .bind(review_id)
    .fetch_all(pool.as_ref())
    .await?;

    let mut claude_messages: Vec<ClaudeMessage> = existing_messages
        .iter()
        .map(|m| ClaudeMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    claude_messages.push(ClaudeMessage {
        role: "user".to_string(),
        content: message.to_string(),
    });

    let (response_text, tokens) = ai_service.chat(claude_messages).await?;

    // Update token count on the review
    sqlx::query("UPDATE ai_reviews SET tokens_used = COALESCE(tokens_used, 0) + $1 WHERE id = $2")
        .bind(tokens)
        .bind(review_id)
        .execute(pool.as_ref())
        .await?;

    // Insert both messages
    sqlx::query("INSERT INTO ai_review_messages (review_id, role, content, tokens_used) VALUES ($1, 'user', $2, NULL)")
        .bind(review_id)
        .bind(message)
        .execute(pool.as_ref())
        .await?;

    sqlx::query("INSERT INTO ai_review_messages (review_id, role, content, tokens_used) VALUES ($1, 'assistant', $2, $3)")
        .bind(review_id)
        .bind(&response_text)
        .bind(tokens)
        .execute(pool.as_ref())
        .await?;

    // Re-fetch all messages for the response
    let all_messages = sqlx::query_as::<_, AiReviewMessage>(
        "SELECT * FROM ai_review_messages WHERE review_id = $1 ORDER BY created_at",
    )
    .bind(review_id)
    .fetch_all(pool.as_ref())
    .await?;

    // Re-fetch updated review (tokens_used changed)
    let updated_review = sqlx::query_as::<_, AiReview>(
        "SELECT * FROM ai_reviews WHERE id = $1",
    )
    .bind(review_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(AiReviewResponse {
        review: updated_review,
        messages: all_messages,
    }))
}

pub async fn delete_ai_review(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(review_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM ai_reviews WHERE id = $1 AND user_id = $2")
        .bind(review_id)
        .bind(auth_user.user_id)
        .execute(pool.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("AI review not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Review deleted" })))
}
