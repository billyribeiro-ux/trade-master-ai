use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{AuthUser, Claims};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub struct AuthService {
    jwt_secret: String,
    access_expiry_seconds: i64,
    refresh_expiry_seconds: i64,
}

impl AuthService {
    pub fn new(config: &Config) -> Self {
        Self {
            jwt_secret: config.jwt_secret.clone(),
            access_expiry_seconds: config.jwt_access_expiry_seconds,
            refresh_expiry_seconds: config.jwt_refresh_expiry_seconds,
        }
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    /// Generate an access token (JWT)
    pub fn generate_access_token(&self, user_id: Uuid, email: &str) -> AppResult<String> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: now + self.access_expiry_seconds,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Generate a refresh token (random string, hashed for storage)
    pub fn generate_refresh_token(&self) -> (String, String) {
        let token = Uuid::new_v4().to_string();
        let hash = self.hash_token(&token);
        (token, hash)
    }

    /// Hash a token for storage
    pub fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify and decode an access token
    pub fn verify_access_token(&self, token: &str) -> AppResult<AuthUser> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        Ok(AuthUser {
            user_id,
            email: token_data.claims.email,
        })
    }

    /// Get refresh token expiry timestamp
    pub fn get_refresh_token_expiry(&self) -> chrono::DateTime<Utc> {
        Utc::now() + chrono::Duration::seconds(self.refresh_expiry_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config {
            database_url: "".to_string(),
            port: 3000,
            cors_origins: vec![],
            jwt_secret: "test_secret_key_at_least_32_characters_long".to_string(),
            jwt_access_expiry_seconds: 900,
            jwt_refresh_expiry_seconds: 2592000,
            anthropic_api_key: None,
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
        }
    }

    #[test]
    fn test_password_hashing() {
        let service = AuthService::new(&test_config());
        let password = "test_password_123";
        
        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_token_generation_and_verification() {
        let service = AuthService::new(&test_config());
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = service.generate_access_token(user_id, email).unwrap();
        let auth_user = service.verify_access_token(&token).unwrap();

        assert_eq!(auth_user.user_id, user_id);
        assert_eq!(auth_user.email, email);
    }

    #[test]
    fn test_refresh_token_hashing() {
        let service = AuthService::new(&test_config());
        let (token, hash) = service.generate_refresh_token();
        
        assert_ne!(token, hash);
        assert_eq!(hash, service.hash_token(&token));
    }
}
