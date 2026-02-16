use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub jwt_secret: String,
    pub jwt_access_expiry_seconds: i64,
    pub jwt_refresh_expiry_seconds: i64,
    pub anthropic_api_key: Option<String>,
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub max_pool_connections: u32,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_from_email: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL must be set")?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .context("PORT must be a valid number")?;

        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let jwt_secret = env::var("JWT_SECRET")
            .context("JWT_SECRET must be set")?;

        let jwt_access_expiry_seconds = env::var("JWT_ACCESS_EXPIRY_SECONDS")
            .unwrap_or_else(|_| "900".to_string()) // 15 minutes
            .parse()
            .context("JWT_ACCESS_EXPIRY_SECONDS must be a valid number")?;

        let jwt_refresh_expiry_seconds = env::var("JWT_REFRESH_EXPIRY_SECONDS")
            .unwrap_or_else(|_| "2592000".to_string()) // 30 days
            .parse()
            .context("JWT_REFRESH_EXPIRY_SECONDS must be a valid number")?;

        let anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok();

        let s3_endpoint = env::var("S3_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:9000".to_string());

        let s3_region = env::var("S3_REGION")
            .unwrap_or_else(|_| "us-east-1".to_string());

        let s3_bucket = env::var("S3_BUCKET")
            .unwrap_or_else(|_| "trademaster-media".to_string());

        let s3_access_key = env::var("S3_ACCESS_KEY")
            .unwrap_or_else(|_| "minioadmin".to_string());

        let s3_secret_key = env::var("S3_SECRET_KEY")
            .unwrap_or_else(|_| "minioadmin".to_string());

        let max_pool_connections = env::var("MAX_POOL_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .context("MAX_POOL_CONNECTIONS must be a valid number")?;

        let smtp_host = env::var("SMTP_HOST").ok();
        let smtp_port = env::var("SMTP_PORT").ok().and_then(|p| p.parse().ok());
        let smtp_username = env::var("SMTP_USERNAME").ok();
        let smtp_password = env::var("SMTP_PASSWORD").ok();
        let smtp_from_email = env::var("SMTP_FROM_EMAIL").ok();

        Ok(Config {
            database_url,
            port,
            cors_origins,
            jwt_secret,
            jwt_access_expiry_seconds,
            jwt_refresh_expiry_seconds,
            anthropic_api_key,
            s3_endpoint,
            s3_region,
            s3_bucket,
            s3_access_key,
            s3_secret_key,
            max_pool_connections,
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_from_email,
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.jwt_secret.len() < 32 {
            anyhow::bail!("JWT_SECRET must be at least 32 characters");
        }

        if self.port == 0 {
            anyhow::bail!("PORT must be greater than 0");
        }

        if self.cors_origins.is_empty() {
            anyhow::bail!("CORS_ORIGINS must contain at least one origin");
        }

        Ok(())
    }
}
