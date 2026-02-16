-- Migration 008: AI Reviews
-- Created: 2026-02-15
-- Description: AI-powered trade reviews and chat conversations

CREATE TABLE ai_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    trade_id UUID REFERENCES trades(id) ON DELETE CASCADE, -- NULL for weekly/monthly reviews
    
    review_type VARCHAR(50) DEFAULT 'trade', -- trade, weekly, monthly
    
    -- Review scores
    overall_score DECIMAL(4,2), -- 0-10
    execution_quality_score DECIMAL(4,2),
    risk_management_score DECIMAL(4,2),
    plan_adherence_score DECIMAL(4,2),
    thesis_alignment_score DECIMAL(4,2), -- 1-5 or NULL
    
    -- Review content
    strengths TEXT[],
    weaknesses TEXT[],
    key_lesson TEXT,
    actionable_fixes TEXT[],
    alternative_scenario TEXT,
    chart_analysis TEXT,
    emotional_state_detected VARCHAR(50),
    
    -- Full AI response (for debugging/improvement)
    raw_response TEXT,
    
    -- Metadata
    tokens_used INTEGER,
    cost_usd DECIMAL(10,4),
    prompt_version VARCHAR(50),
    model_used VARCHAR(100) DEFAULT 'claude-sonnet-4-20250514',
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_ai_reviews_user_id ON ai_reviews(user_id);
CREATE INDEX idx_ai_reviews_trade_id ON ai_reviews(trade_id);
CREATE INDEX idx_ai_reviews_review_type ON ai_reviews(review_type);
CREATE INDEX idx_ai_reviews_created_at ON ai_reviews(created_at DESC);

-- AI review chat messages (follow-up conversations)
CREATE TABLE ai_review_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES ai_reviews(id) ON DELETE CASCADE,
    
    role VARCHAR(20) NOT NULL, -- user, assistant
    content TEXT NOT NULL,
    tokens_used INTEGER,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_ai_review_messages_review_id ON ai_review_messages(review_id);
CREATE INDEX idx_ai_review_messages_created_at ON ai_review_messages(created_at);
