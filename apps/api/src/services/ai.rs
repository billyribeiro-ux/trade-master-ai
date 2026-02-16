use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{ClaudeContent, ClaudeMessage, ClaudeRequest, ClaudeResponse, Trade};
use reqwest::Client;

pub struct AiService {
    api_key: Option<String>,
    client: Client,
}

impl AiService {
    pub fn new(config: &Config) -> Self {
        Self {
            api_key: config.anthropic_api_key.clone(),
            client: Client::new(),
        }
    }

    pub async fn analyze_trade(&self, trade: &Trade) -> AppResult<String> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| AppError::Internal("Anthropic API key not configured".to_string()))?;

        let prompt = self.build_trade_analysis_prompt(trade);
        
        let response = self.call_claude(api_key, &prompt).await?;
        
        Ok(response)
    }

    pub async fn chat(&self, messages: Vec<ClaudeMessage>) -> AppResult<(String, i32)> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| AppError::Internal("Anthropic API key not configured".to_string()))?;

        let request = ClaudeRequest {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
            messages,
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Claude API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "Claude API error ({}): {}",
                status, error_text
            )));
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Claude response: {}", e)))?;

        let text = claude_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        let tokens = claude_response.usage.input_tokens + claude_response.usage.output_tokens;

        Ok((text, tokens))
    }

    async fn call_claude(&self, api_key: &str, prompt: &str) -> AppResult<String> {
        let messages = vec![ClaudeMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }];

        let (response, _) = self.chat(messages).await?;
        Ok(response)
    }

    fn build_trade_analysis_prompt(&self, trade: &Trade) -> String {
        format!(
            r#"You are an expert trading coach analyzing a trade. Provide constructive feedback.

Trade Details:
- Symbol: {}
- Direction: {}
- Entry: ${}, Quantity: {}
- Exit: {}
- P&L: {}
- R-Multiple: {}
- Setup: {}
- Thesis: {}
- Mistakes: {}
- Lessons: {}

Please analyze this trade and provide:
1. What went well
2. What could be improved
3. Key lessons
4. Suggestions for future trades
5. Pattern recognition (if applicable)

Be specific, actionable, and encouraging."#,
            trade.symbol,
            format!("{:?}", trade.direction).to_uppercase(),
            trade.entry_price,
            trade.quantity,
            trade.exit_price.map(|p| format!("${}", p)).unwrap_or_else(|| "Still open".to_string()),
            trade.net_pnl.map(|p| format!("${}", p)).unwrap_or_else(|| "N/A".to_string()),
            trade.r_multiple.map(|r| format!("{}R", r)).unwrap_or_else(|| "N/A".to_string()),
            trade.setup_name.as_deref().unwrap_or("Not specified"),
            trade.thesis.as_deref().unwrap_or("Not specified"),
            trade.mistakes.as_deref().unwrap_or("None noted"),
            trade.lessons.as_deref().unwrap_or("None noted")
        )
    }

    pub fn build_general_prompt(&self, context: &str, question: &str) -> String {
        format!(
            r#"You are an expert trading coach and mentor. 

Context: {}

Question: {}

Provide helpful, specific, and actionable advice."#,
            context, question
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TradeDirection;
    use rust_decimal::Decimal;

    #[test]
    fn test_build_trade_analysis_prompt() {
        let config = Config {
            database_url: "".to_string(),
            port: 3000,
            cors_origins: vec![],
            jwt_secret: "test".to_string(),
            jwt_access_expiry_seconds: 900,
            jwt_refresh_expiry_seconds: 2592000,
            anthropic_api_key: Some("test".to_string()),
            s3_endpoint: "".to_string(),
            s3_region: "".to_string(),
            s3_bucket: "".to_string(),
            s3_access_key: "".to_string(),
            s3_secret_key: "".to_string(),
            max_pool_connections: 10,
            smtp_host: None,
            smtp_port: None,
            smtp_username: None,
            smtp_password: None,
            smtp_from_email: None,
        };

        let service = AiService::new(&config);
        
        let trade = Trade {
            id: uuid::Uuid::new_v4(),
            user_id: uuid::Uuid::new_v4(),
            symbol: "AAPL".to_string(),
            direction: TradeDirection::Long,
            asset_class: crate::models::AssetClass::Stocks,
            status: crate::models::TradeStatus::Closed,
            entry_date: chrono::Utc::now(),
            entry_price: Decimal::from(150),
            quantity: Decimal::from(100),
            stop_loss: Some(Decimal::from(148)),
            take_profit: Some(Decimal::from(155)),
            exit_date: Some(chrono::Utc::now()),
            exit_price: Some(Decimal::from(155)),
            actual_exit_price: Some(Decimal::from(155)),
            pnl: Some(Decimal::from(500)),
            pnl_percent: Some(Decimal::from(3)),
            commissions: Some(Decimal::from(2)),
            net_pnl: Some(Decimal::from(498)),
            r_multiple: Some(Decimal::from(2)),
            mae: None,
            mfe: None,
            hold_time_minutes: Some(120),
            risk_amount: Some(Decimal::from(200)),
            risk_percent: Some(Decimal::from(1)),
            position_size_pct: Some(Decimal::from(10)),
            conviction: Some(crate::models::ConvictionLevel::High),
            setup_name: Some("Bull Flag".to_string()),
            timeframe: Some("5m".to_string()),
            thesis: Some("Strong momentum".to_string()),
            mistakes: None,
            lessons: Some("Patience paid off".to_string()),
            emotional_state: Some("Calm".to_string()),
            market_condition: Some("Trending".to_string()),
            execution_grade: Some("A".to_string()),
            patience_grade: Some("A".to_string()),
            discipline_grade: Some("A".to_string()),
            overall_grade: Some("A".to_string()),
            is_paper_trade: false,
            is_revenge_trade: false,
            broke_rules: false,
            followed_plan: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let prompt = service.build_trade_analysis_prompt(&trade);
        
        assert!(prompt.contains("AAPL"));
        assert!(prompt.contains("Bull Flag"));
        assert!(prompt.contains("$150"));
    }
}
