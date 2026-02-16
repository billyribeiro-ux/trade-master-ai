-- Migration 003: Trades
-- Created: 2026-02-15
-- Description: Core trades table with all trading data

CREATE TABLE trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Basic trade info
    symbol VARCHAR(20) NOT NULL,
    asset_class VARCHAR(20) NOT NULL, -- stock, option, forex, futures, crypto
    direction VARCHAR(10) NOT NULL, -- long, short
    status VARCHAR(20) DEFAULT 'open', -- open, closed, partial
    
    -- Entry data
    entry_price DECIMAL(20,8),
    entry_time TIMESTAMPTZ,
    entry_size DECIMAL(20,8),
    
    -- Exit data
    exit_price DECIMAL(20,8),
    exit_time TIMESTAMPTZ,
    exit_size DECIMAL(20,8),
    
    -- Risk management
    stop_loss DECIMAL(20,8),
    take_profit DECIMAL(20,8),
    planned_risk_pct DECIMAL(5,2),
    actual_risk_pct DECIMAL(5,2),
    
    -- Performance metrics
    pnl DECIMAL(15,2),
    pnl_pct DECIMAL(10,4),
    r_multiple DECIMAL(10,4),
    commissions DECIMAL(10,2) DEFAULT 0,
    slippage DECIMAL(10,2) DEFAULT 0,
    duration_minutes INTEGER,
    
    -- Trade quality
    conviction_score INTEGER CHECK (conviction_score >= 1 AND conviction_score <= 5),
    trade_grade VARCHAR(2), -- A, B, C, D
    setup_name VARCHAR(100),
    
    -- Market context
    market_regime VARCHAR(50), -- trending_up, trending_down, ranging, volatile, choppy, low_volume
    session_type VARCHAR(50), -- premarket, first_30, mid_morning, midday, afternoon, power_hour, after_hours
    sector VARCHAR(50),
    
    -- Market internals at entry (captured automatically if available)
    tick_at_entry DECIMAL(10,2),
    add_at_entry DECIMAL(10,2),
    vold_at_entry DECIMAL(10,2),
    vix_at_entry DECIMAL(10,4),
    
    -- Notes and analysis
    pre_trade_thesis TEXT,
    notes TEXT,
    
    -- Options-specific fields
    strike_price DECIMAL(20,8),
    expiration_date DATE,
    option_type VARCHAR(10), -- call, put
    implied_volatility DECIMAL(10,4),
    delta DECIMAL(10,6),
    gamma DECIMAL(10,6),
    theta DECIMAL(10,6),
    vega DECIMAL(10,6),
    
    -- Flags
    is_scaled BOOLEAN DEFAULT FALSE,
    is_missed BOOLEAN DEFAULT FALSE, -- missed trade (logged but not taken)
    missed_outcome VARCHAR(20), -- winner, loser, unclear (for missed trades)
    held_overnight BOOLEAN DEFAULT FALSE,
    archived BOOLEAN DEFAULT FALSE,
    
    -- Entry source
    entry_source VARCHAR(50) DEFAULT 'manual', -- manual, broker_sync, csv_import
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for common queries
CREATE INDEX idx_trades_user_id ON trades(user_id);
CREATE INDEX idx_trades_symbol ON trades(symbol);
CREATE INDEX idx_trades_entry_time ON trades(entry_time);
CREATE INDEX idx_trades_exit_time ON trades(exit_time);
CREATE INDEX idx_trades_status ON trades(status);
CREATE INDEX idx_trades_asset_class ON trades(asset_class);
CREATE INDEX idx_trades_direction ON trades(direction);
CREATE INDEX idx_trades_setup_name ON trades(setup_name);
CREATE INDEX idx_trades_trade_grade ON trades(trade_grade);
CREATE INDEX idx_trades_archived ON trades(archived);
CREATE INDEX idx_trades_user_entry_time ON trades(user_id, entry_time DESC);
CREATE INDEX idx_trades_user_status ON trades(user_id, status);

-- Composite index for common filter combinations
CREATE INDEX idx_trades_user_filters ON trades(user_id, archived, entry_time DESC) 
    WHERE archived = FALSE;

CREATE TRIGGER update_trades_updated_at BEFORE UPDATE ON trades
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
