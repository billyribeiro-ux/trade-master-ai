use crate::error::{AppError, AppResult};
use crate::models::{AuthUser, CreateTagRequest, Tag, TagWithCount, UpdateTagRequest};
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_tag(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Json(req): Json<CreateTagRequest>,
) -> AppResult<Json<Tag>> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        INSERT INTO tags (user_id, name, color, category)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(&req.name)
    .bind(&req.color)
    .bind(&req.category)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Tag with this name already exists".to_string())
        }
        _ => AppError::from(e),
    })?;

    Ok(Json(tag))
}

pub async fn list_tags(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<TagWithCount>>> {
    let tags = sqlx::query_as::<_, TagWithCount>(
        r#"
        SELECT
            t.*,
            COUNT(tt.trade_id) as trade_count
        FROM tags t
        LEFT JOIN trade_tags tt ON t.id = tt.tag_id
        WHERE t.user_id = $1 OR t.is_system = true
        GROUP BY t.id
        ORDER BY t.name
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(tags))
}

pub async fn get_tag(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(tag_id): Path<Uuid>,
) -> AppResult<Json<Tag>> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        SELECT * FROM tags
        WHERE id = $1 AND (user_id = $2 OR is_system = true)
        "#,
    )
    .bind(tag_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    Ok(Json(tag))
}

pub async fn update_tag(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(tag_id): Path<Uuid>,
    Json(req): Json<UpdateTagRequest>,
) -> AppResult<Json<Tag>> {
    // Verify tag exists, belongs to user, and is not a system tag
    let existing = sqlx::query_as::<_, Tag>(
        r#"
        SELECT * FROM tags WHERE id = $1 AND user_id = $2 AND is_system = false
        "#,
    )
    .bind(tag_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Tag not found or cannot be modified".to_string()))?;

    // Build update query
    let mut updates = vec![];
    let mut param_count = 1;

    if req.name.is_some() {
        param_count += 1;
        updates.push(format!("name = ${}", param_count));
    }
    if req.color.is_some() {
        param_count += 1;
        updates.push(format!("color = ${}", param_count));
    }
    if req.category.is_some() {
        param_count += 1;
        updates.push(format!("category = ${}", param_count));
    }

    if updates.is_empty() {
        return Ok(Json(existing));
    }

    updates.push("updated_at = NOW()".to_string());

    let update_query = format!(
        "UPDATE tags SET {} WHERE id = $1 RETURNING *",
        updates.join(", ")
    );

    let mut query = sqlx::query_as::<_, Tag>(&update_query).bind(tag_id);

    if let Some(v) = &req.name {
        query = query.bind(v);
    }
    if let Some(v) = &req.color {
        query = query.bind(v);
    }
    if let Some(v) = &req.category {
        query = query.bind(v);
    }

    let tag = query.fetch_one(pool.as_ref()).await?;

    Ok(Json(tag))
}

pub async fn delete_tag(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(tag_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query(
        r#"
        DELETE FROM tags WHERE id = $1 AND user_id = $2 AND is_system = false
        "#,
    )
    .bind(tag_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Tag not found or cannot be deleted".to_string(),
        ));
    }

    Ok(Json(serde_json::json!({ "message": "Tag deleted successfully" })))
}

pub async fn add_tag_to_trade(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path((trade_id, tag_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    // Verify trade belongs to user
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(SELECT 1 FROM trades WHERE id = $1 AND user_id = $2)
        "#,
    )
    .bind(trade_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Trade not found".to_string()))?;

    // Verify tag exists and is accessible
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(SELECT 1 FROM tags WHERE id = $1 AND (user_id = $2 OR is_system = true))
        "#,
    )
    .bind(tag_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    // Add tag to trade
    sqlx::query(
        r#"
        INSERT INTO trade_tags (trade_id, tag_id)
        VALUES ($1, $2)
        ON CONFLICT (trade_id, tag_id) DO NOTHING
        "#,
    )
    .bind(trade_id)
    .bind(tag_id)
    .execute(pool.as_ref())
    .await?;

    Ok(Json(serde_json::json!({ "message": "Tag added to trade" })))
}

pub async fn remove_tag_from_trade(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path((trade_id, tag_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    // Verify trade belongs to user
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(SELECT 1 FROM trades WHERE id = $1 AND user_id = $2)
        "#,
    )
    .bind(trade_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?
    .then_some(())
    .ok_or_else(|| AppError::NotFound("Trade not found".to_string()))?;

    // Remove tag from trade
    let result = sqlx::query(
        r#"
        DELETE FROM trade_tags WHERE trade_id = $1 AND tag_id = $2
        "#,
    )
    .bind(trade_id)
    .bind(tag_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tag not associated with trade".to_string()));
    }

    Ok(Json(
        serde_json::json!({ "message": "Tag removed from trade" }),
    ))
}
