-- Migration 009: Psychology & Discipline
-- Created: 2026-02-15
-- Description: Mood logs, goals, tilt events, alert rules

-- Mood logs
CREATE TABLE mood_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    log_date DATE NOT NULL,
    
    -- Mood metrics (1-10 scale)
    pre_market_mood INTEGER CHECK (pre_market_mood >= 1 AND pre_market_mood <= 10),
    post_market_mood INTEGER CHECK (post_market_mood >= 1 AND post_market_mood <= 10),
    stress_level INTEGER CHECK (stress_level >= 1 AND stress_level <= 10),
    confidence_level INTEGER CHECK (confidence_level >= 1 AND confidence_level <= 10),
    sleep_quality INTEGER CHECK (sleep_quality >= 1 AND sleep_quality <= 10),
    
    -- Emotion tags
    emotions TEXT[], -- focused, anxious, overconfident, calm, frustrated, excited, bored, fearful, disciplined
    
    notes TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, log_date)
);

CREATE INDEX idx_mood_logs_user_id ON mood_logs(user_id);
CREATE INDEX idx_mood_logs_log_date ON mood_logs(log_date);

CREATE TRIGGER update_mood_logs_updated_at BEFORE UPDATE ON mood_logs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trading goals
CREATE TABLE trading_goals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    goal_type VARCHAR(50), -- win_rate, profit_factor, consistency, risk_mgmt, journal, custom
    title VARCHAR(255) NOT NULL,
    description TEXT,
    target_value DECIMAL(15,2),
    current_value DECIMAL(15,2) DEFAULT 0,
    target_date DATE,
    status VARCHAR(20) DEFAULT 'active', -- active, achieved, abandoned
    
    achieved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_trading_goals_user_id ON trading_goals(user_id);
CREATE INDEX idx_trading_goals_status ON trading_goals(status);

CREATE TRIGGER update_trading_goals_updated_at BEFORE UPDATE ON trading_goals
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Tilt events (detected automatically)
CREATE TABLE tilt_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    severity VARCHAR(20) NOT NULL, -- warning, alert, critical
    trigger_type VARCHAR(50) NOT NULL, -- rapid_trades, size_escalation, loss_chasing, daily_limit, watchlist_deviation
    message TEXT NOT NULL,
    suggested_action TEXT,
    
    -- Related trades
    trade_ids UUID[],
    
    -- Historical context
    historical_win_rate DECIMAL(5,2), -- win rate in similar tilt states
    
    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_tilt_events_user_id ON tilt_events(user_id);
CREATE INDEX idx_tilt_events_severity ON tilt_events(severity);
CREATE INDEX idx_tilt_events_created_at ON tilt_events(created_at DESC);

-- Custom alert rules
CREATE TABLE alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    name VARCHAR(255) NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    
    -- Condition
    trigger_type VARCHAR(50) NOT NULL, -- daily_loss, trade_count, consecutive_losses, time_since_last_trade, symbol_not_on_watchlist
    operator VARCHAR(20) NOT NULL, -- exceeds, falls_below, equals, within
    threshold_value DECIMAL(15,2),
    time_window_minutes INTEGER,
    
    -- Action
    action VARCHAR(50) NOT NULL, -- notify, cooldown, lockout, require_journal
    action_duration_minutes INTEGER, -- for cooldown/lockout
    custom_message TEXT,
    
    -- Stats
    times_triggered INTEGER DEFAULT 0,
    last_triggered_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_alert_rules_user_id ON alert_rules(user_id);
CREATE INDEX idx_alert_rules_enabled ON alert_rules(enabled);

CREATE TRIGGER update_alert_rules_updated_at BEFORE UPDATE ON alert_rules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
