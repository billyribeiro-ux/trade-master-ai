-- Migration 007: Daily Plans
-- Created: 2026-02-15
-- Description: Daily trading plans and watchlists

CREATE TABLE daily_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plan_date DATE NOT NULL,
    
    -- Market bias
    market_bias VARCHAR(20), -- bullish, bearish, neutral
    bias_reasoning TEXT,
    
    -- Session goals
    session_goals TEXT[],
    max_trades INTEGER,
    max_daily_loss DECIMAL(15,2),
    
    -- Pre-trade checklist
    checklist_items JSONB, -- [{text, required, checked}]
    
    -- Notes
    notes TEXT,
    
    -- AI-generated plan of attack
    ai_plan_of_attack TEXT,
    
    -- Plan adherence (calculated at end of day)
    adherence_score DECIMAL(5,2), -- 0-100
    adherence_details JSONB,
    
    -- Status
    completed BOOLEAN DEFAULT FALSE,
    completed_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, plan_date)
);

CREATE INDEX idx_daily_plans_user_id ON daily_plans(user_id);
CREATE INDEX idx_daily_plans_plan_date ON daily_plans(plan_date);
CREATE INDEX idx_daily_plans_user_date ON daily_plans(user_id, plan_date DESC);

CREATE TRIGGER update_daily_plans_updated_at BEFORE UPDATE ON daily_plans
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Watchlist items
CREATE TABLE watchlist_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id UUID NOT NULL REFERENCES daily_plans(id) ON DELETE CASCADE,
    
    symbol VARCHAR(20) NOT NULL,
    key_levels JSONB, -- [{price, type, notes}]
    catalysts TEXT,
    setup_description TEXT,
    risk_reward_ratio DECIMAL(10,2),
    position_size_suggested DECIMAL(20,8),
    
    -- Post-market outcome
    was_traded BOOLEAN DEFAULT FALSE,
    outcome VARCHAR(20), -- winner, loser, missed, no_setup
    
    sort_order INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_watchlist_items_plan_id ON watchlist_items(plan_id);
CREATE INDEX idx_watchlist_items_symbol ON watchlist_items(symbol);
CREATE INDEX idx_watchlist_items_plan_sort ON watchlist_items(plan_id, sort_order);

CREATE TRIGGER update_watchlist_items_updated_at BEFORE UPDATE ON watchlist_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
