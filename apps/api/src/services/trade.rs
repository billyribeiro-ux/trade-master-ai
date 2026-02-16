use crate::error::{AppError, AppResult};
use crate::models::{CloseTradeRequest, Trade, TradeDirection};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct TradeCalculationService;

impl TradeCalculationService {
    /// Calculate P&L for a trade
    pub fn calculate_pnl(
        direction: &TradeDirection,
        entry_price: Decimal,
        exit_price: Decimal,
        quantity: Decimal,
    ) -> Decimal {
        match direction {
            TradeDirection::Long => (exit_price - entry_price) * quantity,
            TradeDirection::Short => (entry_price - exit_price) * quantity,
        }
    }

    /// Calculate P&L percentage
    pub fn calculate_pnl_percent(pnl: Decimal, entry_price: Decimal, quantity: Decimal) -> Decimal {
        let cost_basis = entry_price * quantity;
        if cost_basis.is_zero() {
            return Decimal::ZERO;
        }
        (pnl / cost_basis) * Decimal::from(100)
    }

    /// Calculate net P&L after commissions
    pub fn calculate_net_pnl(pnl: Decimal, commissions: Option<Decimal>) -> Decimal {
        pnl - commissions.unwrap_or(Decimal::ZERO)
    }

    /// Calculate R-multiple (reward/risk ratio)
    pub fn calculate_r_multiple(pnl: Decimal, risk_amount: Option<Decimal>) -> Option<Decimal> {
        risk_amount.and_then(|risk| {
            if risk.is_zero() {
                None
            } else {
                Some(pnl / risk)
            }
        })
    }

    /// Calculate hold time in minutes
    pub fn calculate_hold_time(
        entry_date: chrono::DateTime<Utc>,
        exit_date: chrono::DateTime<Utc>,
    ) -> i32 {
        let duration = exit_date.signed_duration_since(entry_date);
        duration.num_minutes() as i32
    }

    /// Calculate risk amount from stop loss
    pub fn calculate_risk_from_stop(
        direction: &TradeDirection,
        entry_price: Decimal,
        stop_loss: Decimal,
        quantity: Decimal,
    ) -> Decimal {
        match direction {
            TradeDirection::Long => {
                if entry_price > stop_loss {
                    (entry_price - stop_loss) * quantity
                } else {
                    Decimal::ZERO
                }
            }
            TradeDirection::Short => {
                if stop_loss > entry_price {
                    (stop_loss - entry_price) * quantity
                } else {
                    Decimal::ZERO
                }
            }
        }
    }

    /// Calculate position size percentage
    pub fn calculate_position_size_pct(
        position_value: Decimal,
        account_size: Decimal,
    ) -> Option<Decimal> {
        if account_size.is_zero() {
            None
        } else {
            Some((position_value / account_size) * Decimal::from(100))
        }
    }

    /// Update trade with calculated values on close
    pub fn calculate_close_metrics(
        trade: &Trade,
        close_request: &CloseTradeRequest,
    ) -> AppResult<(Decimal, Decimal, Decimal, Option<Decimal>, i32)> {
        let exit_price = close_request
            .actual_exit_price
            .unwrap_or(close_request.exit_price);

        // Calculate P&L
        let pnl = Self::calculate_pnl(
            &trade.direction,
            trade.entry_price,
            exit_price,
            trade.quantity,
        );

        // Calculate P&L percentage
        let pnl_percent = Self::calculate_pnl_percent(pnl, trade.entry_price, trade.quantity);

        // Calculate net P&L
        let net_pnl = Self::calculate_net_pnl(pnl, trade.commissions);

        // Calculate R-multiple
        let r_multiple = Self::calculate_r_multiple(pnl, trade.risk_amount);

        // Calculate hold time
        let hold_time = Self::calculate_hold_time(trade.entry_date, close_request.exit_date);

        Ok((pnl, pnl_percent, net_pnl, r_multiple, hold_time))
    }

    /// Validate trade data
    pub fn validate_trade_data(
        entry_price: Decimal,
        quantity: Decimal,
        stop_loss: Option<Decimal>,
        take_profit: Option<Decimal>,
        direction: &TradeDirection,
    ) -> AppResult<()> {
        if entry_price <= Decimal::ZERO {
            return Err(AppError::Validation(
                "Entry price must be greater than zero".to_string(),
            ));
        }

        if quantity <= Decimal::ZERO {
            return Err(AppError::Validation(
                "Quantity must be greater than zero".to_string(),
            ));
        }

        // Validate stop loss is on the correct side
        if let Some(sl) = stop_loss {
            match direction {
                TradeDirection::Long => {
                    if sl >= entry_price {
                        return Err(AppError::Validation(
                            "Stop loss must be below entry price for long trades".to_string(),
                        ));
                    }
                }
                TradeDirection::Short => {
                    if sl <= entry_price {
                        return Err(AppError::Validation(
                            "Stop loss must be above entry price for short trades".to_string(),
                        ));
                    }
                }
            }
        }

        // Validate take profit is on the correct side
        if let Some(tp) = take_profit {
            match direction {
                TradeDirection::Long => {
                    if tp <= entry_price {
                        return Err(AppError::Validation(
                            "Take profit must be above entry price for long trades".to_string(),
                        ));
                    }
                }
                TradeDirection::Short => {
                    if tp >= entry_price {
                        return Err(AppError::Validation(
                            "Take profit must be below entry price for short trades".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pnl_long() {
        let pnl = TradeCalculationService::calculate_pnl(
            &TradeDirection::Long,
            Decimal::from(100),
            Decimal::from(110),
            Decimal::from(10),
        );
        assert_eq!(pnl, Decimal::from(100)); // (110 - 100) * 10
    }

    #[test]
    fn test_calculate_pnl_short() {
        let pnl = TradeCalculationService::calculate_pnl(
            &TradeDirection::Short,
            Decimal::from(100),
            Decimal::from(90),
            Decimal::from(10),
        );
        assert_eq!(pnl, Decimal::from(100)); // (100 - 90) * 10
    }

    #[test]
    fn test_calculate_pnl_percent() {
        let pnl_pct = TradeCalculationService::calculate_pnl_percent(
            Decimal::from(100),
            Decimal::from(100),
            Decimal::from(10),
        );
        assert_eq!(pnl_pct, Decimal::from(10)); // (100 / 1000) * 100
    }

    #[test]
    fn test_calculate_r_multiple() {
        let r = TradeCalculationService::calculate_r_multiple(
            Decimal::from(200),
            Some(Decimal::from(100)),
        );
        assert_eq!(r, Some(Decimal::from(2)));
    }

    #[test]
    fn test_validate_trade_data_long() {
        let result = TradeCalculationService::validate_trade_data(
            Decimal::from(100),
            Decimal::from(10),
            Some(Decimal::from(95)),
            Some(Decimal::from(110)),
            &TradeDirection::Long,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_trade_data_invalid_stop() {
        let result = TradeCalculationService::validate_trade_data(
            Decimal::from(100),
            Decimal::from(10),
            Some(Decimal::from(105)), // Stop above entry for long
            Some(Decimal::from(110)),
            &TradeDirection::Long,
        );
        assert!(result.is_err());
    }
}
