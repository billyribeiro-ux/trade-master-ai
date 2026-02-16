-- Migration 001: Users and Authentication
-- Created: 2026-02-15
-- Description: Core user tables, authentication, and profiles

-- Users table (replaces Supabase auth.users)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

-- Refresh tokens for JWT authentication
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_token_hash ON refresh_tokens(token_hash);
CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);

-- User profiles (onboarding data)
CREATE TABLE user_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Onboarding data
    trading_style VARCHAR(50), -- day_trading, swing_trading, scalping, position_trading
    primary_assets TEXT[], -- stocks, options, forex, futures, crypto
    experience_level VARCHAR(50), -- beginner, intermediate, advanced
    account_size_range VARCHAR(50), -- under_5k, 5k_25k, 25k_50k, etc.
    default_risk_pct DECIMAL(5,2) DEFAULT 1.0,
    timezone VARCHAR(100) DEFAULT 'America/New_York',
    active_sessions TEXT[], -- ny_open, london, asia
    
    -- Goals
    goals TEXT[], -- improve_consistency, scale_to_fulltime, reduce_losses, etc.
    
    -- AI personality
    ai_personality VARCHAR(50) DEFAULT 'balanced', -- strict_coach, encouraging_mentor, balanced
    
    -- Settings
    max_trades_per_day INTEGER,
    max_daily_loss DECIMAL(15,2),
    default_commissions DECIMAL(10,2) DEFAULT 0,
    
    -- Status
    onboarding_completed BOOLEAN DEFAULT FALSE,
    onboarding_completed_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_user_profiles_user_id ON user_profiles(user_id);

-- Update timestamp trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply update triggers
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_profiles_updated_at BEFORE UPDATE ON user_profiles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
