import type { AuthResponse, LoginRequest, RegisterRequest, User } from '$lib/types/auth';
import { apiClient } from './client';

export const authApi = {
	async register(data: RegisterRequest): Promise<AuthResponse> {
		const response = await apiClient.post<AuthResponse>('/api/v1/auth/register', data, {
			requiresAuth: false
		});
		apiClient.setTokens(response.access_token, response.refresh_token);
		return response;
	},

	async login(data: LoginRequest): Promise<AuthResponse> {
		const response = await apiClient.post<AuthResponse>('/api/v1/auth/login', data, {
			requiresAuth: false
		});
		apiClient.setTokens(response.access_token, response.refresh_token);
		return response;
	},

	async logout(): Promise<void> {
		try {
			await apiClient.post('/api/v1/auth/logout', {});
		} finally {
			apiClient.clearTokens();
		}
	},

	async getCurrentUser(): Promise<User> {
		return apiClient.get<User>('/api/v1/auth/me');
	},

	async refreshToken(refreshToken: string): Promise<AuthResponse> {
		const response = await apiClient.post<AuthResponse>(
			'/api/v1/auth/refresh',
			{ refresh_token: refreshToken },
			{ requiresAuth: false }
		);
		apiClient.setTokens(response.access_token, response.refresh_token);
		return response;
	}
};
