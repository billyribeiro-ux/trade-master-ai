-- Migration 013: Periodic Reviews
-- Created: 2026-02-15
-- Description: Weekly/monthly/quarterly trading performance reviews

CREATE TABLE periodic_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    review_type VARCHAR(20) NOT NULL, -- weekly, monthly, quarterly
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Auto-computed trade statistics
    total_trades INTEGER,
    winning_trades INTEGER,
    losing_trades INTEGER,
    win_rate DECIMAL(5,2),
    total_pnl DECIMAL(15,2),
    avg_r_multiple DECIMAL(10,2),

    -- Self-assessment
    what_went_well TEXT,
    what_to_improve TEXT,
    key_lessons TEXT[],
    rules_broken TEXT[],
    best_trade_id UUID REFERENCES trades(id) ON DELETE SET NULL,
    worst_trade_id UUID REFERENCES trades(id) ON DELETE SET NULL,

    -- Goals
    goals_met TEXT[],
    goals_missed TEXT[],
    goals_next_period TEXT[],

    -- Ratings (1-10)
    discipline_rating INTEGER CHECK (discipline_rating >= 1 AND discipline_rating <= 10),
    patience_rating INTEGER CHECK (patience_rating >= 1 AND patience_rating <= 10),
    execution_rating INTEGER CHECK (execution_rating >= 1 AND execution_rating <= 10),
    overall_rating INTEGER CHECK (overall_rating >= 1 AND overall_rating <= 10),

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, review_type, period_start)
);

CREATE INDEX idx_periodic_reviews_user_id ON periodic_reviews(user_id);
CREATE INDEX idx_periodic_reviews_type ON periodic_reviews(review_type);
CREATE INDEX idx_periodic_reviews_period ON periodic_reviews(period_end DESC);
CREATE INDEX idx_periodic_reviews_user_type ON periodic_reviews(user_id, review_type, period_end DESC);

CREATE TRIGGER update_periodic_reviews_updated_at BEFORE UPDATE ON periodic_reviews
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
