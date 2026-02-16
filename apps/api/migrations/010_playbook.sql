-- Migration 010: Playbook & Grading
-- Created: 2026-02-15
-- Description: Setup playbook and grading rubrics

-- Playbook setups
CREATE TABLE playbook_setups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Criteria (stored as JSONB array)
    criteria JSONB, -- [{name, weight, required, description}]
    
    -- Expected metrics
    expected_r_min DECIMAL(10,2),
    expected_r_max DECIMAL(10,2),
    min_conviction INTEGER CHECK (min_conviction >= 1 AND min_conviction <= 5),
    
    -- Preferences
    preferred_timeframe VARCHAR(50), -- 1min, 5min, 15min, 1H, 4H, Daily, Weekly
    market_regimes TEXT[], -- trending_up, trending_down, ranging, volatile
    
    -- Example media
    example_screenshots TEXT[], -- S3 URLs
    
    -- Common mistakes
    common_mistakes TEXT,
    
    -- Performance stats (cached, updated periodically)
    trade_count INTEGER DEFAULT 0,
    win_rate DECIMAL(5,2),
    avg_r DECIMAL(10,2),
    profit_factor DECIMAL(10,2),
    total_pnl DECIMAL(15,2),
    
    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, name)
);

CREATE INDEX idx_playbook_setups_user_id ON playbook_setups(user_id);
CREATE INDEX idx_playbook_setups_is_active ON playbook_setups(is_active);

CREATE TRIGGER update_playbook_setups_updated_at BEFORE UPDATE ON playbook_setups
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Grading rubrics
CREATE TABLE grading_rubrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    name VARCHAR(255) NOT NULL,
    
    -- Criteria (stored as JSONB array)
    criteria JSONB, -- [{name, weight_pct, scale_min, scale_max, description}]
    
    -- Grade thresholds
    threshold_a DECIMAL(5,2) DEFAULT 85,
    threshold_b DECIMAL(5,2) DEFAULT 70,
    threshold_c DECIMAL(5,2) DEFAULT 55,
    -- D is anything below C
    
    is_default BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, name)
);

CREATE INDEX idx_grading_rubrics_user_id ON grading_rubrics(user_id);
CREATE INDEX idx_grading_rubrics_is_default ON grading_rubrics(is_default);

CREATE TRIGGER update_grading_rubrics_updated_at BEFORE UPDATE ON grading_rubrics
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Shared rulesets (community sharing)
CREATE TABLE shared_rulesets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creator_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    ruleset_type VARCHAR(50) NOT NULL, -- checklist, rubric, playbook_setup
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Ruleset data (JSONB - structure depends on type)
    ruleset_data JSONB NOT NULL,
    
    -- Visibility
    is_public BOOLEAN DEFAULT FALSE,
    share_token VARCHAR(100) UNIQUE, -- for private sharing
    
    -- Stats
    import_count INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_shared_rulesets_creator ON shared_rulesets(creator_user_id);
CREATE INDEX idx_shared_rulesets_is_public ON shared_rulesets(is_public);
CREATE INDEX idx_shared_rulesets_type ON shared_rulesets(ruleset_type);
CREATE INDEX idx_shared_rulesets_share_token ON shared_rulesets(share_token);

CREATE TRIGGER update_shared_rulesets_updated_at BEFORE UPDATE ON shared_rulesets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
