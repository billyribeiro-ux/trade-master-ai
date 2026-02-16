-- Migration 012: Social, System, and Supporting Tables
-- Created: 2026-02-15
-- Description: Streaks, accountability, broker connections, analytics cache, economic events

-- User streaks (gamification)
CREATE TABLE user_streaks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    streak_type VARCHAR(50) NOT NULL, -- journal, planning, plan_adherence, review
    current_count INTEGER DEFAULT 0,
    best_count INTEGER DEFAULT 0,
    decay_value DECIMAL(10,2) DEFAULT 0, -- quality-weighted value
    last_activity_date DATE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, streak_type)
);

CREATE INDEX idx_user_streaks_user_id ON user_streaks(user_id);
CREATE INDEX idx_user_streaks_streak_type ON user_streaks(streak_type);

CREATE TRIGGER update_user_streaks_updated_at BEFORE UPDATE ON user_streaks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Accountability links (coach/mentor access)
CREATE TABLE accountability_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trader_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    coach_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Permissions
    can_view_plan BOOLEAN DEFAULT TRUE,
    can_view_grades BOOLEAN DEFAULT TRUE,
    can_view_tilt BOOLEAN DEFAULT TRUE,
    can_view_pnl BOOLEAN DEFAULT FALSE,
    can_view_mood BOOLEAN DEFAULT TRUE,
    can_view_reviews BOOLEAN DEFAULT TRUE,
    
    status VARCHAR(20) DEFAULT 'pending', -- pending, active, revoked
    
    invited_at TIMESTAMPTZ DEFAULT NOW(),
    accepted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    
    UNIQUE(trader_user_id, coach_user_id)
);

CREATE INDEX idx_accountability_links_trader ON accountability_links(trader_user_id);
CREATE INDEX idx_accountability_links_coach ON accountability_links(coach_user_id);
CREATE INDEX idx_accountability_links_status ON accountability_links(status);

-- Broker connections
CREATE TABLE broker_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    broker_name VARCHAR(100) NOT NULL, -- thinkorswim, interactive_brokers, tradingview, etc.
    
    -- Encrypted credentials (stored with encryption at rest)
    credentials_encrypted TEXT,
    
    -- Connection status
    is_active BOOLEAN DEFAULT TRUE,
    last_sync_at TIMESTAMPTZ,
    last_sync_status VARCHAR(50), -- success, error, pending
    last_sync_error TEXT,
    
    -- Sync settings
    auto_sync_enabled BOOLEAN DEFAULT FALSE,
    sync_interval_minutes INTEGER DEFAULT 60,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_broker_connections_user_id ON broker_connections(user_id);
CREATE INDEX idx_broker_connections_is_active ON broker_connections(is_active);

CREATE TRIGGER update_broker_connections_updated_at BEFORE UPDATE ON broker_connections
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Analytics cache (for expensive computations)
CREATE TABLE analytics_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    cache_key VARCHAR(255) NOT NULL, -- e.g., "equity_curve_2024-01-01_2024-12-31"
    cache_data JSONB NOT NULL,
    
    expires_at TIMESTAMPTZ NOT NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, cache_key)
);

CREATE INDEX idx_analytics_cache_user_id ON analytics_cache(user_id);
CREATE INDEX idx_analytics_cache_expires_at ON analytics_cache(expires_at);
CREATE INDEX idx_analytics_cache_user_key ON analytics_cache(user_id, cache_key);

-- Economic events calendar
CREATE TABLE economic_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    event_date DATE NOT NULL,
    event_time TIME,
    title VARCHAR(255) NOT NULL,
    currency VARCHAR(10), -- USD, EUR, GBP, etc.
    impact VARCHAR(20), -- high, medium, low
    description TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_economic_events_event_date ON economic_events(event_date);
CREATE INDEX idx_economic_events_impact ON economic_events(impact);
CREATE INDEX idx_economic_events_currency ON economic_events(currency);

-- Weekly review records
CREATE TABLE weekly_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    week_start_date DATE NOT NULL,
    week_end_date DATE NOT NULL,
    
    -- Review data (captured from the guided review flow)
    review_data JSONB NOT NULL,
    
    -- Reflection notes
    reflection_notes TEXT,
    
    completed_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, week_start_date)
);

CREATE INDEX idx_weekly_reviews_user_id ON weekly_reviews(user_id);
CREATE INDEX idx_weekly_reviews_week_start ON weekly_reviews(week_start_date DESC);
