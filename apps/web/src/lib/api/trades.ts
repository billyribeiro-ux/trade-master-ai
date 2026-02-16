import type {
	CloseTradeRequest,
	CreateTradeLegRequest,
	CreateTradeRequest,
	Trade,
	TradeLeg,
	TradeListQuery,
	TradeListResponse,
	TradeStats,
	TradeWithDetails,
	UpdateTradeRequest
} from '$lib/types/trade';
import { apiClient } from './client';

export const tradesApi = {
	async create(data: CreateTradeRequest): Promise<Trade> {
		return apiClient.post<Trade>('/api/v1/trades', data);
	},

	async list(query?: TradeListQuery): Promise<TradeListResponse> {
		const params = new URLSearchParams();
		
		if (query?.page) params.append('page', query.page.toString());
		if (query?.per_page) params.append('per_page', query.per_page.toString());
		if (query?.sort_by) params.append('sort_by', query.sort_by);
		if (query?.sort_order) params.append('sort_order', query.sort_order);
		
		if (query?.filters) {
			Object.entries(query.filters).forEach(([key, value]) => {
				if (value !== undefined && value !== null) {
					if (Array.isArray(value)) {
						value.forEach(v => params.append(key, v.toString()));
					} else {
						params.append(key, value.toString());
					}
				}
			});
		}

		const queryString = params.toString();
		const endpoint = queryString ? `/api/v1/trades?${queryString}` : '/api/v1/trades';
		
		return apiClient.get<TradeListResponse>(endpoint);
	},

	async get(id: string): Promise<TradeWithDetails> {
		return apiClient.get<TradeWithDetails>(`/api/v1/trades/${id}`);
	},

	async update(id: string, data: UpdateTradeRequest): Promise<Trade> {
		return apiClient.put<Trade>(`/api/v1/trades/${id}`, data);
	},

	async close(id: string, data: CloseTradeRequest): Promise<Trade> {
		return apiClient.post<Trade>(`/api/v1/trades/${id}/close`, data);
	},

	async delete(id: string): Promise<void> {
		return apiClient.delete(`/api/v1/trades/${id}`);
	},

	async addLeg(tradeId: string, data: CreateTradeLegRequest): Promise<TradeLeg> {
		return apiClient.post<TradeLeg>(`/api/v1/trades/${tradeId}/legs`, data);
	},

	async getStats(): Promise<TradeStats> {
		return apiClient.get<TradeStats>('/api/v1/trades/stats');
	}
};
