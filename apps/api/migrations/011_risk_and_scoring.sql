-- Migration 011: Risk Management & Edge Scoring
-- Created: 2026-02-15
-- Description: Market snapshots, edge score history

-- Market snapshots (captured at trade entry time)
CREATE TABLE market_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trade_id UUID UNIQUE NOT NULL REFERENCES trades(id) ON DELETE CASCADE,
    
    snapshot_time TIMESTAMPTZ NOT NULL,
    
    -- Market internals
    tick DECIMAL(10,2),
    add DECIMAL(10,2),
    vold DECIMAL(10,2),
    vix DECIMAL(10,4),
    
    -- Major indices
    spy_price DECIMAL(10,2),
    spy_change_pct DECIMAL(10,4),
    qqq_price DECIMAL(10,2),
    qqq_change_pct DECIMAL(10,4),
    
    -- Sector performance (if applicable)
    sector_performance JSONB,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_market_snapshots_trade_id ON market_snapshots(trade_id);
CREATE INDEX idx_market_snapshots_snapshot_time ON market_snapshots(snapshot_time);

-- Edge score history (daily composite score)
CREATE TABLE edge_score_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    score_date DATE NOT NULL,
    
    -- Composite score (0-100)
    composite_score DECIMAL(5,2) NOT NULL,
    
    -- Component scores (0-100 each)
    plan_adherence_score DECIMAL(5,2),
    grade_distribution_score DECIMAL(5,2),
    risk_management_score DECIMAL(5,2),
    journal_quality_score DECIMAL(5,2),
    emotional_stability_score DECIMAL(5,2),
    normalized_pnl_score DECIMAL(5,2),
    
    -- AI explanation
    ai_explanation TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, score_date)
);

CREATE INDEX idx_edge_score_history_user_id ON edge_score_history(user_id);
CREATE INDEX idx_edge_score_history_score_date ON edge_score_history(score_date DESC);
CREATE INDEX idx_edge_score_history_user_date ON edge_score_history(user_id, score_date DESC);
