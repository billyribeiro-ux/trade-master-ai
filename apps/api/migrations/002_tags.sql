-- Migration 002: Tags
-- Created: 2026-02-15
-- Description: User-defined tags for categorizing trades

CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    category VARCHAR(50) NOT NULL, -- strategy, mistake, emotion, session, market_condition, custom
    color VARCHAR(7) DEFAULT '#6366f1', -- hex color
    icon VARCHAR(50), -- icon name (Phosphor icons)
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, name)
);

CREATE INDEX idx_tags_user_id ON tags(user_id);
CREATE INDEX idx_tags_category ON tags(category);

CREATE TRIGGER update_tags_updated_at BEFORE UPDATE ON tags
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Seed default tags function
CREATE OR REPLACE FUNCTION seed_default_tags(p_user_id UUID)
RETURNS VOID AS $$
BEGIN
    -- Strategy tags
    INSERT INTO tags (user_id, name, category, color, icon, sort_order) VALUES
    (p_user_id, 'Breakout', 'strategy', '#10b981', 'TrendUp', 1),
    (p_user_id, 'Pullback', 'strategy', '#3b82f6', 'ArrowBendDownLeft', 2),
    (p_user_id, 'Reversal', 'strategy', '#8b5cf6', 'ArrowsCounterClockwise', 3),
    (p_user_id, 'Range Bound', 'strategy', '#f59e0b', 'ArrowsHorizontal', 4),
    (p_user_id, 'Momentum', 'strategy', '#ef4444', 'Lightning', 5);
    
    -- Mistake tags
    INSERT INTO tags (user_id, name, category, color, icon, sort_order) VALUES
    (p_user_id, 'FOMO', 'mistake', '#ef4444', 'Warning', 10),
    (p_user_id, 'Overtrading', 'mistake', '#f97316', 'WarningCircle', 11),
    (p_user_id, 'Revenge Trade', 'mistake', '#dc2626', 'Fire', 12),
    (p_user_id, 'Ignored Stop', 'mistake', '#991b1b', 'XCircle', 13),
    (p_user_id, 'Poor Entry', 'mistake', '#ea580c', 'ArrowDown', 14);
    
    -- Emotion tags
    INSERT INTO tags (user_id, name, category, color, icon, sort_order) VALUES
    (p_user_id, 'Confident', 'emotion', '#10b981', 'Smiley', 20),
    (p_user_id, 'Anxious', 'emotion', '#f59e0b', 'SmileyNervous', 21),
    (p_user_id, 'Frustrated', 'emotion', '#ef4444', 'SmileyAngry', 22),
    (p_user_id, 'Calm', 'emotion', '#06b6d4', 'SmileySad', 23),
    (p_user_id, 'Overconfident', 'emotion', '#f97316', 'SmileyWink', 24);
    
    -- Session tags
    INSERT INTO tags (user_id, name, category, color, icon, sort_order) VALUES
    (p_user_id, 'Pre-Market', 'session', '#6366f1', 'SunHorizon', 30),
    (p_user_id, 'Market Open', 'session', '#8b5cf6', 'Sun', 31),
    (p_user_id, 'Mid-Day', 'session', '#06b6d4', 'SunDim', 32),
    (p_user_id, 'Power Hour', 'session', '#f59e0b', 'Lightning', 33),
    (p_user_id, 'After Hours', 'session', '#64748b', 'Moon', 34);
    
    -- Market condition tags
    INSERT INTO tags (user_id, name, category, color, icon, sort_order) VALUES
    (p_user_id, 'Trending Up', 'market_condition', '#10b981', 'TrendUp', 40),
    (p_user_id, 'Trending Down', 'market_condition', '#ef4444', 'TrendDown', 41),
    (p_user_id, 'Choppy', 'market_condition', '#f59e0b', 'WaveSine', 42),
    (p_user_id, 'High Volume', 'market_condition', '#3b82f6', 'ChartBar', 43),
    (p_user_id, 'Low Volume', 'market_condition', '#64748b', 'ChartBarHorizontal', 44);
END;
$$ LANGUAGE plpgsql;
