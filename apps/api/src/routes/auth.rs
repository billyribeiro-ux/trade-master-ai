use crate::error::{AppError, AppResult};
use crate::models::{
    AuthResponse, AuthUser, LoginRequest, RefreshToken, RefreshTokenRequest, RegisterRequest,
    User, UserInfo, UserProfile,
};
use crate::services::AuthService;
use axum::{extract::State, Json};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn register(
    State(pool): State<Arc<PgPool>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Validate email format
    if !req.email.contains('@') {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }

    // Validate password strength
    if req.password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Hash password
    let password_hash = auth_service.hash_password(&req.password)?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING id, email, password_hash, email_verified, created_at, updated_at
        "#,
    )
    .bind(&req.email)
    .bind(&password_hash)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Email already registered".to_string())
        }
        _ => AppError::from(e),
    })?;

    // Create empty profile
    sqlx::query(
        r#"
        INSERT INTO user_profiles (user_id)
        VALUES ($1)
        "#,
    )
    .bind(user.id)
    .execute(pool.as_ref())
    .await?;

    // Generate tokens
    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (refresh_token, refresh_token_hash) = auth_service.generate_refresh_token();
    let refresh_expiry = auth_service.get_refresh_token_expiry();

    // Store refresh token
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&refresh_token_hash)
    .bind(refresh_expiry)
    .execute(pool.as_ref())
    .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            email_verified: user.email_verified,
            onboarding_completed: false,
        },
    }))
}

pub async fn login(
    State(pool): State<Arc<PgPool>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Find user
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, email_verified, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&req.email)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    // Verify password
    if !auth_service.verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // Get profile to check onboarding status
    let profile = sqlx::query_as::<_, UserProfile>(
        r#"
        SELECT * FROM user_profiles WHERE user_id = $1
        "#,
    )
    .bind(user.id)
    .fetch_optional(pool.as_ref())
    .await?;

    // Generate tokens
    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (refresh_token, refresh_token_hash) = auth_service.generate_refresh_token();
    let refresh_expiry = auth_service.get_refresh_token_expiry();

    // Store refresh token
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&refresh_token_hash)
    .bind(refresh_expiry)
    .execute(pool.as_ref())
    .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            email_verified: user.email_verified,
            onboarding_completed: profile.map(|p| p.onboarding_completed).unwrap_or(false),
        },
    }))
}

pub async fn refresh(
    State(pool): State<Arc<PgPool>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(req): Json<RefreshTokenRequest>,
) -> AppResult<Json<AuthResponse>> {
    let token_hash = auth_service.hash_token(&req.refresh_token);

    // Find and validate refresh token
    let refresh_token = sqlx::query_as::<_, RefreshToken>(
        r#"
        SELECT * FROM refresh_tokens
        WHERE token_hash = $1 AND revoked = false AND expires_at > NOW()
        "#,
    )
    .bind(&token_hash)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

    // Get user
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, email_verified, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(refresh_token.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    // Get profile
    let profile = sqlx::query_as::<_, UserProfile>(
        r#"
        SELECT * FROM user_profiles WHERE user_id = $1
        "#,
    )
    .bind(user.id)
    .fetch_optional(pool.as_ref())
    .await?;

    // Revoke old refresh token
    sqlx::query(
        r#"
        UPDATE refresh_tokens SET revoked = true WHERE id = $1
        "#,
    )
    .bind(refresh_token.id)
    .execute(pool.as_ref())
    .await?;

    // Generate new tokens
    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (new_refresh_token, new_refresh_token_hash) = auth_service.generate_refresh_token();
    let refresh_expiry = auth_service.get_refresh_token_expiry();

    // Store new refresh token
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&new_refresh_token_hash)
    .bind(refresh_expiry)
    .execute(pool.as_ref())
    .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token: new_refresh_token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            email_verified: user.email_verified,
            onboarding_completed: profile.map(|p| p.onboarding_completed).unwrap_or(false),
        },
    }))
}

pub async fn logout(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<serde_json::Value>> {
    // Revoke all refresh tokens for this user
    sqlx::query(
        r#"
        UPDATE refresh_tokens SET revoked = true WHERE user_id = $1
        "#,
    )
    .bind(auth_user.user_id)
    .execute(pool.as_ref())
    .await?;

    Ok(Json(serde_json::json!({ "message": "Logged out successfully" })))
}

pub async fn me(
    State(pool): State<Arc<PgPool>>,
    auth_user: AuthUser,
) -> AppResult<Json<UserInfo>> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, email_verified, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    let profile = sqlx::query_as::<_, UserProfile>(
        r#"
        SELECT * FROM user_profiles WHERE user_id = $1
        "#,
    )
    .bind(user.id)
    .fetch_optional(pool.as_ref())
    .await?;

    Ok(Json(UserInfo {
        id: user.id,
        email: user.email,
        email_verified: user.email_verified,
        onboarding_completed: profile.map(|p| p.onboarding_completed).unwrap_or(false),
    }))
}
