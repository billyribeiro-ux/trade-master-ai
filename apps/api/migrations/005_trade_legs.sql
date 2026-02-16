-- Migration 005: Trade Legs (Scaling/Position Management)
-- Created: 2026-02-15
-- Description: Track individual entries and exits for scaled positions

CREATE TABLE trade_legs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trade_id UUID NOT NULL REFERENCES trades(id) ON DELETE CASCADE,
    
    leg_type VARCHAR(20) NOT NULL, -- entry, exit, add, trim
    price DECIMAL(20,8) NOT NULL,
    size DECIMAL(20,8) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    fees DECIMAL(10,2) DEFAULT 0,
    notes TEXT,
    sort_order INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_trade_legs_trade_id ON trade_legs(trade_id);
CREATE INDEX idx_trade_legs_timestamp ON trade_legs(timestamp);
CREATE INDEX idx_trade_legs_trade_sort ON trade_legs(trade_id, sort_order);
