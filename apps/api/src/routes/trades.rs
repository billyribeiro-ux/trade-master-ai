use crate::error::{AppError, AppResult};
use crate::models::{
    AuthUser, CloseTradeRequest, CreateTradeLegRequest, CreateTradeRequest, Trade, TradeLeg,
    TradeListQuery, TradeListResponse, TradeMedia, TradeStats, TradeStatus, TradeTag,
    TradeWithDetails, UpdateTradeRequest,
};
use crate::services::TradeCalculationService;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_trade(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Json(req): Json<CreateTradeRequest>,
) -> AppResult<Json<Trade>> {
    // --- Input validation ---
    let symbol = req.symbol.trim().to_uppercase();
    if symbol.is_empty() || symbol.len() > 20 {
        return Err(AppError::Validation(
            "Symbol must be between 1 and 20 characters".to_string(),
        ));
    }

    if req.entry_price <= Decimal::ZERO {
        return Err(AppError::Validation(
            "Entry price must be positive".to_string(),
        ));
    }

    if req.quantity <= Decimal::ZERO {
        return Err(AppError::Validation(
            "Quantity must be positive".to_string(),
        ));
    }

    if let Some(sl) = req.stop_loss {
        if sl <= Decimal::ZERO {
            return Err(AppError::Validation(
                "Stop loss must be positive".to_string(),
            ));
        }
    }

    if let Some(tp) = req.take_profit {
        if tp <= Decimal::ZERO {
            return Err(AppError::Validation(
                "Take profit must be positive".to_string(),
            ));
        }
    }

    if let Some(ref thesis) = req.thesis {
        if thesis.len() > 10_000 {
            return Err(AppError::Validation(
                "Thesis must be 10,000 characters or fewer".to_string(),
            ));
        }
    }

    if let Some(ref setup) = req.setup_name {
        if setup.len() > 200 {
            return Err(AppError::Validation(
                "Setup name must be 200 characters or fewer".to_string(),
            ));
        }
    }

    // Validate stop loss / take profit relative to direction
    TradeCalculationService::validate_trade_data(
        req.entry_price,
        req.quantity,
        req.stop_loss,
        req.take_profit,
        &req.direction,
    )?;

    // Calculate risk amount from stop loss when not explicitly provided
    let risk_amount = match (req.risk_amount, req.stop_loss) {
        (Some(ra), _) => Some(ra),
        (None, Some(sl)) => Some(TradeCalculationService::calculate_risk_from_stop(
            &req.direction,
            req.entry_price,
            sl,
            req.quantity,
        )),
        _ => None,
    };

    let trade = sqlx::query_as::<_, Trade>(
        r#"
        INSERT INTO trades (
            user_id, symbol, direction, asset_class, status,
            entry_date, entry_price, quantity, stop_loss, take_profit,
            risk_amount, risk_percent, position_size_pct, conviction,
            setup_name, timeframe, thesis, emotional_state, market_condition,
            is_paper_trade, commissions
        )
        VALUES ($1, $2, $3, $4, 'open', $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(&symbol)
    .bind(&req.direction)
    .bind(&req.asset_class)
    .bind(req.entry_date)
    .bind(req.entry_price)
    .bind(req.quantity)
    .bind(req.stop_loss)
    .bind(req.take_profit)
    .bind(risk_amount)
    .bind(req.risk_percent)
    .bind(req.position_size_pct)
    .bind(req.conviction)
    .bind(&req.setup_name)
    .bind(&req.timeframe)
    .bind(&req.thesis)
    .bind(&req.emotional_state)
    .bind(&req.market_condition)
    .bind(req.is_paper_trade.unwrap_or(false))
    .bind(req.commissions)
    .fetch_one(pool.as_ref())
    .await?;

    tracing::info!(
        trade_id = %trade.id,
        symbol = %trade.symbol,
        direction = ?trade.direction,
        "Trade created"
    );

    Ok(Json(trade))
}

pub async fn get_trade(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(trade_id): Path<Uuid>,
) -> AppResult<Json<TradeWithDetails>> {
    // Fetch the trade first — fail fast if it doesn't exist or doesn't belong to the user
    let trade = sqlx::query_as::<_, Trade>(
        r#"
        SELECT * FROM trades
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(trade_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Trade not found".to_string()))?;

    // Fetch related data concurrently — eliminates sequential N+1 pattern
    let (tags, legs, media) = tokio::try_join!(
        sqlx::query_as::<_, TradeTag>(
            r#"
            SELECT t.id, t.name, t.color, t.category
            FROM tags t
            INNER JOIN trade_tags tt ON t.id = tt.tag_id
            WHERE tt.trade_id = $1
            ORDER BY t.name
            "#,
        )
        .bind(trade_id)
        .fetch_all(pool.as_ref()),

        sqlx::query_as::<_, TradeLeg>(
            r#"
            SELECT * FROM trade_legs
            WHERE trade_id = $1
            ORDER BY leg_number
            "#,
        )
        .bind(trade_id)
        .fetch_all(pool.as_ref()),

        sqlx::query_as::<_, TradeMedia>(
            r#"
            SELECT * FROM trade_media
            WHERE trade_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(trade_id)
        .fetch_all(pool.as_ref()),
    )?;

    Ok(Json(TradeWithDetails {
        trade,
        tags,
        legs,
        media,
    }))
}

/// Validates that a sort column name is one of the allowed trade table columns.
/// Returns the validated column name or an error. This prevents SQL injection
/// by whitelisting rather than sanitizing.
fn validate_sort_column(input: &str) -> AppResult<&'static str> {
    match input {
        "entry_date" => Ok("entry_date"),
        "exit_date" => Ok("exit_date"),
        "symbol" => Ok("symbol"),
        "direction" => Ok("direction"),
        "asset_class" => Ok("asset_class"),
        "status" => Ok("status"),
        "entry_price" => Ok("entry_price"),
        "exit_price" => Ok("exit_price"),
        "quantity" => Ok("quantity"),
        "pnl" => Ok("pnl"),
        "net_pnl" => Ok("net_pnl"),
        "pnl_percent" => Ok("pnl_percent"),
        "r_multiple" => Ok("r_multiple"),
        "hold_time_minutes" => Ok("hold_time_minutes"),
        "created_at" => Ok("created_at"),
        "updated_at" => Ok("updated_at"),
        _ => Err(AppError::Validation(format!(
            "Invalid sort column '{}'. Allowed: entry_date, exit_date, symbol, direction, \
             asset_class, status, entry_price, exit_price, quantity, pnl, net_pnl, \
             pnl_percent, r_multiple, hold_time_minutes, created_at, updated_at",
            input
        ))),
    }
}

/// Validates sort direction. Only "asc" and "desc" are allowed.
fn validate_sort_order(input: &str) -> AppResult<&'static str> {
    match input.to_lowercase().as_str() {
        "asc" => Ok("ASC"),
        "desc" => Ok("DESC"),
        _ => Err(AppError::Validation(format!(
            "Invalid sort order '{}'. Allowed: asc, desc",
            input
        ))),
    }
}

pub async fn list_trades(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Query(query): Query<TradeListQuery>,
) -> AppResult<Json<TradeListResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * per_page;

    // Validate sort parameters against whitelist — prevents SQL injection
    let sort_column = validate_sort_column(
        query.sort_by.as_deref().unwrap_or("entry_date"),
    )?;
    let sort_direction = validate_sort_order(
        query.sort_order.as_deref().unwrap_or("desc"),
    )?;

    // Build WHERE clause using parameterized conditions only
    let mut conditions = vec!["user_id = $1".to_string()];
    let mut param_count: i32 = 1;

    if query.filters.status.is_some() {
        param_count += 1;
        conditions.push(format!("status = ${}", param_count));
    }
    if query.filters.direction.is_some() {
        param_count += 1;
        conditions.push(format!("direction = ${}", param_count));
    }
    if query.filters.asset_class.is_some() {
        param_count += 1;
        conditions.push(format!("asset_class = ${}", param_count));
    }
    if query.filters.symbol.is_some() {
        param_count += 1;
        conditions.push(format!("symbol ILIKE ${}", param_count));
    }
    if query.filters.setup_name.is_some() {
        param_count += 1;
        conditions.push(format!("setup_name ILIKE ${}", param_count));
    }
    if query.filters.conviction.is_some() {
        param_count += 1;
        conditions.push(format!("conviction = ${}", param_count));
    }
    if query.filters.is_paper_trade.is_some() {
        param_count += 1;
        conditions.push(format!("is_paper_trade = ${}", param_count));
    }
    if query.filters.from_date.is_some() {
        param_count += 1;
        conditions.push(format!("entry_date >= ${}", param_count));
    }
    if query.filters.to_date.is_some() {
        param_count += 1;
        conditions.push(format!("entry_date <= ${}", param_count));
    }

    let where_clause = conditions.join(" AND ");

    // ORDER BY uses only validated static strings — safe from injection
    let order_clause = format!("{} {}", sort_column, sort_direction);

    // Helper closure: binds all active filter params to a query builder.
    // We use a macro-style approach to avoid duplicating the bind chain.
    macro_rules! bind_filters {
        ($q:expr) => {{
            let mut q = $q.bind(auth_user.user_id);
            if let Some(ref v) = query.filters.status { q = q.bind(v); }
            if let Some(ref v) = query.filters.direction { q = q.bind(v); }
            if let Some(ref v) = query.filters.asset_class { q = q.bind(v); }
            if let Some(ref v) = query.filters.symbol { q = q.bind(format!("%{}%", v)); }
            if let Some(ref v) = query.filters.setup_name { q = q.bind(format!("%{}%", v)); }
            if let Some(ref v) = query.filters.conviction { q = q.bind(v); }
            if let Some(v) = query.filters.is_paper_trade { q = q.bind(v); }
            if let Some(v) = query.filters.from_date { q = q.bind(v); }
            if let Some(v) = query.filters.to_date { q = q.bind(v); }
            q
        }};
    }

    // Count total matching rows
    let count_sql = format!("SELECT COUNT(*) FROM trades WHERE {}", where_clause);
    let total = bind_filters!(sqlx::query_scalar::<_, i64>(&count_sql))
        .fetch_one(pool.as_ref())
        .await?;

    // Fetch paginated trades
    let trades_sql = format!(
        "SELECT * FROM trades WHERE {} ORDER BY {} LIMIT ${} OFFSET ${}",
        where_clause,
        order_clause,
        param_count + 1,
        param_count + 2
    );
    let trades = bind_filters!(sqlx::query_as::<_, Trade>(&trades_sql))
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool.as_ref())
        .await?;

    let total_pages = if per_page > 0 {
        (total + per_page - 1) / per_page  // ceiling division without floating point
    } else {
        0
    };

    Ok(Json(TradeListResponse {
        trades,
        total,
        page,
        per_page,
        total_pages,
    }))
}

pub async fn update_trade(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(trade_id): Path<Uuid>,
    Json(req): Json<UpdateTradeRequest>,
) -> AppResult<Json<Trade>> {
    // Verify trade exists and belongs to user
    let existing = sqlx::query_as::<_, Trade>(
        r#"
        SELECT * FROM trades WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(trade_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Trade not found".to_string()))?;

    // Build update query dynamically
    let mut updates = vec![];
    let mut param_count = 1;

    if req.symbol.is_some() {
        param_count += 1;
        updates.push(format!("symbol = ${}", param_count));
    }
    if req.direction.is_some() {
        param_count += 1;
        updates.push(format!("direction = ${}", param_count));
    }
    if req.asset_class.is_some() {
        param_count += 1;
        updates.push(format!("asset_class = ${}", param_count));
    }
    if req.entry_price.is_some() {
        param_count += 1;
        updates.push(format!("entry_price = ${}", param_count));
    }
    if req.quantity.is_some() {
        param_count += 1;
        updates.push(format!("quantity = ${}", param_count));
    }
    if req.stop_loss.is_some() {
        param_count += 1;
        updates.push(format!("stop_loss = ${}", param_count));
    }
    if req.take_profit.is_some() {
        param_count += 1;
        updates.push(format!("take_profit = ${}", param_count));
    }
    if req.thesis.is_some() {
        param_count += 1;
        updates.push(format!("thesis = ${}", param_count));
    }
    if req.setup_name.is_some() {
        param_count += 1;
        updates.push(format!("setup_name = ${}", param_count));
    }
    if req.conviction.is_some() {
        param_count += 1;
        updates.push(format!("conviction = ${}", param_count));
    }

    if updates.is_empty() {
        return Ok(Json(existing));
    }

    updates.push("updated_at = NOW()".to_string());

    let update_query = format!(
        "UPDATE trades SET {} WHERE id = $1 RETURNING *",
        updates.join(", ")
    );

    let mut query = sqlx::query_as::<_, Trade>(&update_query).bind(trade_id);

    if let Some(v) = &req.symbol {
        query = query.bind(v);
    }
    if let Some(v) = &req.direction {
        query = query.bind(v);
    }
    if let Some(v) = &req.asset_class {
        query = query.bind(v);
    }
    if let Some(v) = req.entry_price {
        query = query.bind(v);
    }
    if let Some(v) = req.quantity {
        query = query.bind(v);
    }
    if let Some(v) = req.stop_loss {
        query = query.bind(v);
    }
    if let Some(v) = req.take_profit {
        query = query.bind(v);
    }
    if let Some(v) = &req.thesis {
        query = query.bind(v);
    }
    if let Some(v) = &req.setup_name {
        query = query.bind(v);
    }
    if let Some(v) = &req.conviction {
        query = query.bind(v);
    }

    let trade = query.fetch_one(pool.as_ref()).await?;

    Ok(Json(trade))
}

pub async fn close_trade(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(trade_id): Path<Uuid>,
    Json(req): Json<CloseTradeRequest>,
) -> AppResult<Json<Trade>> {
    // Get existing trade
    let trade = sqlx::query_as::<_, Trade>(
        r#"
        SELECT * FROM trades WHERE id = $1 AND user_id = $2 AND status = 'open'
        "#,
    )
    .bind(trade_id)
    .bind(auth_user.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Open trade not found".to_string()))?;

    // Calculate metrics
    let (pnl, pnl_percent, net_pnl, r_multiple, hold_time) =
        TradeCalculationService::calculate_close_metrics(&trade, &req)?;

    // Update trade
    let updated_trade = sqlx::query_as::<_, Trade>(
        r#"
        UPDATE trades SET
            status = 'closed',
            exit_date = $1,
            exit_price = $2,
            actual_exit_price = $3,
            pnl = $4,
            pnl_percent = $5,
            net_pnl = $6,
            r_multiple = $7,
            hold_time_minutes = $8,
            mistakes = $9,
            lessons = $10,
            execution_grade = $11,
            patience_grade = $12,
            discipline_grade = $13,
            overall_grade = $14,
            broke_rules = $15,
            followed_plan = $16,
            updated_at = NOW()
        WHERE id = $17
        RETURNING *
        "#,
    )
    .bind(req.exit_date)
    .bind(req.exit_price)
    .bind(req.actual_exit_price)
    .bind(pnl)
    .bind(pnl_percent)
    .bind(net_pnl)
    .bind(r_multiple)
    .bind(hold_time)
    .bind(&req.mistakes)
    .bind(&req.lessons)
    .bind(&req.execution_grade)
    .bind(&req.patience_grade)
    .bind(&req.discipline_grade)
    .bind(&req.overall_grade)
    .bind(req.broke_rules)
    .bind(req.followed_plan)
    .bind(trade_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(updated_trade))
}

pub async fn delete_trade(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(trade_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query(
        r#"
        DELETE FROM trades WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(trade_id)
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Trade not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "message": "Trade deleted successfully" })))
}

pub async fn add_trade_leg(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Path(trade_id): Path<Uuid>,
    Json(req): Json<CreateTradeLegRequest>,
) -> AppResult<Json<TradeLeg>> {
    // Verify trade exists and belongs to user
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

    // Get next leg number
    let next_leg_number = sqlx::query_scalar::<_, Option<i32>>(
        r#"
        SELECT MAX(leg_number) FROM trade_legs WHERE trade_id = $1
        "#,
    )
    .bind(trade_id)
    .fetch_one(pool.as_ref())
    .await?
    .map(|n| n + 1)
    .unwrap_or(1);

    // Create leg
    let leg = sqlx::query_as::<_, TradeLeg>(
        r#"
        INSERT INTO trade_legs (trade_id, leg_number, action, quantity, price, timestamp, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(trade_id)
    .bind(next_leg_number)
    .bind(&req.action)
    .bind(req.quantity)
    .bind(req.price)
    .bind(req.timestamp)
    .bind(&req.notes)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(leg))
}

pub async fn get_trade_stats(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<TradeStats>> {
    let stats = sqlx::query_as::<_, TradeStats>(
        r#"
        SELECT
            COUNT(*) as total_trades,
            COUNT(*) FILTER (WHERE pnl > 0) as winning_trades,
            COUNT(*) FILTER (WHERE pnl <= 0) as losing_trades,
            COALESCE(
                CAST(COUNT(*) FILTER (WHERE pnl > 0) AS DECIMAL) / NULLIF(COUNT(*), 0) * 100,
                0
            ) as win_rate,
            COALESCE(SUM(net_pnl), 0) as total_pnl,
            COALESCE(AVG(net_pnl) FILTER (WHERE pnl > 0), 0) as avg_win,
            COALESCE(AVG(net_pnl) FILTER (WHERE pnl <= 0), 0) as avg_loss,
            CASE
                WHEN ABS(SUM(net_pnl) FILTER (WHERE pnl <= 0)) > 0
                THEN ABS(SUM(net_pnl) FILTER (WHERE pnl > 0) / SUM(net_pnl) FILTER (WHERE pnl <= 0))
                ELSE NULL
            END as profit_factor,
            AVG(r_multiple) as avg_r_multiple,
            COALESCE(MAX(net_pnl), 0) as largest_win,
            COALESCE(MIN(net_pnl), 0) as largest_loss,
            AVG(hold_time_minutes)::INTEGER as avg_hold_time_minutes
        FROM trades
        WHERE user_id = $1 AND status = 'closed'
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(stats))
}
