use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{ClaudeMessage, ClaudeRequest, ClaudeResponse, Trade};
use reqwest::Client;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_RETRIES: u32 = 2;
const INITIAL_BACKOFF_MS: u64 = 500;
const CIRCUIT_BREAKER_THRESHOLD: u64 = 5;
const CIRCUIT_BREAKER_RESET_SECS: u64 = 60;

pub struct AiService {
    api_key: Option<String>,
    client: Client,
    consecutive_failures: AtomicU64,
    last_failure_epoch: AtomicU64,
}

impl AiService {
    pub fn new(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(4)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            api_key: config.anthropic_api_key.clone(),
            client,
            consecutive_failures: AtomicU64::new(0),
            last_failure_epoch: AtomicU64::new(0),
        }
    }

    /// Checks whether the circuit breaker is open (too many recent failures).
    fn is_circuit_open(&self) -> bool {
        let failures = self.consecutive_failures.load(Ordering::Relaxed);
        if failures < CIRCUIT_BREAKER_THRESHOLD {
            return false;
        }

        let last_fail = self.last_failure_epoch.load(Ordering::Relaxed);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // If enough time has passed, allow a probe request through
        if now.saturating_sub(last_fail) > CIRCUIT_BREAKER_RESET_SECS {
            return false;
        }

        true
    }

    fn record_success(&self) {
        self.consecutive_failures.store(0, Ordering::Relaxed);
    }

    fn record_failure(&self) {
        self.consecutive_failures.fetch_add(1, Ordering::Relaxed);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.last_failure_epoch.store(now, Ordering::Relaxed);
    }

    pub async fn analyze_trade(&self, trade: &Trade) -> AppResult<String> {
        let prompt = self.build_trade_analysis_prompt(trade);
        let messages = vec![ClaudeMessage {
            role: "user".to_string(),
            content: prompt,
        }];
        let (response, _) = self.chat(messages).await?;
        Ok(response)
    }

    /// Sends messages to the Claude API with retry logic and circuit breaker.
    pub async fn chat(&self, messages: Vec<ClaudeMessage>) -> AppResult<(String, i32)> {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            AppError::AiError("AI service is not configured. Set ANTHROPIC_API_KEY.".to_string())
        })?;

        if self.is_circuit_open() {
            return Err(AppError::AiError(
                "AI service is temporarily unavailable. Please try again later.".to_string(),
            ));
        }

        let request_body = ClaudeRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            messages,
        };

        let mut last_error: Option<AppError> = None;

        for attempt in 0..=MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(INITIAL_BACKOFF_MS * 2u64.pow(attempt - 1));
                tokio::time::sleep(backoff).await;
                tracing::info!(attempt, "Retrying Claude API request");
            }

            match self.send_request(api_key, &request_body).await {
                Ok(result) => {
                    self.record_success();
                    return Ok(result);
                }
                Err(e) => {
                    tracing::warn!(attempt, error = %e, "Claude API request failed");

                    // Don't retry on client errors (4xx) except 429 (rate limit)
                    if matches!(&e, AppError::Validation(_)) {
                        return Err(e);
                    }

                    last_error = Some(e);
                }
            }
        }

        self.record_failure();
        Err(last_error.unwrap_or_else(|| {
            AppError::AiError("AI request failed after retries".to_string())
        }))
    }

    /// Performs a single HTTP request to the Claude API.
    async fn send_request(
        &self,
        api_key: &str,
        request_body: &ClaudeRequest,
    ) -> AppResult<(String, i32)> {
        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(request_body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AppError::AiError("AI request timed out. Please try again.".to_string())
                } else if e.is_connect() {
                    AppError::AiError("Could not reach AI service.".to_string())
                } else {
                    AppError::AiError(format!("AI request failed: {}", e))
                }
            })?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();

            // Log the full error server-side, return a safe message to the client
            tracing::error!(status = %status, body = %error_text, "Claude API error");

            return match status.as_u16() {
                401 => Err(AppError::AiError(
                    "AI service authentication failed. Check API key.".to_string(),
                )),
                429 => Err(AppError::AiError(
                    "AI rate limit reached. Please wait and try again.".to_string(),
                )),
                400 => Err(AppError::Validation(
                    "Invalid request to AI service.".to_string(),
                )),
                _ => Err(AppError::AiError(
                    "AI service returned an error. Please try again.".to_string(),
                )),
            };
        }

        let claude_response: ClaudeResponse = response.json().await.map_err(|e| {
            tracing::error!(error = %e, "Failed to parse Claude response");
            AppError::AiError("Unexpected response from AI service.".to_string())
        })?;

        let text = claude_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        if text.is_empty() {
            return Err(AppError::AiError(
                "AI returned an empty response.".to_string(),
            ));
        }

        let tokens = claude_response.usage.input_tokens + claude_response.usage.output_tokens;

        Ok((text, tokens))
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
