import type { CreateTagRequest, Tag, TagWithCount, UpdateTagRequest } from '$lib/types/tag';
import { apiClient } from './client';

export const tagsApi = {
	async create(data: CreateTagRequest): Promise<Tag> {
		return apiClient.post<Tag>('/api/v1/tags', data);
	},

	async list(): Promise<TagWithCount[]> {
		return apiClient.get<TagWithCount[]>('/api/v1/tags');
	},

	async get(id: string): Promise<Tag> {
		return apiClient.get<Tag>(`/api/v1/tags/${id}`);
	},

	async update(id: string, data: UpdateTagRequest): Promise<Tag> {
		return apiClient.put<Tag>(`/api/v1/tags/${id}`, data);
	},

	async delete(id: string): Promise<void> {
		return apiClient.delete(`/api/v1/tags/${id}`);
	},

	async addToTrade(tradeId: string, tagId: string): Promise<void> {
		return apiClient.post(`/api/v1/trades/${tradeId}/tags/${tagId}`, {});
	},

	async removeFromTrade(tradeId: string, tagId: string): Promise<void> {
		return apiClient.delete(`/api/v1/trades/${tradeId}/tags/${tagId}`);
	}
};
