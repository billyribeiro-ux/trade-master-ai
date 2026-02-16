-- Migration 006: Trade Media (Screenshots, Recordings, Order Confirmations)
-- Created: 2026-02-15
-- Description: Media files attached to trades

CREATE TABLE trade_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trade_id UUID NOT NULL REFERENCES trades(id) ON DELETE CASCADE,
    
    media_type VARCHAR(50) NOT NULL, -- screenshot, recording, order_confirmation
    storage_url TEXT NOT NULL, -- S3/R2 URL
    thumbnail_url TEXT, -- Thumbnail for images
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT, -- bytes
    mime_type VARCHAR(100),
    
    -- AI analysis results (if screenshot analyzed)
    ai_analysis JSONB,
    
    -- User annotations
    annotations JSONB, -- drawing data, notes, highlights
    
    captured_at TIMESTAMPTZ,
    sort_order INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_trade_media_trade_id ON trade_media(trade_id);
CREATE INDEX idx_trade_media_type ON trade_media(media_type);
CREATE INDEX idx_trade_media_trade_sort ON trade_media(trade_id, sort_order);
