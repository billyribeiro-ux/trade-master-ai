use crate::error::AppResult;
use crate::services::RiskCalculator;
use axum::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PositionSizeRequest {
    pub account_size: Decimal,
    pub risk_percent: Decimal,
    pub entry_price: Decimal,
    pub stop_loss: Decimal,
}

#[derive(Debug, Serialize)]
pub struct PositionSizeResponse {
    pub position_size: Decimal,
    pub risk_amount: Decimal,
    pub position_value: Decimal,
}

pub async fn calculate_position_size(
    Json(req): Json<PositionSizeRequest>,
) -> AppResult<Json<PositionSizeResponse>> {
    let position_size = RiskCalculator::calculate_position_size(
        req.account_size,
        req.risk_percent,
        req.entry_price,
        req.stop_loss,
    );

    let risk_amount = req.account_size * (req.risk_percent / Decimal::from(100));
    let position_value = position_size * req.entry_price;

    Ok(Json(PositionSizeResponse {
        position_size,
        risk_amount,
        position_value,
    }))
}

#[derive(Debug, Deserialize)]
pub struct RiskRewardRequest {
    pub entry_price: Decimal,
    pub stop_loss: Decimal,
    pub target_price: Decimal,
}

#[derive(Debug, Serialize)]
pub struct RiskRewardResponse {
    pub risk_reward_ratio: Option<Decimal>,
    pub risk_amount: Decimal,
    pub reward_amount: Decimal,
}

pub async fn calculate_risk_reward(
    Json(req): Json<RiskRewardRequest>,
) -> AppResult<Json<RiskRewardResponse>> {
    let ratio = RiskCalculator::calculate_risk_reward_ratio(
        req.entry_price,
        req.stop_loss,
        req.target_price,
    );

    let risk = (req.entry_price - req.stop_loss).abs();
    let reward = (req.target_price - req.entry_price).abs();

    Ok(Json(RiskRewardResponse {
        risk_reward_ratio: ratio,
        risk_amount: risk,
        reward_amount: reward,
    }))
}

#[derive(Debug, Deserialize)]
pub struct KellyRequest {
    pub win_rate: Decimal,
    pub avg_win: Decimal,
    pub avg_loss: Decimal,
}

#[derive(Debug, Serialize)]
pub struct KellyResponse {
    pub kelly_percentage: Decimal,
    pub half_kelly: Decimal,
    pub quarter_kelly: Decimal,
}

pub async fn calculate_kelly(
    Json(req): Json<KellyRequest>,
) -> AppResult<Json<KellyResponse>> {
    let kelly = RiskCalculator::calculate_kelly_criterion(
        req.win_rate / Decimal::from(100),
        req.avg_win,
        req.avg_loss,
    );

    Ok(Json(KellyResponse {
        kelly_percentage: kelly,
        half_kelly: kelly / Decimal::from(2),
        quarter_kelly: kelly / Decimal::from(4),
    }))
}

#[derive(Debug, Deserialize)]
pub struct PortfolioHeatRequest {
    pub open_positions_risk: Vec<Decimal>,
    pub account_size: Decimal,
}

#[derive(Debug, Serialize)]
pub struct PortfolioHeatResponse {
    pub portfolio_heat: Decimal,
    pub total_risk: Decimal,
    pub positions_count: usize,
}

pub async fn calculate_portfolio_heat(
    Json(req): Json<PortfolioHeatRequest>,
) -> AppResult<Json<PortfolioHeatResponse>> {
    let heat = RiskCalculator::calculate_portfolio_heat(
        req.open_positions_risk.clone(),
        req.account_size,
    );

    let total_risk: Decimal = req.open_positions_risk.iter().sum();

    Ok(Json(PortfolioHeatResponse {
        portfolio_heat: heat,
        total_risk,
        positions_count: req.open_positions_risk.len(),
    }))
}
