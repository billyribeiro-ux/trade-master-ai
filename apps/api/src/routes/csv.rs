use crate::error::{AppError, AppResult};
use crate::models::{AssetClass, AuthUser, ConvictionLevel, Trade, TradeDirection};
use crate::services::TradeCalculationService;
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CsvImportRequest {
    pub trades: Vec<CsvTradeRow>,
}

#[derive(Debug, Deserialize)]
pub struct CsvTradeRow {
    pub symbol: String,
    pub direction: String,
    pub asset_class: String,
    pub entry_date: String,
    pub entry_price: String,
    pub quantity: String,
    pub exit_date: Option<String>,
    pub exit_price: Option<String>,
    pub stop_loss: Option<String>,
    pub take_profit: Option<String>,
    pub setup_name: Option<String>,
    pub timeframe: Option<String>,
    pub conviction: Option<String>,
    pub thesis: Option<String>,
    pub commissions: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CsvImportResponse {
    pub success_count: usize,
    pub error_count: usize,
    pub errors: Vec<CsvImportError>,
}

#[derive(Debug, Serialize)]
pub struct CsvImportError {
    pub row: usize,
    pub symbol: String,
    pub error: String,
}

pub async fn import_csv(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
    Json(req): Json<CsvImportRequest>,
) -> AppResult<Json<CsvImportResponse>> {
    let mut success_count = 0;
    let mut errors = Vec::new();

    for (index, row) in req.trades.iter().enumerate() {
        match import_single_trade(&pool, &auth_user, row).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                errors.push(CsvImportError {
                    row: index + 1,
                    symbol: row.symbol.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    Ok(Json(CsvImportResponse {
        success_count,
        error_count: errors.len(),
        errors,
    }))
}

async fn import_single_trade(
    pool: &PgPool,
    auth_user: &AuthUser,
    row: &CsvTradeRow,
) -> AppResult<Trade> {
    // Parse direction
    let direction = match row.direction.to_lowercase().as_str() {
        "long" | "buy" => TradeDirection::Long,
        "short" | "sell" => TradeDirection::Short,
        _ => return Err(AppError::Validation(format!("Invalid direction: {}", row.direction))),
    };

    // Parse asset class
    let asset_class = match row.asset_class.to_lowercase().as_str() {
        "stocks" | "stock" | "equity" => AssetClass::Stocks,
        "options" | "option" => AssetClass::Options,
        "futures" | "future" => AssetClass::Futures,
        "forex" | "fx" => AssetClass::Forex,
        "crypto" | "cryptocurrency" => AssetClass::Crypto,
        _ => return Err(AppError::Validation(format!("Invalid asset class: {}", row.asset_class))),
    };

    // Parse dates
    let entry_date = DateTime::parse_from_rfc3339(&row.entry_date)
        .or_else(|_| DateTime::parse_from_str(&row.entry_date, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| DateTime::parse_from_str(&row.entry_date, "%Y-%m-%d"))
        .map_err(|_| AppError::Validation(format!("Invalid entry date: {}", row.entry_date)))?
        .with_timezone(&Utc);

    let exit_date = if let Some(exit_str) = &row.exit_date {
        Some(
            DateTime::parse_from_rfc3339(exit_str)
                .or_else(|_| DateTime::parse_from_str(exit_str, "%Y-%m-%d %H:%M:%S"))
                .or_else(|_| DateTime::parse_from_str(exit_str, "%Y-%m-%d"))
                .map_err(|_| AppError::Validation(format!("Invalid exit date: {}", exit_str)))?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    // Parse prices
    let entry_price: Decimal = row.entry_price.parse()
        .map_err(|_| AppError::Validation(format!("Invalid entry price: {}", row.entry_price)))?;
    
    let quantity: Decimal = row.quantity.parse()
        .map_err(|_| AppError::Validation(format!("Invalid quantity: {}", row.quantity)))?;

    let exit_price = if let Some(exit_str) = &row.exit_price {
        Some(exit_str.parse::<Decimal>()
            .map_err(|_| AppError::Validation(format!("Invalid exit price: {}", exit_str)))?)
    } else {
        None
    };

    let stop_loss = if let Some(sl_str) = &row.stop_loss {
        Some(sl_str.parse::<Decimal>()
            .map_err(|_| AppError::Validation(format!("Invalid stop loss: {}", sl_str)))?)
    } else {
        None
    };

    let take_profit = if let Some(tp_str) = &row.take_profit {
        Some(tp_str.parse::<Decimal>()
            .map_err(|_| AppError::Validation(format!("Invalid take profit: {}", tp_str)))?)
    } else {
        None
    };

    let commissions = if let Some(comm_str) = &row.commissions {
        Some(comm_str.parse::<Decimal>()
            .map_err(|_| AppError::Validation(format!("Invalid commissions: {}", comm_str)))?)
    } else {
        None
    };

    // Parse conviction
    let conviction = if let Some(conv_str) = &row.conviction {
        match conv_str.to_lowercase().as_str() {
            "low" => Some(ConvictionLevel::Low),
            "medium" | "med" => Some(ConvictionLevel::Medium),
            "high" => Some(ConvictionLevel::High),
            _ => None,
        }
    } else {
        None
    };

    // Validate trade data
    TradeCalculationService::validate_trade_data(
        entry_price,
        quantity,
        stop_loss,
        take_profit,
        &direction,
    )?;

    // Calculate risk amount if stop loss provided
    let risk_amount = if let Some(sl) = stop_loss {
        Some(TradeCalculationService::calculate_risk_from_stop(
            &direction,
            entry_price,
            sl,
            quantity,
        ))
    } else {
        None
    };

    // Determine status
    let status = if exit_price.is_some() && exit_date.is_some() {
        "closed"
    } else {
        "open"
    };

    // Calculate P&L if closed
    let (pnl, pnl_percent, net_pnl, r_multiple, hold_time) = if let (Some(exit_p), Some(exit_d)) = (exit_price, exit_date) {
        let pnl = TradeCalculationService::calculate_pnl(&direction, entry_price, exit_p, quantity);
        let pnl_pct = TradeCalculationService::calculate_pnl_percent(pnl, entry_price, quantity);
        let net = TradeCalculationService::calculate_net_pnl(pnl, commissions);
        let r_mult = TradeCalculationService::calculate_r_multiple(pnl, risk_amount);
        let hold = TradeCalculationService::calculate_hold_time(entry_date, exit_d);
        (Some(pnl), Some(pnl_pct), Some(net), r_mult, Some(hold))
    } else {
        (None, None, None, None, None)
    };

    // Insert trade
    let trade = sqlx::query_as::<_, Trade>(
        r#"
        INSERT INTO trades (
            user_id, symbol, direction, asset_class, status,
            entry_date, entry_price, quantity, stop_loss, take_profit,
            exit_date, exit_price, actual_exit_price,
            pnl, pnl_percent, net_pnl, r_multiple, hold_time_minutes,
            risk_amount, conviction, setup_name, timeframe, thesis, commissions
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(&row.symbol.to_uppercase())
    .bind(&direction)
    .bind(&asset_class)
    .bind(status)
    .bind(entry_date)
    .bind(entry_price)
    .bind(quantity)
    .bind(stop_loss)
    .bind(take_profit)
    .bind(exit_date)
    .bind(exit_price)
    .bind(exit_price)
    .bind(pnl)
    .bind(pnl_percent)
    .bind(net_pnl)
    .bind(r_multiple)
    .bind(hold_time)
    .bind(risk_amount)
    .bind(conviction)
    .bind(&row.setup_name)
    .bind(&row.timeframe)
    .bind(&row.thesis)
    .bind(commissions)
    .fetch_one(pool)
    .await?;

    Ok(trade)
}

#[derive(Debug, Serialize)]
pub struct CsvTemplateResponse {
    pub headers: Vec<String>,
    pub example_rows: Vec<Vec<String>>,
}

pub async fn get_csv_template() -> AppResult<Json<CsvTemplateResponse>> {
    Ok(Json(CsvTemplateResponse {
        headers: vec![
            "symbol".to_string(),
            "direction".to_string(),
            "asset_class".to_string(),
            "entry_date".to_string(),
            "entry_price".to_string(),
            "quantity".to_string(),
            "exit_date".to_string(),
            "exit_price".to_string(),
            "stop_loss".to_string(),
            "take_profit".to_string(),
            "setup_name".to_string(),
            "timeframe".to_string(),
            "conviction".to_string(),
            "thesis".to_string(),
            "commissions".to_string(),
        ],
        example_rows: vec![
            vec![
                "AAPL".to_string(),
                "long".to_string(),
                "stocks".to_string(),
                "2024-01-15 09:30:00".to_string(),
                "150.00".to_string(),
                "100".to_string(),
                "2024-01-15 15:45:00".to_string(),
                "155.00".to_string(),
                "148.00".to_string(),
                "157.00".to_string(),
                "Bull Flag Breakout".to_string(),
                "5m".to_string(),
                "high".to_string(),
                "Strong momentum with volume confirmation".to_string(),
                "1.00".to_string(),
            ],
            vec![
                "TSLA".to_string(),
                "short".to_string(),
                "stocks".to_string(),
                "2024-01-16 10:00:00".to_string(),
                "200.00".to_string(),
                "50".to_string(),
                "".to_string(),
                "".to_string(),
                "205.00".to_string(),
                "190.00".to_string(),
                "Resistance Rejection".to_string(),
                "15m".to_string(),
                "medium".to_string(),
                "".to_string(),
                "0.50".to_string(),
            ],
        ],
    }))
}
