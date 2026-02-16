export type TradeDirection = 'long' | 'short';
export type TradeStatus = 'open' | 'closed' | 'cancelled';
export type AssetClass = 'stocks' | 'options' | 'futures' | 'forex' | 'crypto';
export type ConvictionLevel = 'low' | 'medium' | 'high';

export interface Trade {
	id: string;
	user_id: string;
	symbol: string;
	direction: TradeDirection;
	asset_class: AssetClass;
	status: TradeStatus;
	entry_date: string;
	entry_price: number;
	quantity: number;
	stop_loss?: number;
	take_profit?: number;
	exit_date?: string;
	exit_price?: number;
	actual_exit_price?: number;
	pnl?: number;
	pnl_percent?: number;
	commissions?: number;
	net_pnl?: number;
	r_multiple?: number;
	mae?: number;
	mfe?: number;
	hold_time_minutes?: number;
	risk_amount?: number;
	risk_percent?: number;
	position_size_pct?: number;
	conviction?: ConvictionLevel;
	setup_name?: string;
	timeframe?: string;
	thesis?: string;
	mistakes?: string;
	lessons?: string;
	emotional_state?: string;
	market_condition?: string;
	execution_grade?: string;
	patience_grade?: string;
	discipline_grade?: string;
	overall_grade?: string;
	is_paper_trade: boolean;
	is_revenge_trade: boolean;
	broke_rules: boolean;
	followed_plan: boolean;
	created_at: string;
	updated_at: string;
}

export interface CreateTradeRequest {
	symbol: string;
	direction: TradeDirection;
	asset_class: AssetClass;
	entry_date: string;
	entry_price: number;
	quantity: number;
	stop_loss?: number;
	take_profit?: number;
	risk_amount?: number;
	risk_percent?: number;
	position_size_pct?: number;
	conviction?: ConvictionLevel;
	setup_name?: string;
	timeframe?: string;
	thesis?: string;
	emotional_state?: string;
	market_condition?: string;
	is_paper_trade?: boolean;
	commissions?: number;
}

export interface UpdateTradeRequest {
	symbol?: string;
	direction?: TradeDirection;
	asset_class?: AssetClass;
	entry_date?: string;
	entry_price?: number;
	quantity?: number;
	stop_loss?: number;
	take_profit?: number;
	exit_date?: string;
	exit_price?: number;
	actual_exit_price?: number;
	risk_amount?: number;
	risk_percent?: number;
	position_size_pct?: number;
	conviction?: ConvictionLevel;
	setup_name?: string;
	timeframe?: string;
	thesis?: string;
	mistakes?: string;
	lessons?: string;
	emotional_state?: string;
	market_condition?: string;
	execution_grade?: string;
	patience_grade?: string;
	discipline_grade?: string;
	overall_grade?: string;
	is_paper_trade?: boolean;
	is_revenge_trade?: boolean;
	broke_rules?: boolean;
	followed_plan?: boolean;
	commissions?: number;
}

export interface CloseTradeRequest {
	exit_date: string;
	exit_price: number;
	actual_exit_price?: number;
	mistakes?: string;
	lessons?: string;
	execution_grade?: string;
	patience_grade?: string;
	discipline_grade?: string;
	overall_grade?: string;
	broke_rules?: boolean;
	followed_plan?: boolean;
}

export interface TradeLeg {
	id: string;
	trade_id: string;
	leg_number: number;
	action: string;
	quantity: number;
	price: number;
	timestamp: string;
	notes?: string;
	created_at: string;
}

export interface CreateTradeLegRequest {
	action: string;
	quantity: number;
	price: number;
	timestamp: string;
	notes?: string;
}

export interface TradeMedia {
	id: string;
	trade_id: string;
	media_type: string;
	s3_key: string;
	s3_url: string;
	file_size?: number;
	mime_type?: string;
	caption?: string;
	annotations?: Record<string, unknown>;
	created_at: string;
}

export interface TradeTag {
	id: string;
	name: string;
	color: string;
	category?: string;
}

export interface TradeWithDetails {
	trade: Trade;
	tags: TradeTag[];
	legs: TradeLeg[];
	media: TradeMedia[];
}

export interface TradeFilters {
	status?: TradeStatus;
	direction?: TradeDirection;
	asset_class?: AssetClass;
	symbol?: string;
	setup_name?: string;
	conviction?: ConvictionLevel;
	is_paper_trade?: boolean;
	is_revenge_trade?: boolean;
	broke_rules?: boolean;
	followed_plan?: boolean;
	tag_ids?: string[];
	from_date?: string;
	to_date?: string;
	min_pnl?: number;
	max_pnl?: number;
	min_r_multiple?: number;
	max_r_multiple?: number;
}

export interface TradeListQuery {
	page?: number;
	per_page?: number;
	sort_by?: string;
	sort_order?: 'asc' | 'desc';
	filters?: TradeFilters;
}

export interface TradeListResponse {
	trades: Trade[];
	total: number;
	page: number;
	per_page: number;
	total_pages: number;
}

export interface TradeStats {
	total_trades: number;
	winning_trades: number;
	losing_trades: number;
	win_rate: number;
	total_pnl: number;
	avg_win: number;
	avg_loss: number;
	profit_factor?: number;
	avg_r_multiple?: number;
	largest_win: number;
	largest_loss: number;
	avg_hold_time_minutes?: number;
}
