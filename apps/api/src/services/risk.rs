use rust_decimal::Decimal;

pub struct RiskCalculator;

impl RiskCalculator {
    pub fn calculate_position_size(
        account_size: Decimal,
        risk_percent: Decimal,
        entry_price: Decimal,
        stop_loss: Decimal,
    ) -> Decimal {
        let risk_amount = account_size * (risk_percent / Decimal::from(100));
        let risk_per_share = (entry_price - stop_loss).abs();
        
        if risk_per_share.is_zero() {
            return Decimal::ZERO;
        }
        
        risk_amount / risk_per_share
    }

    pub fn calculate_risk_reward_ratio(
        entry_price: Decimal,
        stop_loss: Decimal,
        target_price: Decimal,
    ) -> Option<Decimal> {
        let risk = (entry_price - stop_loss).abs();
        let reward = (target_price - entry_price).abs();
        
        if risk.is_zero() {
            return None;
        }
        
        Some(reward / risk)
    }

    pub fn calculate_kelly_criterion(
        win_rate: Decimal,
        avg_win: Decimal,
        avg_loss: Decimal,
    ) -> Decimal {
        if avg_loss.is_zero() {
            return Decimal::ZERO;
        }
        
        let win_loss_ratio = avg_win / avg_loss;
        let kelly = (win_rate * win_loss_ratio - (Decimal::ONE - win_rate)) / win_loss_ratio;
        
        kelly.max(Decimal::ZERO).min(Decimal::from(100))
    }

    pub fn calculate_max_position_size(
        account_size: Decimal,
        max_risk_percent: Decimal,
    ) -> Decimal {
        account_size * (max_risk_percent / Decimal::from(100))
    }

    pub fn calculate_breakeven_price(
        entry_price: Decimal,
        quantity: Decimal,
        commissions: Decimal,
    ) -> Decimal {
        if quantity.is_zero() {
            return entry_price;
        }
        
        entry_price + (commissions / quantity)
    }

    pub fn calculate_portfolio_heat(
        open_positions_risk: Vec<Decimal>,
        account_size: Decimal,
    ) -> Decimal {
        let total_risk: Decimal = open_positions_risk.iter().sum();
        
        if account_size.is_zero() {
            return Decimal::ZERO;
        }
        
        (total_risk / account_size) * Decimal::from(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_size_calculation() {
        let size = RiskCalculator::calculate_position_size(
            Decimal::from(10000),
            Decimal::from(1),
            Decimal::from(100),
            Decimal::from(98),
        );
        assert_eq!(size, Decimal::from(50));
    }

    #[test]
    fn test_risk_reward_ratio() {
        let ratio = RiskCalculator::calculate_risk_reward_ratio(
            Decimal::from(100),
            Decimal::from(98),
            Decimal::from(106),
        );
        assert_eq!(ratio, Some(Decimal::from(3)));
    }

    #[test]
    fn test_kelly_criterion() {
        let kelly = RiskCalculator::calculate_kelly_criterion(
            Decimal::from_str_exact("0.6").unwrap(),
            Decimal::from(200),
            Decimal::from(100),
        );
        assert!(kelly > Decimal::ZERO);
    }

    #[test]
    fn test_portfolio_heat() {
        let heat = RiskCalculator::calculate_portfolio_heat(
            vec![Decimal::from(100), Decimal::from(150), Decimal::from(200)],
            Decimal::from(10000),
        );
        assert_eq!(heat, Decimal::from_str_exact("4.5").unwrap());
    }
}
