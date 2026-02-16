-- Migration 004: Trade Tags Junction
-- Created: 2026-02-15
-- Description: Many-to-many relationship between trades and tags

CREATE TABLE trade_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trade_id UUID NOT NULL REFERENCES trades(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(trade_id, tag_id)
);

CREATE INDEX idx_trade_tags_trade_id ON trade_tags(trade_id);
CREATE INDEX idx_trade_tags_tag_id ON trade_tags(tag_id);
