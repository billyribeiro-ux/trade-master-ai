import { apiClient } from './api/client';

export interface User {
	id: string;
	email: string;
	displayName: string | null;
	createdAt: string;
}

export interface LoginRequest {
	email: string;
	password: string;
}

export interface RegisterRequest {
	email: string;
	password: string;
	displayName: string;
}

export interface TokenResponse {
	accessToken: string;
	user: User;
}

export interface ForgotPasswordRequest {
	email: string;
}

export interface ResetPasswordRequest {
	token: string;
	newPassword: string;
}

export const authService = {
	async login(email: string, password: string): Promise<TokenResponse> {
		const response = await apiClient.post<TokenResponse>('/auth/login', {
			email,
			password,
		});
		return response;
	},

	async register(
		email: string,
		password: string,
		displayName: string,
	): Promise<TokenResponse> {
		const response = await apiClient.post<TokenResponse>('/auth/register', {
			email,
			password,
			displayName,
		});
		return response;
	},

	async logout(): Promise<void> {
		await apiClient.post('/auth/logout', {});
	},

	async refreshToken(): Promise<TokenResponse> {
		const response = await apiClient.post<TokenResponse>('/auth/refresh', {});
		return response;
	},

	async getMe(): Promise<User> {
		const response = await apiClient.get<User>('/auth/me');
		return response;
	},

	async forgotPassword(email: string): Promise<void> {
		await apiClient.post('/auth/forgot-password', { email });
	},

	async resetPassword(token: string, newPassword: string): Promise<void> {
		await apiClient.post('/auth/reset-password', { token, newPassword });
	},

	googleLogin(): void {
		const apiUrl = import.meta.env.PUBLIC_API_URL || 'http://localhost:3001';
		window.location.href = `${apiUrl}/api/v1/auth/google`;
	},

	appleLogin(): void {
		const apiUrl = import.meta.env.PUBLIC_API_URL || 'http://localhost:3001';
		window.location.href = `${apiUrl}/api/v1/auth/apple`;
	},
};
