// Analytics API Response Types

export interface EquityCurvePoint {
	date: string;
	cumulative_pnl: number;
	trade_count: number;
}

export interface EquityCurveData {
	points: EquityCurvePoint[];
	starting_balance: number;
	current_balance: number;
	total_return_pct: number;
}

export interface WinLossDistribution {
	wins: number[];
	losses: number[];
	avg_win: number;
	avg_loss: number;
	largest_win: number;
	largest_loss: number;
	win_count: number;
	loss_count: number;
}

export interface SetupPerformance {
	setup_name: string;
	trade_count: number;
	win_rate: number;
	total_pnl: number;
	avg_pnl: number;
	avg_r_multiple: number | null;
}

export interface HourlyPerformance {
	hour: number;
	trade_count: number;
	win_rate: number;
	avg_pnl: number;
}

export interface DailyPerformance {
	day_of_week: number;
	day_name: string;
	trade_count: number;
	win_rate: number;
	avg_pnl: number;
}

export interface MonthlyPerformance {
	month: string;
	trade_count: number;
	total_pnl: number;
	win_rate: number;
}

export interface TimeBasedAnalytics {
	hourly: HourlyPerformance[];
	daily: DailyPerformance[];
	monthly: MonthlyPerformance[];
}

export interface DrawdownPoint {
	date: string;
	drawdown_pct: number;
	peak_balance: number;
	current_balance: number;
}

export interface DrawdownData {
	points: DrawdownPoint[];
	max_drawdown_pct: number;
	max_drawdown_date: string | null;
	current_drawdown_pct: number;
	recovery_factor: number | null;
}

// AI Review Types
export interface AiReview {
	id: string;
	user_id: string;
	trade_id: string | null;
	review_type: 'trade_analysis' | 'general' | 'pattern_recognition' | 'psychology';
	prompt: string;
	response: string;
	insights: string[];
	recommendations: string[];
	score: number | null;
	created_at: string;
}

export interface ChatMessage {
	id: string;
	review_id: string;
	role: 'user' | 'assistant';
	content: string;
	created_at: string;
}

export interface AiReviewWithMessages {
	review: AiReview;
	messages: ChatMessage[];
}

// CSV Import Types
export interface CsvImportResult {
	success_count: number;
	error_count: number;
	errors: CsvImportError[];
}

export interface CsvImportError {
	row: number;
	field?: string;
	message: string;
}

// Planning Types
export interface DailyPlan {
	id: string;
	user_id: string;
	plan_date: string;
	market_bias: string | null;
	key_levels: string | null;
	setups_to_watch: string | null;
	max_trades: number | null;
	max_risk_pct: number | null;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreateDailyPlanRequest {
	plan_date: string;
	market_bias?: string;
	key_levels?: string;
	setups_to_watch?: string;
	max_trades?: number;
	max_risk_pct?: number;
	notes?: string;
}

export interface UpdateDailyPlanRequest {
	market_bias?: string;
	key_levels?: string;
	setups_to_watch?: string;
	max_trades?: number;
	max_risk_pct?: number;
	notes?: string;
}

// Psychology Types
export interface MoodLog {
	id: string;
	user_id: string;
	log_date: string;
	pre_market_mood: number | null;
	post_market_mood: number | null;
	stress_level: number | null;
	confidence_level: number | null;
	notes: string | null;
	created_at: string;
}

