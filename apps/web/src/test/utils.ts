import { render, type RenderResult } from '@testing-library/svelte';
import type { ComponentProps, Component } from 'svelte';

/**
 * Enhanced render function with common test utilities
 */
export function renderComponent<T extends Component>(
	component: T,
	props?: ComponentProps<T>
): RenderResult<T> {
	return render(component, { props: props as any });
}

/**
 * Mock API client for testing
 */
export const mockApiClient = {
	get: vi.fn(),
	post: vi.fn(),
	put: vi.fn(),
	delete: vi.fn(),
	setTokens: vi.fn(),
	clearTokens: vi.fn()
};

/**
 * Mock auth API for testing
 */
export const mockAuthApi = {
	login: vi.fn(),
	register: vi.fn(),
	logout: vi.fn(),
	getCurrentUser: vi.fn(),
	refreshToken: vi.fn()
};

/**
 * Mock trades API for testing
 */
export const mockTradesApi = {
	list: vi.fn(),
	get: vi.fn(),
	create: vi.fn(),
	update: vi.fn(),
	delete: vi.fn(),
	getStats: vi.fn()
};

/**
 * Wait for async operations to complete
 */
export const waitFor = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * Create mock user data
 */
export function createMockUser(overrides = {}) {
	return {
		id: 'test-user-id',
		email: 'test@example.com',
		...overrides
	};
}

/**
 * Create mock trade data
 */
export function createMockTrade(overrides = {}) {
	return {
		id: 'test-trade-id',
		user_id: 'test-user-id',
		symbol: 'AAPL',
		direction: 'long',
		asset_class: 'stock',
		status: 'open',
		entry_price: 150.0,
		entry_date: new Date().toISOString(),
		quantity: 100,
		...overrides
	};
}

/**
 * Create mock trade stats
 */
export function createMockTradeStats(overrides = {}) {
	return {
		total_trades: 100,
		winning_trades: 60,
		losing_trades: 40,
		win_rate: 60.0,
		total_pnl: 5000.0,
		avg_win: 150.0,
		avg_loss: -75.0,
		profit_factor: 2.0,
		...overrides
	};
}

